use alloc::vec::Vec;
mod dscp;
mod ecn;
mod ip_flag;
mod protocol_type;

use crate::net::common::{checksum, IpAddr};
use dscp::Dscp;
use ecn::Ecn;
use ip_flag::IpFlag;
pub use protocol_type::ProtocolType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct IpPacket {
    version: u8,
    header_len: u8,
    dscp: Dscp,
    ecn: Ecn,
    total_len: u16,
    identification: u16,
    flags: IpFlag,
    offset: u16,
    ttl: u8,
    protocol: ProtocolType,
    header_checksum: u16,
    src_ip_addr: IpAddr,
    dest_ip_addr: IpAddr,
    // TODO: Implement `Options` field
    data: Vec<u8>,
}

impl IpPacket {
    pub fn protocol_type(&self) -> ProtocolType {
        self.protocol.clone()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn src_ip_addr(&self) -> IpAddr {
        self.src_ip_addr
    }

    pub fn dest_ip_addr(&self) -> IpAddr {
        self.dest_ip_addr
    }
}

impl IpPacket {
    pub fn tos(&self) -> u8 {
        (Into::<u8>::into(self.dscp) << 2) as u8 | Into::<u8>::into(self.ecn) as u8
    }

    pub fn refresh_checksum(&mut self) {
        let mut data: Vec<u8> = self.clone().into();
        data[10] = 0;
        data[11] = 0;
        let data = &data[..20];
        self.header_checksum = checksum(data);
    }

    pub fn with_version(mut self, v: u8) -> Self {
        self.version = v;
        self.refresh_checksum();
        self
    }

    pub fn with_data(mut self, data: &Vec<u8>) -> Self {
        self.data = data.to_vec();
        self.total_len = self.header_len as u16 * 4 + data.len() as u16;
        self.refresh_checksum();
        self
    }

    pub fn with_protocol(mut self, protocol: ProtocolType) -> Self {
        self.protocol = protocol;
        self.refresh_checksum();
        self
    }

    pub fn with_src(mut self, addr: IpAddr) -> Self {
        self.src_ip_addr = addr;
        self.refresh_checksum();
        self
    }

    pub fn with_dest(mut self, addr: IpAddr) -> Self {
        self.dest_ip_addr = addr;
        self.refresh_checksum();
        self
    }
}

impl Default for IpPacket {
    fn default() -> Self {
        Self {
            version: 4,
            header_len: 5,
            dscp: Dscp::new(0, false, false, false),
            ecn: Ecn(None),
            total_len: 4,
            identification: 0,
            flags: IpFlag::new(true, false),
            offset: 0,
            ttl: 0x40,
            protocol: ProtocolType::ICMP,
            header_checksum: Default::default(),
            src_ip_addr: IpAddr([0, 0, 0, 0]),
            dest_ip_addr: IpAddr([0, 0, 0, 0]),
            data: Default::default(),
        }
    }
}

impl From<&Vec<u8>> for IpPacket {
    fn from(value: &Vec<u8>) -> Self {
        let version = value[0] >> 4;
        let header_len = (value[0] << 4) >> 4;
        let dscp = Dscp::from(value[1]);
        let ecn = Ecn::from(value[1]);
        let total_len = u16::from_le_bytes([value[2], value[3]]);
        let identification = u16::from_le_bytes([value[4], value[5]]);
        let flags = IpFlag::from(value[6]);
        let offset = u16::from_le_bytes([(value[6] & 0b11111), value[7]]);
        let ttl = value[8];
        let protocol = ProtocolType::from(value[9]);
        let header_checksum = u16::from_le_bytes([value[10], value[11]]);
        let src_ip_addr = IpAddr::from(&value[12..16]);
        let dest_ip_addr = IpAddr::from(&value[16..21]);
        let data = value[(4 * header_len) as usize..].to_vec();

        Self {
            version,
            header_len,
            dscp,
            ecn,
            total_len,
            identification,
            flags,
            offset,
            ttl,
            protocol,
            header_checksum,
            src_ip_addr,
            dest_ip_addr,
            data,
        }
    }
}

impl Into<Vec<u8>> for IpPacket {
    fn into(self) -> Vec<u8> {
        let mut tmp = Vec::new();
        tmp.push((self.version << 4) | self.header_len as u8);
        tmp.push(self.tos());
        tmp.append(&mut self.total_len.to_be_bytes().to_vec());
        tmp.append(&mut self.identification.to_be_bytes().to_vec());
        tmp.push((Into::<u8>::into(self.flags)) | (self.offset >> 5) as u8);
        tmp.push((self.offset & 0xf) as u8);

        tmp.push(self.ttl);
        tmp.push(self.protocol.into());
        tmp.append(&mut self.header_checksum.to_be_bytes().to_vec());
        tmp.append(&mut self.src_ip_addr().0.to_vec());
        tmp.append(&mut self.dest_ip_addr.0.to_vec());
        tmp.append(&mut self.data());
        tmp
    }
}
