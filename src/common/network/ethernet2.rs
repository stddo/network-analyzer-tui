use crate::common::network::{ReadError, SufficientOffset};
use crate::common::network::link::internet::IPFrame;

pub struct Ethernet2Frame {
    pub mac_header: MacHeader,
    pub payload: IPFrame,
    pub crc_checksum: u32
}

impl Ethernet2Frame {
    pub fn new(bytes: &[u8]) -> Result<Ethernet2Frame, ReadError> {
        Ok(Ethernet2Frame {
            mac_header: MacHeader::new(&bytes[..14])?,
            payload: IPFrame::new(&bytes[14..bytes.len() - 4])?,
            crc_checksum: u32::from_be_bytes(bytes[bytes.len() - 4..].try_into().unwrap())
        })
    }
}

pub struct MacHeader {
    pub destination: [u8; 6],
    pub source: [u8; 6],
    pub ether_type: u16
}

impl SufficientOffset for MacHeader {
    const SIZE: usize = 14;
}

impl MacHeader {
    pub fn new(bytes: &[u8]) -> Result<MacHeader, ReadError> {
        Self::assert_offset_size(bytes.len())?;

        Ok(MacHeader {
            destination: bytes[..6].try_into().unwrap(),
            source: bytes[6..12].try_into().unwrap(),
            ether_type: 0
        })
    }
}