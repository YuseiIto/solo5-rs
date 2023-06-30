#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IpFlag {
    df: bool, // Don't flagment
    mf: bool, // More flagment
}

impl From<u8> for IpFlag {
    fn from(value: u8) -> Self {
        Self {
            df: value & (1 << 6) != 0,
            mf: value & (1 << 5) != 0,
        }
    }
}

impl Into<u8> for IpFlag {
    fn into(self) -> u8 {
        (self.df as u8) << 6 | (self.mf as u8) << 5
    }
}

impl IpFlag {
    pub fn new(df: bool, mf: bool) -> Self {
        Self { df, mf }
    }
}
