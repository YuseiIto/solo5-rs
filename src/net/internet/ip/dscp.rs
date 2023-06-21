#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dscp {
    priority: u8, // Greater is more priority
    d: bool,      // Less delay
    t: bool,      // Betetr throughput
    r: bool,      // Better reliability
}

impl From<u8> for Dscp {
    fn from(value: u8) -> Self {
        Self {
            priority: value >> 5,
            d: value & (1 << 4) != 0,
            t: value & (1 << 3) != 0,
            r: value & (1 << 2) != 0,
        }
    }
}

impl Dscp {
    pub fn new(priority: u8, d: bool, t: bool, r: bool) -> Self {
        Dscp { priority, d, t, r }
    }
}

impl Into<u8> for Dscp {
    fn into(self) -> u8 {
        self.priority << 5 | (self.d as u8) << 4 | (self.t as u8) << 3 | (self.r as u8) << 2
    }
}
