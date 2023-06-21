use alloc::vec::Vec;
use thiserror_no_std::Error;

mod hardware_type;
pub use hardware_type::*;

mod protocol_type;
pub use protocol_type::*;

mod opcode;
pub use opcode::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArpPacket {
    pub hardware_type: HardWareType,
    pub protocol_type: ProtocolType,
    pub hardware_size: usize,
    pub protocol_size: usize,
    pub opcode: ArpOpCode,
    pub sender_hardware_address: Vec<u8>,
    pub sender_protocol_address: Vec<u8>,
    pub target_hardware_address: Vec<u8>,
    pub target_protocol_address: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum ArpPacketError {
    #[error("Hardware type error. Reason:{0}")]
    HarwareType(HardWareTypeError),
    #[error("Protocol type error. Reason:{0}")]
    ProtocolType(ProtocolTypeError),
    #[error("Opcode error. Reason:{0}")]
    OpCode(ArpOpCodeError),
}

impl TryFrom<&Vec<u8>> for ArpPacket {
    type Error = ArpPacketError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let hardware_type_u16 = u16::from_be_bytes([value[0], value[1]]);
        let hardware_type = hardware_type_u16
            .try_into()
            .map_err(|e| ArpPacketError::HarwareType(e))?;

        let protocol_type_u16 = u16::from_be_bytes([value[2], value[3]]);
        let protocol_type = protocol_type_u16
            .try_into()
            .map_err(|e| ArpPacketError::ProtocolType(e))?;

        let hardware_size = value[4] as usize;
        let protocol_size = value[5] as usize;

        let opcode_u16 = u16::from_be_bytes([value[6], value[7]]);
        let opcode = opcode_u16
            .try_into()
            .map_err(|e| ArpPacketError::OpCode(e))?;

        let sender_hardware_address = &value[8..8 + hardware_size];
        let sender_hardware_address = Vec::from(sender_hardware_address);

        let sender_protocol_address = &value[8 + hardware_size..8 + hardware_size + protocol_size];
        let sender_protocol_address = Vec::from(sender_protocol_address);

        let target_hardware_address = &value
            [8 + hardware_size + protocol_size..8 + hardware_size + protocol_size + hardware_size];
        let target_hardware_address = Vec::from(target_hardware_address);

        let target_protocol_address = &value
            [8 + hardware_size * 2 + protocol_size..8 + hardware_size * 2 + protocol_size * 2];
        let target_protocol_address = Vec::from(target_protocol_address);

        Ok(Self {
            hardware_type,
            protocol_type,
            hardware_size,
            protocol_size,
            opcode,
            sender_hardware_address,
            sender_protocol_address,
            target_hardware_address,
            target_protocol_address,
        })
    }
}

impl Into<Vec<u8>> for ArpPacket {
    fn into(mut self) -> Vec<u8> {
        let mut v = Vec::new();

        v.append(&mut Into::<[u8; 2]>::into(self.hardware_type).to_vec());
        v.append(&mut Into::<[u8; 2]>::into(self.protocol_type).to_vec());
        v.append(&mut (self.hardware_size as u8).to_be_bytes().to_vec());
        v.append(&mut (self.protocol_size as u8).to_be_bytes().to_vec());
        v.append(&mut Into::<[u8; 2]>::into(self.opcode).to_vec());
        v.append(&mut self.sender_hardware_address);
        v.append(&mut self.sender_protocol_address);
        v.append(&mut self.target_hardware_address);
        v.append(&mut self.target_protocol_address);

        v
    }
}

impl ArpPacket {
    pub fn reply_with(&self, my_hardware_address: &Vec<u8>) -> Self {
        Self {
            hardware_type: self.hardware_type,
            protocol_type: self.protocol_type,
            hardware_size: self.hardware_size,
            protocol_size: self.protocol_size,
            opcode: ArpOpCode::Reply,
            sender_hardware_address: my_hardware_address.to_vec(),
            sender_protocol_address: self.target_protocol_address.to_vec(),
            target_hardware_address: self.sender_hardware_address.to_vec(),
            target_protocol_address: self.sender_protocol_address.to_vec(),
        }
    }

    pub fn sender_hardware_address(&self) -> Vec<u8> {
        self.sender_hardware_address.to_vec()
    }

    pub fn sender_protocol_address(&self) -> Vec<u8> {
        self.sender_protocol_address.to_vec()
    }

    pub fn target_hardware_address(&self) -> Vec<u8> {
        self.target_hardware_address.to_vec()
    }

    pub fn target_protocol_address(&self) -> Vec<u8> {
        self.target_protocol_address.to_vec()
    }
}
