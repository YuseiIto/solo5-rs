use thiserror_no_std::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType {
    Ipv4,
}

// TODO: Implement more protocols

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum ProtocolTypeError {
    #[error("Unknown protocol type. Code:{0}")]
    UnknownProtocol(u16),
}

impl TryFrom<u16> for ProtocolType {
    type Error = ProtocolTypeError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0800 => Ok(Self::Ipv4),
            _ => Err(ProtocolTypeError::UnknownProtocol(value)),
        }
    }
}

impl Into<[u8; 2]> for ProtocolType {
    // NOTE: The return value must be in network byte order (i.e. big endian)
    fn into(self) -> [u8; 2] {
        match self {
            Self::Ipv4 => [8, 0],
        }
    }
}
