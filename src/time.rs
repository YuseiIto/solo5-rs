use solo5_sys::{solo5_clock_monotonic, solo5_clock_wall};

pub fn clock_monolithic_ns() -> u64 {
    unsafe { solo5_clock_monotonic() }
}

pub fn clock_wallc_ns() -> u64 {
    unsafe { solo5_clock_wall() }
}
