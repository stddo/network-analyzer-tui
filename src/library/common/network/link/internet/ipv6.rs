use crate::library::common::network::packet::PacketReader;
use crate::network::ReadError;

pub struct Ipv6Header {
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_addr: [u8; 16],
    pub dst_addr: [u8; 16]
}

impl Ipv6Header {
    const SIZE: usize = 40;

    pub(crate) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Ipv6Header, ReadError> {
        let bytes = packet_reader.read(Self::SIZE)?;

        Ok(Ipv6Header {
            traffic_class: (bytes[0] << 4) | (bytes[1] >> 4),
            flow_label: u32::from_be_bytes([0, bytes[1] & 0x0F, bytes[2], bytes[3]]),
            payload_length: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            next_header: bytes[7],
            hop_limit: bytes[8],
            src_addr: bytes[8..24].try_into().unwrap(),
            dst_addr: bytes[24..40].try_into().unwrap()
        })
    }
}