use alloc::string::String;
use solo5_sys::{
    solo5_result_t, solo5_result_t_SOLO5_R_AGAIN as SOLO5_R_AGAIN,
    solo5_result_t_SOLO5_R_EINVAL as SOLO5_R_EINVAL,
    solo5_result_t_SOLO5_R_EUNSPEC as SOLO5_R_EUNSPEC, solo5_result_t_SOLO5_R_OK as SOLO5_R_OK,
};
use thiserror_no_std::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum Solo5Error {
    #[error("Try again")]
    Again,
    #[error("Ivnalid argument")]
    InvalidArgs,
    #[error("Unspecified error")]
    Unspecified,
    #[error("Validation Error. Reason: {0}")]
    ValidationError(String),
}

impl From<solo5_result_t> for Solo5Error {
    fn from(raw_res: solo5_result_t) -> Self {
        match raw_res {
            SOLO5_R_OK => {
                panic!("SOLO5_R_OK can't be converted to Solo5Error since it's not an error")
            }
            SOLO5_R_AGAIN => Self::Again,
            SOLO5_R_EINVAL => Self::InvalidArgs,
            SOLO5_R_EUNSPEC => Self::Unspecified,
            e => panic!("Unknown solo5_result_t error code given. Value:{}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Solo5Result<T>(Result<T, Solo5Error>);

impl<T> Solo5Result<T> {
    pub fn from(value: solo5_result_t, success: T) -> Result<T, Solo5Error> {
        match value {
            SOLO5_R_OK => Ok(success),
            _ => Err(Solo5Error::from(value)),
        }
    }
}

impl<T> From<Solo5Result<T>> for Result<T, Solo5Error> {
    fn from(val: Solo5Result<T>) -> Self {
        val.0
    }
}
