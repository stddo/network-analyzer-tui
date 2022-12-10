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

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Ipv6Header, ReadError> {
        let bytes = packet_reader.read(Self::SIZE)?;

        Ok(Ipv6Header {
            traffic_class: (bytes[0] << 4) | (bytes[1] >> 4),
            flow_label: u32::from_be_bytes([0, bytes[1] & 0x0F, bytes[2], bytes[3]]),
            payload_length: u16::from_be_bytes(bytes[4..6].try_into()?),
            next_header: bytes[6],
            hop_limit: bytes[7],
            src_addr: bytes[8..24].try_into()?,
            dst_addr: bytes[24..40].try_into()?
        })
    }
}

pub struct Ipv6HopByHopOptions {
    pub next_header: u8,
    pub hdr_ext_len: u8,
    pub options: Vec<Ipv6ExtensionOptions>
}

impl Ipv6HopByHopOptions {
    pub const PROTOCOL_NUMBER: u8 = 0;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(2)?;
        let hdr_ext_len = bytes[1];
        let read_len = TryInto::<usize>::try_into(hdr_ext_len)? * 8 + 6;
        packet_reader.read(read_len)?;
        // TODO read options

        Ok(Ipv6HopByHopOptions {
            next_header: bytes[0],
            hdr_ext_len,
            options: vec![]
        })
    }
}

pub struct Ipv6Fragment {
    pub next_header: u8,
    pub reserved: u8,
    pub fragment_offset: u16,
    pub res: u8,
    pub m_flag: bool,
    pub identification: u32
}

impl Ipv6Fragment {
    pub const PROTOCOL_NUMBER: u8 = 44;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(8)?;

        Ok(Ipv6Fragment {
            next_header: bytes[0],
            reserved: bytes[1],
            fragment_offset: (bytes[0] as u16) << 5 | (bytes[1] as u16 >> 3),
            res: (bytes[1] & 0x06) >> 1,
            m_flag: bytes[3] & 0x01 == 1,
            identification: u32::from_be_bytes(bytes[4..8].try_into()?)
        })
    }
}

pub struct Ipv6DestinationOptions {
    pub next_header: u8,
    pub hdr_ext_len: u8,
    pub options: Vec<Ipv6ExtensionOptions>
}

impl Ipv6DestinationOptions {
    pub const PROTOCOL_NUMBER: u8 = 60;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(2)?;
        let hdr_ext_len = bytes[1];
        let read_len = TryInto::<usize>::try_into(hdr_ext_len)? * 8 + 6;
        packet_reader.read(read_len)?;
        // TODO read data

        Ok(Ipv6DestinationOptions {
            next_header: bytes[0],
            hdr_ext_len,
            options: vec![]
        })
    }
}

pub struct Ipv6Routing {
    pub next_header: u8,
    pub hdr_ext_len: u8,
    pub routing_type: u8,
    pub segments_left: u8,
    pub type_specific_data: Vec<u8>
}

impl Ipv6Routing {
    pub const PROTOCOL_NUMBER: u8 = 43;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(4)?;
        let hdr_ext_len = bytes[1];
        let read_len = TryInto::<usize>::try_into(hdr_ext_len)? * 8 + 4;
        packet_reader.read(read_len)?;
        // TODO read data

        Ok(Ipv6Routing {
            next_header: bytes[0],
            hdr_ext_len,
            routing_type: bytes[2],
            segments_left: bytes[3],
            type_specific_data: vec![]
        })
    }
}

pub struct Ipv6Authentication {
    pub next_header: u8,
    pub payload_len: u8,
    pub reserved: u16,
    pub spi: u32,
    pub seq_num: u32,
    pub icv: Vec<u8>
}

impl Ipv6Authentication {
    pub const PROTOCOL_NUMBER: u8 = 51;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(12)?;
        let payload_len = bytes[1];
        let read_len = (TryInto::<usize>::try_into(payload_len)? - 1) * 4;

        Ok(Ipv6Authentication {
            next_header: bytes[0],
            payload_len,
            reserved: u16::from_be_bytes(bytes[2..4].try_into()?),
            spi: u32::from_be_bytes(bytes[4..8].try_into()?),
            seq_num: u32::from_be_bytes(bytes[8..12].try_into()?),
            icv: packet_reader.read(read_len)?.to_vec()
        })
    }
}

pub struct Ipv6EncapsulatingSecurityPayload {
}

impl Ipv6EncapsulatingSecurityPayload {
    pub const PROTOCOL_NUMBER: u8 = 50;

    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        todo!()
    }
}

pub struct Ipv6ExtensionOptions {
    pub option_type: u8,
    pub opt_data_len: u8,
    pub option_data: Vec<u8>
}

impl Ipv6ExtensionOptions {
    pub(in crate::library::common::network::link::internet) fn new<'a, 'b: 'a>(packet_reader: &'a mut PacketReader<'b>) -> Result<Self, ReadError> {
        let bytes = packet_reader.read(16)?;
        let opt_data_len = bytes[1];

        Ok(Ipv6ExtensionOptions {
            option_type: bytes[0],
            opt_data_len,
            option_data: packet_reader.read(TryInto::<usize>::try_into(opt_data_len)? / 8)?.to_vec()
        })
    }
}