use core::cell::Cell;

#[cfg(feature = "tlsf_dump")]
use crate::console::{put_num, puts};

pub struct Block {
    pub size: Cell<usize>,
    pub prev_phys_block: Cell<*mut Self>,
    pub prev_free: Cell<*mut Self>,
    pub next_free: Cell<*mut Self>,
}

impl Block {
    pub const fn header_size() -> usize {
        return core::mem::size_of::<Self>();
    }

    pub fn arena_size(&self) -> usize {
        self.size.get() >> 2 << 2
    }

    pub fn set_size(&self, size: usize) -> usize {
        let current_flags = 0b11 & self.size.get();
        let aligned_size = size & !(0b11);
        let new_size = aligned_size | current_flags;

        #[cfg(tlsf_enable_dump)]
        {
            puts("New size is 0x");
            put_num(new_size as u64, 16, 0);
            puts("bytes\n");
        }

        self.size.set(new_size);
        new_size
    }

    /// TLSF marks Last/Free state on LSBs of size
    pub fn mark(&self, is_free: Option<bool>, is_last: Option<bool>) {
        let is_free = is_free.unwrap_or((self.size.get() & 1 as usize) != 0);
        let is_last = is_last.unwrap_or((self.size.get() & 0b10) != 0);
        self.size
            .set(self.arena_size() | (is_last as usize) << 1 | (is_free as usize))
    }

    pub fn is_last(&self) -> bool {
        (self.size.get() & 0b10) != 0
    }

    pub fn is_free(&self) -> bool {
        (self.size.get() & 0b1) != 0
    }

    pub fn arena(&mut self) -> *mut u8 {
        ((self as *mut Self as usize) + Self::header_size()) as *mut u8
    }

    /// Stretch (Extend) a block.
    /// **Don't to detach from the list before calling this to avoid conflict**
    pub fn stretch(&self, stretch_size: usize) {
        // Calculate new size
        let current_size = self.arena_size();
        let new_size = current_size + stretch_size;
        self.set_size(new_size);
    }
}
