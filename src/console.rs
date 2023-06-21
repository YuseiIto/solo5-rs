use core::cmp::max;
use core::fmt::Error;
pub use core::fmt::Write;
use solo5_sys::solo5_console_write;

pub fn puts(s: &str) {
    unsafe {
        solo5_console_write(s.as_ptr() as *const i8, s.len() as u64);
    }
}

pub fn put_num(v: u64, radix: u32, least_len: usize) {
    let mut s = [0 as u8; 64];
    let required_len = match v {
        0 => 0,
        _ => v.ilog(radix as u64) as usize,
    } + 1;
    let s_len = max(least_len, required_len);

    let mut tmp = v;

    for i in (0..s_len).rev() {
        let d = tmp as u32 % radix;
        s[i] = char::from_digit(d, radix).unwrap() as u8;
        tmp /= radix as u64;
    }

    unsafe {
        solo5_console_write(s.as_ptr() as *const i8, s.len() as u64);
    }
}

pub struct Console;
impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        Ok(puts(&s))
    }
}

#[macro_export]
macro_rules! console {
    ($($arg:tt)*) => {
        let mut console=$crate::console::Console;
        //FIXME: I don't know hot to handle this unwrap properly
        $crate::console::Write::write_fmt(&mut console,(core::format_args!($($arg)*))).unwrap()
    };
}

#[macro_export]
macro_rules! consoleln {
    ($($arg:tt)*) => {
        let mut console=$crate::console::Console;
        //FIXME: I don't know hot to handle this unwrap properly
        $crate::console::Write::write_fmt(&mut console,(core::format_args_nl!($($arg)*))).unwrap()
    };
}
