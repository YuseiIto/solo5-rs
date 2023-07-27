use super::MacAddr;
use crate::{time::clock_monotonic_ns, Solo5Error, Solo5Result};
use solo5_sys::{
    solo5_net_acquire, solo5_net_info, solo5_net_read, solo5_net_write, solo5_yield, SOLO5_NET_ALEN,
};

/// Represents single virtual network device.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkDevice {
    handle: u64,
    mac_addr: MacAddr,
    mtu: usize,
}

/// Constructor
impl NetworkDevice {
    /// Network device constructor.
    /// it acquires the device  and saves the device handle.
    pub fn acquire(device_name: &str) -> Result<Self, Solo5Error> {
        let name_cstr = alloc::ffi::CString::new(device_name).unwrap();
        let mut handle = 0 as u64;
        let mut info = solo5_net_info {
            mac_address: [0; SOLO5_NET_ALEN as usize],
            mtu: 0,
        };

        let acquire_result = unsafe {
            solo5_net_acquire(
                name_cstr.as_ptr() as *const i8,
                core::ptr::addr_of_mut!(handle),
                core::ptr::addr_of_mut!(info),
            )
        };

        let res = Self {
            handle,
            mac_addr: MacAddr::from(&info.mac_address),
            mtu: info.mtu as usize,
        };

        Solo5Result::from(acquire_result, res).into()
    }

    /// Read specified length of bytes from this devide
    pub fn read(&self, max_size: usize, buf: &mut [u8]) -> Result<usize, Solo5Error> {
        let mut read_size = 0;
        let res = unsafe {
            solo5_net_read(
                self.handle,
                buf.as_mut_ptr(),
                max_size as u64,
                core::ptr::addr_of_mut!(read_size),
            )
        };
        Solo5Result::from(res, read_size as usize).into()
    }

    pub fn write(&self, bytes: &[u8]) -> Result<(), Solo5Error> {
        let size = bytes.len() as u64;
        let res = unsafe { solo5_net_write(self.handle, bytes.as_ptr(), size) };
        Solo5Result::from(res, ()).into()
    }

    pub fn wait_until_timeout(&self, timeout_ns: u64) -> bool {
        let mut handles = 0;
        let deadline = clock_monotonic_ns() + timeout_ns;
        unsafe {
            solo5_yield(deadline, core::ptr::addr_of_mut!(handles));
        }
        handles & (1 << self.handle) != 0
    }

    /// Returns MTU of this data link.
    pub fn mtu(&self) -> u64 {
        self.handle
    }

    /// Returns assigned MAC Address of this device.
    pub fn mac_addr(&self) -> MacAddr {
        self.mac_addr.clone()
    }

    /// Returns solo5 handle of this device
    pub fn handle(&self) -> u64 {
        self.handle
    }
}
