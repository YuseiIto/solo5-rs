#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ecn(pub Option<bool>);

impl From<u8> for Ecn {
    fn from(value: u8) -> Self {
        let ect = value & 0b10 != 0;
        let ce = value & 0b1 != 0;
        match ect {
            true => Self(Some(ce)),
            false => Self(None),
        }
    }
}

impl Into<u8> for Ecn {
    fn into(self) -> u8 {
        let ect = self.0.is_some();
        let ce = self.0.unwrap_or_else(|| false);
        (ect as u8) << 1 | (ce as u8)
    }
}

impl Ecn {
    pub fn ect(&self) -> bool {
        self.0.is_some()
    }

    /// Returns the content of CE bit if ect==true.
    /// Otherwise it panics.
    pub fn ce(&self) -> bool {
        self.0.unwrap()
    }
}
