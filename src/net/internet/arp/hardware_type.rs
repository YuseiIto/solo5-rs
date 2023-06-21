use thiserror_no_std::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardWareType {
    Ethernet,
}

// TODO: Implement more hardwares

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum HardWareTypeError {
    #[error("Unknown hardware type. Code:{0}")]
    UnknownHardware(u16),
}

impl TryFrom<u16> for HardWareType {
    type Error = HardWareTypeError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(Self::Ethernet),
            _ => Err(HardWareTypeError::UnknownHardware(value)),
        }
    }
}

impl Into<[u8; 2]> for HardWareType {
    // NOTE: The return value must be in network byte order (i.e. big endian)

    fn into(self) -> [u8; 2] {
        match self {
            Self::Ethernet => [0, 1],
        }
    }
}
