use crate::net::common::checksum;
use alloc::vec::Vec;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IcmpPacket {
    message_type: u8,
    code: u8,
    checksum: u16,
    id: u16,
    sequence: u16,
    data: Vec<u8>,
}

impl From<&Vec<u8>> for IcmpPacket {
    fn from(value: &Vec<u8>) -> Self {
        Self {
            message_type: value[0],
            code: value[1],
            checksum: u16::from_le_bytes([value[2], value[3]]),
            id: u16::from_le_bytes([value[4], value[5]]),
            sequence: u16::from_le_bytes([value[6], value[7]]),
            data: value[8..].to_vec(),
        }
    }
}

impl IcmpPacket {
    pub fn refresh_checksum(&mut self) {
        let mut data: Vec<u8> = self.clone().into();
        data[2] = 0;
        data[3] = 0;
        self.checksum = checksum(&data);
    }

    pub fn into_reply(mut self) -> Self {
        self.message_type = 0;
        self.refresh_checksum();
        self
    }
}

impl Into<Vec<u8>> for IcmpPacket {
    fn into(self) -> Vec<u8> {
        let mut tmp = Vec::new();
        tmp.push(self.message_type);
        tmp.push(self.code);
        tmp.append(&mut self.checksum.to_be_bytes().to_vec());
        tmp.append(&mut self.id.to_le_bytes().to_vec());
        tmp.append(&mut self.sequence.to_le_bytes().to_vec());
        tmp.append(&mut self.data.clone());
        tmp
    }
}
