#![feature(format_args_nl)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use solo5_rs::{consoleln, Solo5StartInfo};

#[solo5_rs::main]
fn main(start: Solo5StartInfo) -> u64 {
    consoleln!("Command line arguments: {}", start.cmdline);
    consoleln!("Heap size: 0x{:x}", start.heap_size);

    return 22;
}
