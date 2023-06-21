#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ProtocolType {
    ICMP,
    UDP,
    Unknown(u8),
}

impl From<u8> for ProtocolType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ICMP,
            17 => Self::UDP,
            _ => Self::Unknown(value),
        }
    }
}

impl Into<u8> for ProtocolType {
    fn into(self) -> u8 {
        match self {
            Self::ICMP => 1,
            Self::UDP => 17,
            Self::Unknown(value) => value,
        }
    }
}
