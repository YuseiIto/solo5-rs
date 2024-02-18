#![no_std]
#![feature(alloc_error_handler)]
#![feature(strict_provenance)]

use linked_list_allocator::LockedHeap;
extern crate alloc;

#[macro_use]
pub mod console;
pub mod block;

#[cfg(not(test))]
// Language items causes duplicate definition conflict for test since test depends on std
pub mod lang;

// pub mod tlsf;
pub use block::BlockDevice;
mod result;
pub use result::{Solo5Error, Solo5Result};
pub mod time;
pub use solo5_rs_macros::main;
mod misc;
pub use misc::*;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(feature = "net")]
pub mod net;
