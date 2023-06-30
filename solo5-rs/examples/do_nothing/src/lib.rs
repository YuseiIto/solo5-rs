#![feature(format_args_nl)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

use core::panic::PanicInfo;
use solo5_sys::{self, solo5_start_info};

#[no_mangle]
pub extern "C" fn solo5_app_main(_: &solo5_start_info) -> u64 {
    main();
    return 0;
}

fn main() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
