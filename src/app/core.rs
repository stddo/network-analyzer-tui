use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use network_analyzer::app::LocalProcess;
use network_analyzer::network::ethernet2::Ethernet2Frame;
use network_analyzer::network::link::internet::IPHeader;
use network_analyzer::network::link::internet::transport::TransportHeader;
use network_analyzer::pcap::Sniffer;

use crate::app::core::multithreading::RwList;

mod multithreading;

pub struct PacketRetriever {
    running: Arc<Mutex<bool>>,
    join_handle: Option<JoinHandle<()>>,
    process: Arc<LocalProcess>,
    pub packets: Arc<RwList<Ethernet2Frame>>
}

impl Drop for PacketRetriever {
    fn drop(&mut self) {
        *self.running.lock().unwrap() = false;
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().unwrap();
        }
    }
}

impl PacketRetriever {
    pub fn new(process: LocalProcess) -> PacketRetriever {
        PacketRetriever {
            running: Arc::new(Mutex::new(true)),
            join_handle: None,
            process: Arc::new(process),
            packets: Arc::new(RwList::new())
        }
    }

    pub fn run(&mut self) {
        let packets = self.packets.clone();
        let running = self.running.clone();
        let process = self.process.clone();
        self.join_handle = Some(thread::spawn(move || {
            let sniffer = Sniffer::new();
            sniffer.sniff(|packet| {
                let packet = match &packet.payload.header {
                    IPHeader::V4Header(_) => {
                        match &packet.payload.payload.header {
                            TransportHeader::TCP(tcp) => {
                                if tcp.dst_port == process.local_port || tcp.dst_port == process.remote_port || tcp.src_port == process.local_port || tcp.src_port == process.remote_port {
                                    Some(packet)
                                } else { None }
                            }
                            _ => { None }
                        }
                    }
                    IPHeader::V6Header(_) => { None }
                };

                if let Some(packet) = packet {
                    packets.add(packet);
                }
                return !*running.lock().unwrap();
            });
        }));
    }
}