#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IpAddr(pub [u8; 4]);

impl From<&[u8]> for IpAddr {
    fn from(value: &[u8]) -> Self {
        let value = [value[0], value[1], value[2], value[3]];
        Self(value)
    }
}

impl Into<[u8; 4]> for IpAddr {
    fn into(self) -> [u8; 4] {
        self.0
    }
}
