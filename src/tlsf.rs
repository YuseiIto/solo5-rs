use core::alloc::{GlobalAlloc, Layout};

//TODO: Configure it from given heap size automatically
const FLI_MAX: usize = 31;
const SLI: usize = 4;
const SLI_POW2: usize = 1 << SLI; // 2.pow(SLI)
type FirstLevelBitmapType = u32; // Ceil(FLI/8) bytes(or FLI bits)
type SecondLevelBitmapType = u16; // Ceil(SLI_POW2/8) bytes
const SECOND_LEVEL_BITMAP_LENGTH: usize = FLI_MAX;

use core::cell::Cell;
use core::ptr;

#[cfg(feature = "tlsf_dump")]
use crate::console::put_num;

mod block;
use crate::console::puts;
use block::Block;
use core::cmp::max;
pub struct TLSF {
    initialized: Cell<bool>,
    heap_start: Cell<*mut Block>,
    heap_size: Cell<usize>,
    first_level_bitmap: Cell<FirstLevelBitmapType>,
    second_level_bitmap: [Cell<SecondLevelBitmapType>; SECOND_LEVEL_BITMAP_LENGTH],
    heads: [[Cell<*mut Block>; SLI_POW2]; FLI_MAX],
}

impl TLSF {
    /// Computes size mapping to block.
    /// Return value is `(f,s)` where f is the first level index and s is the second level.
    fn mapping(size: usize) -> (usize, usize) {
        let leading_zeros = size.leading_zeros();
        let f = (usize::BITS - leading_zeros - 1) as usize;
        let s = (size << leading_zeros + 1) >> (usize::BITS - SLI as u32);

        (f, s)
    }

    /// Initialize TLSF structure with specified size and position
    pub fn init(&self, heap_start: usize, heap_size: usize) {
        #[cfg(feature = "tlsf_dump")]
        {
            puts("Initializing allocator.\nGiven size: 0x");
            put_num(heap_size as u64, 16, 0);
            puts(", from 0x");
            put_num(heap_start as u64, 16, 10);
            puts("\n");

            puts("Block header size: 0x");
            put_num(Block::header_size() as u64, 16, 0);
            puts("bytes.")
        }

        // Initialise TLSF structure
        self.heap_start.set(heap_start as *mut Block);
        self.heap_size.set(heap_size);
        self.initialized.set(true);

        // Build initial block
        let initial_block = self.heap_start.get();
        let initial_block_size = heap_size - Block::header_size();

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Initial block is sized 0x");
            put_num(initial_block_size as u64, 16, 0);
            puts(" bytes!\n");
        }

        unsafe {
            (*initial_block).mark(Some(true), Some(true));
            (*initial_block).set_size(initial_block_size);
        }

        self.append_free_block(initial_block);
        self.print_debug();

