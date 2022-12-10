use std::mem;

use crate::library::common::network::packet::PacketReader;
use crate::library::common::network::ReadError;

pub struct Ethernet2Header {
    pub destination: [u8; 6],
    pub source: [u8; 6],
    pub ether_type: u16
}

impl Ethernet2Header {
    const SIZE: usize = mem::size_of::<Ethernet2Header>();

    pub fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Ethernet2Header, ReadError> {
        let bytes = packet_reader.read(Self::SIZE)?;

        Ok(Ethernet2Header {
            destination: bytes[..6].try_into()?,
            source: bytes[6..12].try_into()?,
            ether_type: u16::from_be_bytes(bytes[12..14].try_into()?)
        })
    }
}