use crate::network::ethernet2::Ethernet2Header;
use crate::network::link::internet::IpHeader;
use crate::network::link::internet::transport::application::ApplicationHeader;
use crate::network::link::internet::transport::TransportHeader;
use crate::network::ReadError;

pub struct Packet {
    pub lp_header: Ethernet2Header,
    pub ip_header: IpHeader,
    pub tp_header: TransportHeader,
    pub ap_header: ApplicationHeader
}

impl Packet {
    pub fn from_ethernet_bytes(bytes: &[u8]) -> Result<Packet, ReadError> {
        let mut packet_reader = PacketReader::new(bytes);

        let lp_header = Ethernet2Header::new(&mut packet_reader)?;
        let ip_header = IpHeader::new(packet_reader.peek(1)?[0] >> 4, &mut packet_reader)?;
        let protocol = ip_header.protocol();
        Ok(Packet {
            lp_header,
            ip_header,
            tp_header: TransportHeader::new(protocol, &mut packet_reader)?,
            ap_header: ApplicationHeader::new(&mut packet_reader)?
        })
    }
}

pub struct PacketReader<'a> {
    bytes: &'a [u8],
    position: usize
}

impl<'a> PacketReader<'a> {
    fn new(bytes: &[u8]) -> PacketReader {
        PacketReader {
            bytes,
            position: 0
        }
    }

    fn peek<'b>(&'b self, n: usize) -> Result<&'a [u8], ReadError> {
        if self.bytes.len() < self.position + n {
            return Err(ReadError::DataOffsetTooSmall(self.position + n - self.bytes.len()));
        }

        Ok(&self.bytes[self.position..self.position + n])
    }

    pub fn read<'b>(&'b mut self, n: usize) -> Result<&'a [u8], ReadError> {
        let r = self.peek(n)?;
        self.position += n;
        Ok(r)
    }
}
