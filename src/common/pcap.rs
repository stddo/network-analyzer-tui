use pcap::{Capture, Device, Error};

use crate::common::network::ethernet2::Ethernet2Frame;
use crate::common::network::link::from_ethernet_bytes;

pub struct Sniffer {
    device: Device
}

impl Sniffer {
    pub fn new() -> Sniffer {
        Sniffer {
            device: Device::list().unwrap()[8].clone()
        }
    }

    pub fn sniff(&self, f: impl Fn(Ethernet2Frame) -> bool) {
        let mut cap = Capture::from_device(self.device.clone()).unwrap()
            .timeout(0).open().unwrap();
        //cap.filter("internet and udp", false);

        loop {
            match cap.next() {
                Ok(packet) => {
                    let frame = from_ethernet_bytes(packet.data);
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