use std::net::{Ipv4Addr, Ipv6Addr};

use crate::library::common::network::link::internet::IpHeader::{V4Header, V6Header};
use crate::library::common::network::link::internet::ipv4::Ipv4Header;
use crate::library::common::network::link::internet::ipv6::Ipv6Header;
use crate::library::common::network::ReadError;
use crate::library::common::network::packet::PacketReader;

pub mod transport;
pub mod ipv4;
pub mod ipv6;

pub enum IpHeader {
    V4Header(Ipv4Header),
    V6Header(Ipv6Header),
}

impl IpHeader {
    pub fn new<'a, 'b: 'a>(version: u8, packet_reader: &'a mut PacketReader<'b>) -> Result<IpHeader, ReadError> {
        match version {
            4 => Ok(V4Header(Ipv4Header::new(packet_reader)?)),
            6 => Ok(V6Header(Ipv6Header::new(packet_reader)?)),
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
            V6Header(header) => header.next_header
        }
    }

    pub fn formatted_src_ip(&self) -> String {
        match self {
            V4Header(header) => {
                Ipv4Addr::from(header.src_addr).to_string()
            }
            V6Header(header) => {
                Ipv6Addr::from(header.src_addr).to_string()
            }
        }
    }

    pub fn formatted_dst_ip(&self) -> String {
        match self {
            V4Header(header) => {
                Ipv4Addr::from(header.dst_addr).to_string()
            }
            V6Header(header) => {
                Ipv6Addr::from(header.dst_addr).to_string()
            }
        }
    }
}