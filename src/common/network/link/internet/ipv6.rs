use crate::common::network::SufficientOffset;
use crate::network::ReadError;

pub struct IPv6Header {
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_addr: [u8; 16],
    pub dst_addr: [u8; 16]
}

impl SufficientOffset for IPv6Header {
    const SIZE: usize = 40;
}

impl IPv6Header {
    pub(crate) fn new(bytes: &[u8]) -> Result<IPv6Header, ReadError> {
        Self::assert_offset_size(bytes.len())?;

        Ok(IPv6Header {
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