        #[cfg(feature = "tlsf_dump")]
        puts("Initialize complete!\n");
    }

    fn first_level_bitmap(&self) -> FirstLevelBitmapType {
        self.first_level_bitmap.get()
    }

    fn second_level_bitmap(&self, f: usize) -> SecondLevelBitmapType {
        self.second_level_bitmap[f].get()
    }

    fn heads(&self, f: usize, s: usize) -> *mut Block {
        self.heads[f][s].get()
    }

    fn reserve_exisiting_block(&self, f: usize, s: usize) -> Option<*mut Block> {
        if (self.first_level_bitmap() & (1 << f) == 0)
            || (self.second_level_bitmap(f) & (1 << s) == 0)
        {
            return None;
        }
        let reserve_block = self.heads(f, s);
        let reserve_block_ref = match unsafe { reserve_block.as_ref() } {
            Some(x) => x,
            None => {
                puts("Tlsf critical error! Block is expected but found null pointer\n");
                return None;
            }
        };

        self.detach_free_block(reserve_block_ref);
        unsafe {
            (*reserve_block).mark(Some(false), None);
        }
        Some(reserve_block)
    }

    fn reserve_larger_block(&self, f: usize, s: usize) -> Option<*mut Block> {
        if (self.first_level_bitmap() & 1 << f) != 0 {
            // Look for Larger `s` but same `f` blocks
            let mut i = s + 1;
            while i < SLI_POW2 && (self.second_level_bitmap(f) & (1 << i)) == 0 {
                i += 1;
            }

            if i < SLI_POW2 {
                return self.reserve_exisiting_block(f, i);
            }
        }

        // Look for larger `f` block
        let new_f = {
            let mut i = f + 1;
            // FIXME: Use correct max
            while (self.first_level_bitmap() & 1 << i) == 0 && i < FLI_MAX {
                i += 1;
            }
            i
        };

        #[cfg(feature = "tlsf_dump")]
        {
            puts("new_f:");
            put_num(new_f as u64, 10, 0);
            puts("\n");
        }

        if new_f == FLI_MAX {
            #[cfg(feature = "tlsf_dump")]
            puts("new_f is FLI_MAX!\n");
            return None;
        }

        let new_s = {
            let mut i = 0;
            while i < SLI_POW2 && (self.second_level_bitmap(new_f) & (1 << i)) == 0 {
                i += 1;
            }
            i
        };

        #[cfg(feature = "tlsf_dump")]
        {
            puts("new_s:");
            put_num(new_s as u64, 10, 0);
            puts("\n");
        }

        if new_s == SLI_POW2 {
            #[cfg(feature = "tlsf_dump")]
            puts("new_s is SLI_POW2!\n");
            return None;
        }

        return self.reserve_exisiting_block(new_f, new_s);
    }

    fn set_head(&self, f: usize, s: usize, block: *mut Block) {
        self.heads[f][s].set(block);
    }

    fn append_free_block(&self, block: *mut Block) {
        let (f, s) = Self::mapping(unsafe { (*block).arena_size() });
        unsafe { (*block).mark(Some(true), None) };

        if self.heads(f, s).is_null() {
            self.set_head(f, s, block);
            unsafe {
                (*block).prev_free.set(ptr::null_mut());
                (*block).next_free.set(ptr::null_mut());
            }
            self.first_level_bitmap
                .set(self.first_level_bitmap() | 1 << f);
            self.second_level_bitmap[f].set(self.second_level_bitmap(f) | 1 << s);
        } else {
            unsafe {
                (*block).prev_free.set(ptr::null_mut());
                (*block).next_free.set(self.heads(f, s));
                (*self.heads(f, s)).prev_free.set(block);
            }
            self.set_head(f, s, block);
        }
    }

    /// Shrink given block to specified size, and fit it to the layout
    fn shrink_block(&self, current_block: *mut Block, layout: Layout) {
        let shrinked_block_arena_size = layout.size();
        let shrinked_block_size_total = shrinked_block_arena_size + Block::header_size();

        let current_block_arena_size = unsafe { (*current_block).arena_size() };
        let current_block_is_last = unsafe { (*current_block).is_last() };

        if current_block_arena_size <= layout.size() + Block::header_size() + 4 {
            // Current block is too small to shrink. (Later block must be at least 4 bytes long.)
            return;
        }

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Current block arena size 0x");
            put_num(current_block_arena_size as u64, 16, 0);
            puts("\n");
        }

        let next_block_of_current_block =
            unsafe { current_block.byte_add(Block::header_size() + current_block_arena_size) };

        #[cfg(feature = "tlsf_dump")]
        {
            puts("got next block of shrinking block. 0x");
            put_num(next_block_of_current_block as u64, 16, 0);
            puts("\n");
        }

        unsafe {
            (*current_block).set_size(shrinked_block_arena_size);
            (*current_block).mark(None, Some(false));
        }

        // Construct new block by the rest part
        let remaining_size = current_block_arena_size - shrinked_block_arena_size;
        let new_block_arena_size = remaining_size - Block::header_size();

        let new_block_start = unsafe { current_block.byte_add(shrinked_block_size_total) };
        let new_block = new_block_start as *mut Block;

        unsafe {
            *new_block = Block {
                size: Cell::new(new_block_arena_size),
                prev_phys_block: Cell::new(current_block),
                prev_free: Cell::new(core::ptr::null_mut()),
                next_free: Cell::new(core::ptr::null_mut()),
            };
            (*new_block).mark(Some(true), Some(current_block_is_last));
        }

        #[cfg(feature = "tlsf_dump")]
        {
            puts("New block: 0x");
            put_num(new_block as u64, 16, 0);
            puts("\n");
            puts("Shrink done. setting phys block.\n");
        }

        if next_block_of_current_block
            < unsafe { self.heap_start.get().byte_add(self.heap_size.get()) }
        {
            unsafe {
                (*next_block_of_current_block)
                    .prev_phys_block
                    .set(new_block);
            }
        }

        #[cfg(feature = "tlsf_dump")]
        puts("Phys block set.\n");
        self.append_free_block(new_block);
    }

    fn initialized(&self) -> bool {
        self.initialized.get()
    }

    /// Detach a block from linked list.
    pub fn detach_free_block(&self, target: &Block) {
        let prev_free_block = target.prev_free.get();
        let next_free_block = target.next_free.get();

        let (f, s) = Self::mapping(target.arena_size());

        if prev_free_block.is_null() && next_free_block.is_null() {
            self.set_head(f, s, ptr::null_mut());
            self.first_level_bitmap
                .set(self.first_level_bitmap() & !(1 << f));
            self.second_level_bitmap[f].set(self.second_level_bitmap(f) & !(1 << s));
        } else if prev_free_block.is_null() {
            self.set_head(f, s, next_free_block);

            target.next_free.set(ptr::null_mut());
            unsafe {
                (*next_free_block).prev_free.set(ptr::null_mut());
            }
        } else if next_free_block.is_null() {
            target.prev_free.set(ptr::null_mut());
            unsafe {
                (*prev_free_block).next_free.set(ptr::null_mut());
            }
        } else {
            unsafe {
                (*next_free_block).prev_free.set(prev_free_block);
                (*prev_free_block).next_free.set(next_free_block);
            }
            target.next_free.set(ptr::null_mut());
            target.prev_free.set(ptr::null_mut());
        }
    }

    fn print_debug(&self) {
        #[cfg(feature = "tlsf_dump")]
        self.print_debug_anyway();
    }

    #[cfg(feature = "tlsf_dump")]
    fn print_debug_anyway(&self) {
        use core::ptr::null_mut;

        let heap_start = self.heap_start.get();
        let mut block = heap_start;
        let heap_end = unsafe { block.byte_add(self.heap_size.get()) };

        puts("Memory allocator dump ------------------\n");

        puts("Heap starts: 0x");
        put_num(heap_start as u64, 16, 0);
        puts("\n Heap ends: 0x");
        put_num(heap_end as u64, 16, 0);
        puts("\nFirst level index:");
        put_num(self.first_level_bitmap() as u64, 2, 32);
        puts("\n");
        puts("Second level bitmaps:\n");
        for f in 0..FLI_MAX {
            let slb = self.second_level_bitmap(f) as u64;
            if slb != 0 {
                put_num(f as u64, 10, 2);
                puts("\t");
                put_num(slb, 2, 16);
                puts("\n");
            }
        }

        let mut prev_block = 0;
        let mut conflict_count = 0;

        while block.addr() < heap_end.addr() {
            let current_block_addr = block as u64;
            let current_block_size = unsafe { (*block).arena_size() };
            let prev_addr = unsafe { (*block).prev_phys_block.get().addr() } as u64;

            if prev_block != prev_addr {
                conflict_count += 1;
                puts("<WARN: Prev conflict detected!>\n");

                if conflict_count > 10 {
                    puts("abort: Too many conflicts\n");
                }
                break;
            }
            puts("[");
            put_num(current_block_addr, 16, 8);
            puts(" - ");
            put_num(
                current_block_addr + (Block::header_size() + current_block_size) as u64,
                16,
                8,
            );

            puts(" ");
            puts("(0x");
            put_num(current_block_size as u64, 16, 8);
            puts(" bytes arena. Prev: 0x");
            put_num(prev_addr, 16, 8);

            puts(") ");

            let status = match unsafe { (*block).is_free() } {
                true => "Free",
                false => "Busy",
            };

            puts(status);

            puts(" ");

            let is_last = match unsafe { (*block).is_last() } {
                true => "Last",
                false => "    ",
            };

            puts(is_last);
            puts(" ]\n");

            if unsafe { (*block).is_last() } {
                break;
            }

            prev_block = block as u64;
            block = unsafe { block.byte_add(Block::header_size() + current_block_size) };
        }
    }

    fn dealloc_inner(&self, target_block: *mut Block) -> *mut Block {
        let target_block = unsafe { target_block.as_mut() }.unwrap();
        let previous_block = target_block.prev_phys_block.get();

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Got prev pointer. 0x");
            put_num(previous_block as u64, 16, 0);
            puts(" bytes.\n");
        }

        if previous_block.is_null() {
            #[cfg(feature = "tlsf_dump")]
            puts("Prev is null! It's the first block!\n");
            return target_block;
        }

        let previous_block = unsafe { previous_block.as_mut() }.unwrap();

        #[cfg(feature = "tlsf_dump")]
        puts("Got prev reference\n");

        if previous_block.is_free() {
            #[cfg(feature = "tlsf_dump")]
            {
                puts("prev is free. recurseing\n");
                puts("Prev's arena size is 0x");
                put_num(previous_block.arena_size() as u64, 16, 0);
                puts(" bytes\n");
            }

            self.detach_free_block(previous_block);

            previous_block.stretch(target_block.arena_size() + Block::header_size());
            previous_block.mark(None, Some(target_block.is_last()));
            self.dealloc_inner(previous_block)
        } else {
            #[cfg(feature = "tlsf_dump")]
            puts("prev is not free\n");
            target_block
        }
    }
}

