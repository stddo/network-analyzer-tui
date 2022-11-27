use tcp::TCPHeader;
use udp::UDPHeader;

use crate::common::network::ReadError;

pub mod tcp;
pub mod udp;

pub enum TransportHeader {
    TCP(TCPHeader),
    UDP(UDPHeader),
    Default(Vec<u8>)
}

impl TransportHeader {
    pub fn len(&self) -> usize {
        0
    }
}

impl TransportHeader {
    pub fn new(protocol: u8, bytes: &[u8]) -> Result<TransportHeader, ReadError> {
        Ok(match protocol {
            6 => TransportHeader::TCP(TCPHeader::new(bytes)?),
            17 => TransportHeader::UDP(UDPHeader::new(bytes)?),
            _ => TransportHeader::Default(bytes.to_vec())
        })
    }
}

pub struct TransportPayload(pub Vec<u8>);