use tcp::TCPHeader;
use udp::UDPHeader;

use crate::common::network::ReadError;
use crate::network::link::PacketReader;

pub mod application;
pub mod tcp;
pub mod udp;

pub enum TransportHeader {
    TCP(TCPHeader),
    UDP(UDPHeader),
    Default(Vec<u8>)
}

impl TransportHeader {
    pub fn new<'a, 'b: 'a>(protocol: u8, packet_reader: &'a mut PacketReader<'b>) -> Result<TransportHeader, ReadError> {
        Ok(match protocol {
            6 => TransportHeader::TCP(TCPHeader::new(packet_reader)?),
            17 => TransportHeader::UDP(UDPHeader::new(packet_reader)?),
            _ => TransportHeader::Default(vec![])
        })
    }

    pub fn src_port(&self) -> u16 {
        match self {
            TransportHeader::TCP(tcp) => tcp.src_port,
            TransportHeader::UDP(udp) => udp.src_port,
            TransportHeader::Default(_) => 0
        }
    }

    pub fn dst_port(&self) -> u16 {
        match self {
            TransportHeader::TCP(tcp) => tcp.dst_port,
            TransportHeader::UDP(udp) => udp.dst_port,
            TransportHeader::Default(_) => 0
        }
    }
}

pub struct TransportPayload(pub Vec<u8>);