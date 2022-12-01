use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use network_analyzer::app::App;
use network_analyzer::network::link::internet::transport::TransportHeader;
use network_analyzer::network::packet::Packet;
use network_analyzer::pcap::Sniffer;

use crate::app::core::multithreading::RwList;

mod multithreading;

pub struct PacketRetriever {
    running: Arc<Mutex<bool>>,
    join_handle: Option<JoinHandle<()>>,
    app: Arc<App>,
    pub packets: Arc<RwList<Packet>>
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
    pub fn new(app: App) -> PacketRetriever {
        PacketRetriever {
            running: Arc::new(Mutex::new(true)),
            join_handle: None,
            app: Arc::new(app),
            packets: Arc::new(RwList::new())
        }
    }

    pub fn run(&mut self) {
        let packets = self.packets.clone();
        let running = self.running.clone();
        let app = self.app.clone();
        self.join_handle = Some(thread::spawn(move || {
            let sniffer = Sniffer::new();
            sniffer.sniff(|packet| {
                let packet = match &packet.tp_header {
                    TransportHeader::TCP(tcp) => {
                        if app.processes.iter().any(|process| {
                            tcp.dst_port == process.local_port || tcp.dst_port == process.remote_port || tcp.src_port == process.local_port || tcp.src_port == process.remote_port
                        }) {
                            Some(packet)
                        } else { None }
                    }
                    _ => { None }
                };

                if let Some(packet) = packet {
                    packets.add(packet);
                }
                return !*running.lock().unwrap();
            });
        }));
    }
}