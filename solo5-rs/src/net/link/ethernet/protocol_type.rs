use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType {
    IpV4,
    Arp,
    AppleTalk,
    IEEE802_1q,
    IPX,
    IpV6,
}

impl ProtocolType {
    pub fn from_ethernet_value(value: &[u8; 2]) -> Option<Self> {
        let value = ((value[0] as u16) << 8) + value[1] as u16;
        match value {
            0x0800 => Some(ProtocolType::IpV4),
            0x0806 => Some(ProtocolType::Arp),
            0x809b => Some(ProtocolType::AppleTalk),
            0x8100 => Some(ProtocolType::IEEE802_1q),
            0x8137 => Some(ProtocolType::IPX),
            0x8644 => Some(ProtocolType::IpV6),
            _ => None,
        }
    }
}

impl Into<Vec<u8>> for ProtocolType {
    fn into(self) -> Vec<u8> {
        match self {
            Self::IpV4 => Vec::from([0x08, 0x00]),
            Self::Arp => Vec::from([0x08, 0x06]),
            Self::AppleTalk => Vec::from([0x80, 0x9b]),
            Self::IEEE802_1q => Vec::from([0x81, 0x00]),
            Self::IPX => Vec::from([0x81, 0x37]),
            Self::IpV6 => Vec::from([0x86, 0x44]),
        }
    }
}
