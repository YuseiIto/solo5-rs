use alloc::vec::Vec;
use thiserror_no_std::Error;

mod protocol_type;
use crate::net::common::MacAddr;
pub use protocol_type::ProtocolType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetFrame {
    dest_mac: MacAddr,
    src_mac: MacAddr,
    protocol_type: ProtocolType,
    data: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum EthernetFrameError {
    #[error("Unknown frame error")]
    UnkownFrameError,
}

impl TryFrom<&Vec<u8>> for EthernetFrame {
    type Error = EthernetFrameError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let dest_mac = &value[0..6];
        let src_mac = &value[6..12];
        let protocol_type = &value[12..14];
        let protocol_type =
            match ProtocolType::from_ethernet_value(&[protocol_type[0], protocol_type[1]]) {
                Some(x) => x,
                None => return Err(EthernetFrameError::UnkownFrameError),
            };

        Ok(Self {
            dest_mac: dest_mac.into(),
            src_mac: src_mac.into(),
            protocol_type,
            data: value[14..].to_vec(),
        })
    }
}

impl EthernetFrame {
    pub fn protocol_type(&self) -> ProtocolType {
        self.protocol_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn with_data(from: &MacAddr, to: &MacAddr, protocol: ProtocolType, data: &Vec<u8>) -> Self {
        Self {
            dest_mac: to.clone(),
            src_mac: from.clone(),
            protocol_type: protocol,
            data: data.to_vec(),
        }
    }

    pub fn src_mac_addr(&self) -> MacAddr {
        self.src_mac.clone()
    }

    pub fn dest_mac_addr(&self) -> MacAddr {
        self.dest_mac.clone()
    }
}

impl Into<Vec<u8>> for EthernetFrame {
    fn into(self) -> Vec<u8> {
        let mut tmp = Vec::new();
        tmp.append(&mut self.dest_mac.into());
        tmp.append(&mut self.src_mac.into());
        tmp.append(&mut self.protocol_type.into());
        tmp.append(&mut self.data.to_vec());
        tmp
    }
}
