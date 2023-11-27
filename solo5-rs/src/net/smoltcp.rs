use alloc::rc::Rc;
use alloc::{vec, vec::Vec};
use core::cell::RefCell;

use super::raw::NetworkDevice;
use crate::Solo5Error;
use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;

/// A virtual tap interface which is mapped to solo5 network interface.

#[derive(Debug)]
pub struct Solo5NetInterface {
    lower: Rc<RefCell<NetworkDevice>>,
    mtu: usize,
    medium: Medium,
}

impl Solo5NetInterface {
    /// Attaches to a net interface called `name`, or creates it if it does not exist.
    pub fn new(name: &str) -> Result<Self, Solo5Error> {
        let lower = NetworkDevice::acquire(name)?;
        let mtu = lower.mtu() as usize;
        Ok(Self {
            lower: Rc::new(RefCell::new(lower)),
            mtu,
            medium: Medium::Ethernet,
        })
    }
}

impl Device for Solo5NetInterface {
    type RxToken<'a> = RxToken;
    type TxToken<'a> = TxToken;

    fn capabilities(&self) -> DeviceCapabilities {
        let mut cap = DeviceCapabilities::default();
        cap.max_transmission_unit = self.mtu;
        cap.medium = self.medium;
        return cap;
    }

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut lower = self.lower.borrow_mut();
        let mut buffer = vec![0; self.mtu];
        match lower.read(self.mtu as usize, &mut buffer[..]) {
            Ok(size) => {
                buffer.resize(size, 0);
                let rx = RxToken { buffer };
                let tx = TxToken {
                    lower: self.lower.clone(),
                };
                Some((rx, tx))
            }
            Err(Solo5Error::Again) => None,
            Err(err) => panic!("{}", err),
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(TxToken {
            lower: self.lower.clone(),
        })
    }
}

#[doc(hidden)]
pub struct RxToken {
    buffer: Vec<u8>,
}

impl phy::RxToken for RxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.buffer[..])
    }
}

#[doc(hidden)]
pub struct TxToken {
    lower: Rc<RefCell<NetworkDevice>>,
}

impl phy::TxToken for TxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut lower = self.lower.borrow_mut();
        let mut buffer = vec![0; len];
        let result = f(&mut buffer);
        match lower.write(&buffer[..]) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }
        result
    }
}
