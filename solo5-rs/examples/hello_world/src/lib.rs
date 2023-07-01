#![feature(format_args_nl)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

use solo5_rs::consoleln;

#[solo5_rs::main(alloc)]
fn main() {
    consoleln!("Hello,World");
}
