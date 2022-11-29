use crate::common::network::ReadError;
use crate::network::link::PacketReader;

pub struct TCPHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence_number: u32,
    pub ack: u32,
    pub data_offset: u8,
    pub reserved: u8,
    pub flags: TCPFlags,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Vec<u8>
}

impl TCPHeader {
    const SIZE: usize = 20;

    pub fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<TCPHeader, ReadError> {
        let bytes = packet_reader.peek(Self::SIZE)?;

        let data_offset = bytes[12] >> 4;
        let options_end = data_offset as usize * 4;

        let bytes = if data_offset > 5 {
            packet_reader.read(options_end)?
        } else {
            bytes
        };

        let options = if data_offset > 5 {
            bytes[20..options_end].to_vec()
        } else {
            vec![]
        };

        Ok(TCPHeader {
            src_port: u16::from_be_bytes(bytes[..2].try_into().unwrap()),
            dst_port: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            sequence_number: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            ack: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            data_offset,
            reserved: bytes[12] & 0x0E,
            flags: TCPFlags::new(&bytes[12..14].try_into().unwrap()),
            window_size: u16::from_be_bytes(bytes[14..16].try_into().unwrap()),
            checksum: u16::from_be_bytes(bytes[16..18].try_into().unwrap()),
            urgent_pointer: u16::from_be_bytes(bytes[18..20].try_into().unwrap()),
            options
        })
    }
}

pub struct TCPFlags {
    pub ns: bool,
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool
}

impl TCPFlags {
    fn new(bytes: &[u8; 2]) -> TCPFlags {
        TCPFlags {
            ns: bytes[0] & 0x1 != 0,
            cwr: bytes[1] & 0x80 != 0,
            ece: bytes[1] & 0x40 != 0,
            urg: bytes[1] & 0x20 != 0,
            ack: bytes[1] & 0x10 != 0,
            psh: bytes[1] & 0x8 != 0,
            rst: bytes[1] & 0x4 != 0,
            syn: bytes[1] & 0x2 != 0,
            fin: bytes[1] & 0x1 != 0
        }
    }
}