use std::net::{IpAddr, Ipv4Addr};
use pcap::{Capture, Device, Error};

use crate::common::network::link::Packet;

pub struct Sniffer {
    device: Device
}

impl Sniffer {
    pub fn new() -> Sniffer {
        let device = {
            let devices = Device::list().unwrap();
            devices.iter().find(|device| {
                device.addresses.iter().any(|addr| if let IpAddr::V4(addr) = addr.addr {
                    addr == Ipv4Addr::new(192, 168, 178, 20)
                } else { false })
            }).unwrap().clone()
        };

        Sniffer {
            device
        }
    }

    pub fn sniff(&self, f: impl Fn(Packet) -> bool) {
        let mut cap = Capture::from_device(self.device.clone()).unwrap()
            .timeout(0).open().unwrap();
        //cap.filter("internet and udp", false);

        loop {
            match cap.next() {
                Ok(packet) => {
                    let frame = Packet::from_ethernet_bytes(packet.data);
                    if let Ok(frame) = frame {
                        if f(frame) {
                            break;
                        }
                    }
                }
                Err(e) => {
                    if let Error::TimeoutExpired = e {
                    } else {
                        println!("{:?}", e);
                    }
                }
            }
        }
    }
}