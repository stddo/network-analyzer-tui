use crate::common::network::SufficientOffset;
use crate::network::ReadError;

pub struct UDPHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16
}

impl SufficientOffset for UDPHeader {
    const SIZE: usize = 8;
}

impl UDPHeader {
    pub fn new(bytes: &[u8]) -> Result<UDPHeader, ReadError> {
        Self::assert_offset_size(bytes.len())?;

        Ok(UDPHeader {
            src_port: u16::from_be_bytes(bytes[..2].try_into().unwrap()),
            dst_port: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            length: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            checksum: u16::from_be_bytes(bytes[6..8].try_into().unwrap())
        })
    }
}