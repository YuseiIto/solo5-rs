use alloc::vec::Vec;

/// MAC Address representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddr([u8; 6]);

impl From<&[u8; 6]> for MacAddr {
    fn from(value: &[u8; 6]) -> Self {
        Self(value.clone())
    }
}

impl From<&[u8]> for MacAddr {
    fn from(value: &[u8]) -> Self {
        let sized_value = [value[0], value[1], value[2], value[3], value[4], value[5]];
        Self(sized_value)
    }
}

impl From<&Vec<u8>> for MacAddr {
    fn from(value: &Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl Into<[u8; 6]> for MacAddr {
    fn into(self) -> [u8; 6] {
        return self.0;
    }
}

impl Into<Vec<u8>> for MacAddr {
    fn into(self) -> Vec<u8> {
        return Vec::from(Into::<[u8; 6]>::into(self));
    }
}
