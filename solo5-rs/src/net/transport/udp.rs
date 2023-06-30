use crate::net::common::{checksum, IpAddr};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UdpPacket {
    src_port: u16,
    dest_port: u16,
    len: u16,
    checksum: u16,
    data: Vec<u8>,
}

impl From<&Vec<u8>> for UdpPacket {
    fn from(value: &Vec<u8>) -> Self {
        let src_port = u16::from_be_bytes([value[0], value[1]]);
        let dest_port = u16::from_be_bytes([value[2], value[3]]);
        let len = u16::from_be_bytes([value[4], value[5]]);
        let checksum = u16::from_be_bytes([value[6], value[7]]);
        let data = value[8..].to_vec();

        Self {
            src_port,
            dest_port,
            len,
            checksum,
            data,
        }
    }
}

impl Into<Vec<u8>> for UdpPacket {
    fn into(self) -> Vec<u8> {
        let mut tmp = Vec::new();
        tmp.append(&mut self.src_port.to_be_bytes().to_vec());
        tmp.append(&mut self.dest_port.to_be_bytes().to_vec());
        tmp.append(&mut self.len.to_be_bytes().to_vec());
        tmp.append(&mut self.checksum.to_be_bytes().to_vec());
        tmp.append(&mut self.data.to_vec());
        tmp
    }
}

impl UdpPacket {
    pub fn set_checksum_with(&mut self, src_ip: &IpAddr, dest_ip: &IpAddr) {
        let mut pseudo_header: Vec<u8> = Vec::new();
        pseudo_header.append(&mut src_ip.0.to_vec());
        pseudo_header.append(&mut dest_ip.0.to_vec());
        pseudo_header.push(0);
        pseudo_header.push(0x11);
        pseudo_header.append(&mut self.len.to_be_bytes().to_vec());

        let mut tmp: Vec<u8> = self.clone().into();
        tmp[6] = 0;
        tmp[7] = 0;
        pseudo_header.append(&mut tmp);
        self.checksum = checksum(&pseudo_header);
    }

    pub fn src_port(&self) -> u16 {
        self.src_port
    }

    pub fn dest_port(&self) -> u16 {
        self.dest_port
    }
    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn checksum(&self) -> u16 {
        self.checksum
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn new(
        src_ip: &IpAddr,
        src_port: u16,
        dest_ip: &IpAddr,
        dest_port: u16,
        data: &Vec<u8>,
    ) -> Self {
        let mut packet = Self {
            src_port,
            dest_port,
            len: data.len() as u16 + 8,
            checksum: 0,
            data: data.to_vec(),
        };
        packet.set_checksum_with(src_ip, dest_ip);
        packet
    }
}
