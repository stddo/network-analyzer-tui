use std::net::{Ipv4Addr, Ipv6Addr};

use crate::library::common::network::link::internet::IpHeader::{V4Header, V6Header};
use crate::library::common::network::link::internet::ipv4::Ipv4Header;
use crate::library::common::network::link::internet::ipv6::Ipv6Header;
use crate::library::common::network::packet::PacketReader;
use crate::library::common::network::ReadError;
use crate::network::link::internet::ipv6::{Ipv6Authentication, Ipv6DestinationOptions, Ipv6EncapsulatingSecurityPayload, Ipv6Fragment, Ipv6HopByHopOptions, Ipv6Routing};

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

pub enum IpExtension {
    Ipv4Extension(Ipv4Extension),
    Ipv6Extension(Ipv6Extension)
}

impl IpExtension {
    pub fn new<'a, 'b: 'a>(next_header: u8, packet_reader: &'a mut PacketReader<'b>) -> Result<IpExtension, ReadError> {
        match next_header {
            Ipv6HopByHopOptions::PROTOCOL_NUMBER => Ok(IpExtension::Ipv6Extension(Ipv6Extension::HopByHopOptions(Ipv6HopByHopOptions::new(packet_reader)?))),
            Ipv6Routing::PROTOCOL_NUMBER => Ok(IpExtension::Ipv6Extension(Ipv6Extension::Routing(Ipv6Routing::new(packet_reader)?))),
            Ipv6Fragment::PROTOCOL_NUMBER => Ok(IpExtension::Ipv6Extension(Ipv6Extension::Fragment(Ipv6Fragment::new(packet_reader)?))),
            Ipv6DestinationOptions::PROTOCOL_NUMBER => Ok(IpExtension::Ipv6Extension(Ipv6Extension::DestinationOptions(Ipv6DestinationOptions::new(packet_reader)?))),
            Ipv6Authentication::PROTOCOL_NUMBER => Ok(IpExtension::Ipv6Extension(Ipv6Extension::Authentication(Ipv6Authentication::new(packet_reader)?))),
            _ => Err(ReadError::UnsupportedIpExtension)
        }
    }

    pub fn list<'a, 'b: 'a>(mut next_header: u8, packet_reader: &'a mut PacketReader<'b>) -> Result<(u8, Vec<IpExtension>), ReadError> {
        let mut res = vec![];
        loop {
            if let Ok(extension) = IpExtension::new(next_header, packet_reader) {
                next_header = extension.next_header();
                res.push(extension);
            } else {
                break;
            }
        }
        Ok((next_header, res))
    }

    fn next_header(&self) -> u8 {
        match self {
            IpExtension::Ipv4Extension(_) => {
                panic!("Unsupported extension");
            }
            IpExtension::Ipv6Extension(extension) => {
                match extension {
                    Ipv6Extension::HopByHopOptions(e) => e.next_header,
                    Ipv6Extension::Fragment(e) => e.next_header,
                    Ipv6Extension::DestinationOptions(e) => e.next_header,
                    Ipv6Extension::Routing(e) => e.next_header,
                    Ipv6Extension::Authentication(e) => e.next_header,
                    Ipv6Extension::EncapsulatingSecurityPayload(_) => panic!("Unsupported extension")
                }
            }
        }
    }
}

pub enum Ipv4Extension {}

pub enum Ipv6Extension {
    HopByHopOptions(Ipv6HopByHopOptions),
    Fragment(Ipv6Fragment),
    DestinationOptions(Ipv6DestinationOptions),
    Routing(Ipv6Routing),
    Authentication(Ipv6Authentication),
    EncapsulatingSecurityPayload(Ipv6EncapsulatingSecurityPayload)
}