use crate::common::network::link::internet::IPHeader::{V4Header, V6Header};
use crate::common::network::link::internet::ipv4::IPv4Header;
use crate::common::network::link::internet::ipv6::IPv6Header;
use crate::common::network::link::internet::transport::{TransportHeader, TransportPayload};
use crate::common::network::ReadError;

pub mod transport;
pub mod ipv4;
pub mod ipv6;

pub struct IPFrame {
    pub header: IPHeader,
    pub payload: TransportFrame,
}

impl IPFrame {
    pub fn new(bytes: &[u8]) -> Result<IPFrame, ReadError> {
        let header = IPHeader::new(bytes)?;
        let protocol = header.protocol();
        let header_len = header.len();

        Ok(IPFrame {
            header,
            payload: TransportFrame::new(protocol, &bytes[header_len..])?
        })
    }
}

pub enum IPHeader {
    V4Header(IPv4Header),
    V6Header(IPv6Header),
}

impl IPHeader {
    pub fn new(bytes: &[u8]) -> Result<IPHeader, ReadError> {
        let version = bytes[0] >> 4;
        match version {
            4 => Ok(V4Header(IPv4Header::new(bytes)?)),
            6 => Ok(V6Header(IPv6Header::new(bytes))),
            v => Err(ReadError::IPUnexpectedVersion(v))
        }
    }

    pub fn len(&self) -> usize {
        match self {
            V4Header(header) => header.len(),
            V6Header(_) => 40
        }
    }

    pub fn protocol(&self) -> u8 {
        match self {
            V4Header(header) => header.protocol,
            V6Header(header) => 0/*header.next_header*/
        }
    }
}

pub struct TransportFrame {
    pub header: TransportHeader,
    pub payload: TransportPayload,
}

impl TransportFrame {
    fn new(protocol: u8, bytes: &[u8]) -> Result<TransportFrame, ReadError> {
        let header = TransportHeader::new(protocol, bytes)?;
        let header_len = header.len();
        Ok(TransportFrame {
            header,
            payload: TransportPayload(bytes[header_len..].to_vec())
        })
    }
}