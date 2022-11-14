use crate::common::network::ethernet2::Ethernet2Frame;
use crate::common::network::ReadError;

pub mod internet;

pub fn from_ethernet_bytes(bytes: &[u8]) -> Result<Ethernet2Frame, ReadError> {
    Ethernet2Frame::new(bytes)
}