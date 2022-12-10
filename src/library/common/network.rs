use std::array::TryFromSliceError;
use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub mod link;
pub mod ethernet2;
pub mod packet;

#[derive(Debug)]
pub enum ReadError {
    IPUnexpectedVersion(u8),
    DataOffsetTooSmall(usize),
    CouldntParse,
    UnsupportedIpExtension
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::IPUnexpectedVersion(n) => {
                write!(f, "Supported versions of IP headers are v4 and v6, passed value was {}.", n)
            }
            ReadError::DataOffsetTooSmall(n) => {
                write!(f, "Passed array of bytes is too small by {} bytes.", n)
            }
            ReadError::CouldntParse => {
                write!(f, "Couldn't parse bytes into desired representation.")
            }
            ReadError::UnsupportedIpExtension => {
                write!(f, "Passed Ip extension number is not supported.")
            }
        }
    }
}

impl Error for ReadError {}

impl From<Infallible> for ReadError {
    fn from(_: Infallible) -> Self {
        ReadError::CouldntParse
    }
}

impl From<TryFromSliceError> for ReadError {
    fn from(_: TryFromSliceError) -> Self {
        ReadError::CouldntParse
    }
}