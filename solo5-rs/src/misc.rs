use alloc::string::String;
use core::ffi::CStr;

pub struct Solo5StartInfo {
    pub cmdline: String,
    pub heap_start: *const u8,
    pub heap_size: u64,
}

impl From<&solo5_sys::solo5_start_info> for Solo5StartInfo {
    fn from(from: &solo5_sys::solo5_start_info) -> Self {
        let c_str = unsafe { CStr::from_ptr(from.cmdline) };
        let cmdline = c_str.to_str().unwrap();

        Self {
            cmdline: String::from(cmdline),
            heap_start: from.heap_start as *const u8,
            heap_size: from.heap_size,
        }
    }
}
