use crate::common::network::{ReadError, SufficientOffset};

pub struct IPv4Header {
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: IPv4Flags,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub options: Vec<u8>
}

impl SufficientOffset for IPv4Header {
    const SIZE: usize = 20;
}

impl IPv4Header {
    pub fn new(bytes: &[u8]) -> Result<IPv4Header, ReadError> {
        Self::assert_offset_size(bytes.len())?;

        let ihl = bytes[0] & 0x0F;
        let options_end = ihl as usize * 4;
        let options = if ihl > 5 {
            bytes[20..options_end].to_vec()
        } else {
            vec![]
        };

        Ok(IPv4Header {
            ihl,
            dscp: bytes[1] >> 2,
            ecn: bytes[1] & 0x03,
            total_length: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            identification: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            flags: IPv4Flags::new(bytes[6]),
            fragment_offset: u16::from_be_bytes(bytes[6..8].try_into().unwrap()) & 0x1FFF,
            ttl: bytes[8],
            protocol: bytes[9],
            header_checksum: u16::from_be_bytes(bytes[10..12].try_into().unwrap()),
            src_addr: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            dst_addr: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
            options
        })
    }
}

impl IPv4Header {
    pub fn len(&self) -> usize {
        self.ihl as usize * 4
    }
}

pub struct IPv4Flags {
    pub reserved: bool,
    pub df: bool,
    pub mf: bool
}

impl IPv4Flags {
    fn new(byte: u8) -> IPv4Flags {
        IPv4Flags {
            reserved: byte & 0x80 != 0,
            df: byte & 0x40 != 0,
            mf: byte & 0x20 != 0
        }
    }
}