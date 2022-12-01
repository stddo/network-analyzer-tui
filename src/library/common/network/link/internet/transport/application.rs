use crate::library::common::network::packet::PacketReader;
use crate::network::ReadError;

pub enum ApplicationHeader {
    Default
}

impl ApplicationHeader {
    pub fn new<'a, 'b: 'a>(_packet_reader: &'a mut PacketReader<'b>) -> Result<ApplicationHeader, ReadError> {
        Ok(ApplicationHeader::Default)
    }
}