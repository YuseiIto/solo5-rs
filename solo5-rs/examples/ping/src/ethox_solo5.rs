use ethox::layer;
use ethox::managed::Partial;
use ethox::nic::common::{EnqueueFlag, PacketInfo};
use ethox::nic::{self, Capabilities, Device, Packet, Personality};
use ethox::time::Instant;
use ethox::wire::{ethernet::Address, PayloadMut};
use solo5_rs::consoleln;
use solo5_rs::net::common::NetworkDevice;
use solo5_rs::Solo5Error;

#[derive(Debug)]
pub struct Solo5NetInterfaceDesc {
    lower: NetworkDevice,
}

#[derive(Debug)]
pub struct Solo5NetInterface<C> {
    inner: Solo5NetInterfaceDesc,
    buffer: Partial<C>,
    last_err: Option<Solo5Error>,
}

enum Received {
    Ok,
    Err(ethox::layer::Error),
}

impl Solo5NetInterfaceDesc {
    pub fn new(name: &str) -> Result<Solo5NetInterfaceDesc, Solo5Error> {
        match NetworkDevice::acquire(name) {
            Ok(lower) => Ok(Self { lower }),
            Err(e) => Err(e),
        }
    }

    pub fn interface_mac(&self) -> [u8; 6] {
        self.lower.mac_addr().into()
    }

    pub fn interface_mtu(&mut self) -> Result<usize, Solo5Error> {
        // NOTE: This should always success. Would be replaced with type ! (never).
        Ok(self.lower.mtu() as usize)
    }

    /// Receive a single message on the tap into the buffer.
    pub fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, Solo5Error> {
        let len = buffer.len();

        // FIXME: The duration 1us was specified without much thought, so there is room for reconsideration.
        if !self.lower.wait_until_timeout(1) {
            return Ok(0);
        }

        let read = match self.lower.read(len) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        for i in 0..read.len() {
            buffer[i] = read[i];
        }
        Ok(read.len())
    }

    /// Send a single message onto the tap from the buffer.
    pub fn send(&mut self, buffer: &[u8]) -> Result<usize, Solo5Error> {
        consoleln!("Sending...");
        match self.lower.write(buffer) {
            Ok(_) => Ok(buffer.len()),
            Err(e) => Err(e),
        }
    }
}

impl<C: PayloadMut> Solo5NetInterface<C> {
    /// Open a tap interface by name with one buffer for packets.
    pub fn new(name: &str, buffer: C) -> Result<Self, Solo5Error> {
        let inner = Solo5NetInterfaceDesc::new(name)?;
        Self::with_descriptor(inner, buffer)
    }

    /// Wrap an existing descriptor with a buffer into a device.
    pub fn with_descriptor(inner: Solo5NetInterfaceDesc, buffer: C) -> Result<Self, Solo5Error> {
        Ok(Solo5NetInterface {
            inner,
            buffer: Partial::new(buffer),
            last_err: None,
        })
    }

    pub fn mac_addr(&self) -> Address {
        Address::from_bytes(&self.inner.interface_mac())
    }

    /// Take the last io error returned by the OS.
    pub fn last_err(&mut self) -> Option<Solo5Error> {
        self.last_err.take()
    }

    /// Resize the partial buffer to its full length.
    fn recycle(&mut self) {
        let length = self.buffer.inner().payload().as_slice().len();
        self.buffer.set_len_unchecked(length);
    }

    /// Send the current buffer as a frame.
    fn send(&mut self) -> layer::Result<()> {
        let result = self.inner.send(self.buffer.payload_mut().as_mut_slice());
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(self.store_err(err)),
        }
    }

    fn recv(&mut self) -> Received {
        self.recycle();
        let result = self.inner.recv(self.buffer.payload_mut().as_mut_slice());
        match result {
            Ok(len) => {
                self.buffer.set_len_unchecked(len);
                Received::Ok
            }
            Err(err) => Received::Err(self.store_err(err)),
        }
    }

    fn store_err(&mut self, err: Solo5Error) -> ethox::layer::Error {
        let as_nic = io_error_to_layer(&err);
        self.last_err = Some(err);
        as_nic
    }
    fn current_info() -> PacketInfo {
        PacketInfo {
            timestamp: Instant::from_millis(solo5_rs::time::clock_wallc_ns() as i64 / (10 ^ 6)),
            capabilities: Capabilities::no_support(),
        }
    }
}

impl<C: PayloadMut> Device for Solo5NetInterface<C> {
    type Handle = EnqueueFlag;
    type Payload = Partial<C>;

    /// A description of the device.
    ///
    /// Could be dynamically configured but the optimizer and the user is likely happier if the
    /// implementation does not take advantage of this fact.
    fn personality(&self) -> Personality {
        Personality::baseline()
    }

    fn tx(
        &mut self,
        _: usize,
        mut sender: impl nic::Send<Self::Handle, Self::Payload>,
    ) -> layer::Result<usize> {
        let mut handle = EnqueueFlag::set_true(Self::current_info());
        self.recycle();
        sender.send(Packet {
            handle: &mut handle,
            payload: &mut self.buffer,
        });

        if handle.was_sent() {
            self.send()?;
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn rx(
        &mut self,
        _: usize,
        mut receptor: impl nic::Recv<Self::Handle, Self::Payload>,
    ) -> layer::Result<usize> {
        match self.recv() {
            Received::Ok => (),
            Received::Err(err) => return Err(err),
        }

        let mut handle = EnqueueFlag::set_true(Self::current_info());
        receptor.receive(Packet {
            handle: &mut handle,
            payload: &mut self.buffer,
        });

        if handle.was_sent() {
            self.send()?;
        }

        Ok(1)
    }
}

fn io_error_to_layer(_: &Solo5Error) -> layer::Error {
    // FIXME: not the best feed back.
    layer::Error::Illegal
}