unsafe impl GlobalAlloc for TLSF {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !self.initialized() {
            panic!("Allocator is not initialized");
        }

        let size_to_allocate = max((layout.size() + 3) & !(0b11), 4);
        let layout = Layout::from_size_align(size_to_allocate, layout.align())
            .expect("Failed to compute the size to allocate");

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Allocating 0x");
            put_num(layout.size() as u64, 16, 0);
            puts(" bytes. (Align: 0x");
            put_num(layout.align() as u64, 16, 0);
            puts(" )\n");
        }

        let (f, s) = Self::mapping(size_to_allocate);

        #[cfg(feature = "tlsf_dump")]
        {
            puts("f=");
            put_num(f as u64, 10, 0);
            puts(",s=");
            put_num(s as u64, 10, 0);
            puts("\n");
        }

        // Existing and just-sized block is the first priority
        if let Some(x) = self.reserve_exisiting_block(f, s) {
            #[cfg(feature = "tlsf_dump")]
            {
                puts("Found a block which just-fits. at 0x");
                put_num(x as u64, 16, 0);
                puts("\n");
            }

            self.print_debug();
            return (*x).arena();
        }

        // Search for larger block
        if let Some(x) = self.reserve_larger_block(f, s) {
            #[cfg(feature = "tlsf_dump")]
            {
                puts("Shrinking block: 0x");
                put_num(x as u64, 16, 0);
                puts("\n");
            }

            self.shrink_block(x, layout);
            self.print_debug();
            return (*x).arena();
        } else {
            return core::ptr::null_mut();
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let target_block_ptr = (ptr as usize - Block::header_size()) as *mut Block;

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Deallocating the block from 0x");
            put_num(target_block_ptr as u64, 16, 0);
            puts("~.\n");
        }

        let target_block_ref = target_block_ptr.as_mut().unwrap();
        let next_block_ptr =
            target_block_ptr.byte_add(Block::header_size() + target_block_ref.arena_size());

        #[cfg(feature = "tlsf_dump")]
        puts("Computed the pointer\n");

        let merged_block = self.dealloc_inner(target_block_ref);

        #[cfg(feature = "tlsf_dump")]
        {
            puts("Block recursively merged to 0x");
            put_num(merged_block as *mut Block as u64, 16, 0);
            puts("~.\n");
        }

        unsafe {
            (*next_block_ptr).prev_phys_block.set(merged_block);
        }

        #[cfg(feature = "tlsf_dump")]
        puts("Free block fixed\n");

        self.append_free_block(merged_block);
        self.print_debug();
    }
}

unsafe impl Sync for TLSF {}
impl TLSF {
    const SLB_INIT: Cell<SecondLevelBitmapType> = Cell::new(0);
    const HEAD_INIT_PRIMITIVE: Cell<*mut Block> = Cell::new(core::ptr::null_mut());
    const HEAD_INIT: [Cell<*mut Block>; SLI_POW2] = [Self::HEAD_INIT_PRIMITIVE; SLI_POW2];
    const fn default() -> Self {
        Self {
            initialized: Cell::new(false),
            heap_start: Cell::new(ptr::null_mut()),
            heap_size: Cell::new(0),
            first_level_bitmap: Cell::new(0),
            second_level_bitmap: [Self::SLB_INIT; SECOND_LEVEL_BITMAP_LENGTH],
            heads: [Self::HEAD_INIT; FLI_MAX],
        }
    }
}

#[global_allocator]
pub static GLOBAL: TLSF = TLSF::default();
