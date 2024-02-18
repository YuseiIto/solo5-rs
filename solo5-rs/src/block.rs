use crate::result::Solo5Error;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use solo5_sys::solo5_block_info;

#[derive(Debug, Clone)]
pub struct BlockDevice {
    name: String,
    handle: u64,
    info: solo5_block_info,
}

impl BlockDevice {
    pub fn acquire(name: &str) -> Result<Self, Solo5Error> {
        let mut handle = 0_u64;
        let mut info = solo5_block_info {
            capacity: 1212,
            block_size: 1212,
        };

        let name_c = alloc::ffi::CString::new(name).unwrap();
        let acquire_result = unsafe {
            solo5_sys::solo5_block_acquire(
                name_c.as_ptr(),
                core::ptr::addr_of_mut!(handle),
                core::ptr::addr_of_mut!(info),
            )
        };

        match acquire_result {
            0 => Ok(Self {
                name: name.to_string(),
                handle,
                info,
            }),
            1 => Err(Solo5Error::Again),
            2 => Err(Solo5Error::InvalidArgs),
            3 => Err(Solo5Error::Unspecified),
            _ => unreachable!(),
        }
    }

    pub fn write(&self, pos_offset: usize, bytes: &[u8]) -> Result<(), Solo5Error> {
        let pos_offset = pos_offset as u64;
        if pos_offset % self.info.block_size != 0 {
            return Err(Solo5Error::ValidationError(
                "Argument pos_offset must be multiple of block size".to_string(),
            ));
        }

        let size = bytes.len() as u64;
        if size % self.info.block_size != 0 {
            return Err(Solo5Error::ValidationError(
                "Byte length must be multiple of block size".to_string(),
            ));
        }

        let write_result = unsafe {
            solo5_sys::solo5_block_write(
                self.handle,
                pos_offset,
                bytes.as_ptr(),
                bytes.len() as u64,
            )
        };
        match write_result {
            0 => Ok(()),
            1 => Err(Solo5Error::Again),
            2 => Err(Solo5Error::InvalidArgs),
            3 => Err(Solo5Error::Unspecified),
            _ => unreachable!(),
        }
    }

    pub fn read(&self, pos_offset: usize, size: usize) -> Result<Vec<u8>, Solo5Error> {
        let mut buf = vec![0; size];

        let pos_offset = pos_offset as u64;
        if pos_offset % self.info.block_size != 0 {
            return Err(Solo5Error::ValidationError(
                "Argument pos_offset must be multiple of block size".to_string(),
            ));
        }

        let size = size as u64;
        if size % self.info.block_size != 0 {
            return Err(Solo5Error::ValidationError(
                "Byte length must be multiple of block size".to_string(),
            ));
        }

        let read_result =
            unsafe { solo5_sys::solo5_block_read(self.handle, pos_offset, buf.as_mut_ptr(), size) };

        match read_result {
            0 => Ok(buf),
            1 => Err(Solo5Error::Again),
            2 => Err(Solo5Error::InvalidArgs),
            3 => Err(Solo5Error::Unspecified),
            _ => unreachable!(),
        }
    }

    pub fn block_size(&self) -> usize {
        self.info.block_size as usize
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }
}
