use crate::library::common::network::ReadError;
use crate::library::common::network::packet::PacketReader;

pub struct Ipv4Header {
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
    pub src_addr: [u8; 4],
    pub dst_addr: [u8; 4],
    pub options: Vec<u8>
}

impl Ipv4Header {
    const SIZE: usize = 20;

    pub fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Ipv4Header, ReadError> {
        let bytes = packet_reader.read(Self::SIZE)?;

        let ihl = bytes[0] & 0x0F;
        let options_end = ihl as usize * 4;

        let options = if ihl > 5 {
            packet_reader.read(options_end)?.to_vec()
        } else {
            vec![]
        };

        Ok(Ipv4Header {
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
            src_addr: bytes[12..16].try_into().unwrap(),
            dst_addr: bytes[16..20].try_into().unwrap(),
            options
        })
    }
}

impl Ipv4Header {
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