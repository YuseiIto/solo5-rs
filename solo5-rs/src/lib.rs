#![no_std]
#![feature(alloc_error_handler)]
#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

extern crate alloc;

#[macro_use]
pub mod console;
pub mod block;
pub mod lang;
pub mod tlsf;
pub use block::BlockDevice;
mod result;
pub use result::{Solo5Error, Solo5Result};
pub mod time;
pub use solo5_rs_macros::main;
mod misc;
pub use misc::*;

#[cfg(feature = "net")]
pub mod net;
