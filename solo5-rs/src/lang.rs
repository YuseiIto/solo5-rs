/// Rust Language Items
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(panic: &PanicInfo) -> ! {
    console!("{}\n", panic);
    unsafe { solo5_sys::solo5_abort() }
    loop {} // Unreachable
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}
