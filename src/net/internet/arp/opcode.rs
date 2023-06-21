use thiserror_no_std::Error;

// TODO: Implement more opcodes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArpOpCode {
    Request,
    Reply,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum ArpOpCodeError {
    #[error("Unknown opcode given. Code:{0}")]
    UnknownOpCode(u16),
}

impl TryFrom<u16> for ArpOpCode {
    type Error = ArpOpCodeError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Request),
            2 => Ok(Self::Reply),
            _ => Err(ArpOpCodeError::UnknownOpCode(value)),
        }
    }
}

impl Into<[u8; 2]> for ArpOpCode {
    fn into(self) -> [u8; 2] {
        match self {
            Self::Request => [0, 1],
            Self::Reply => [0, 2],
        }
    }
}
