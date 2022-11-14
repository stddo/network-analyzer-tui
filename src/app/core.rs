use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::app::AppState;

struct Sniffer {
    app_state: Arc<Mutex<AppState>>
}

impl Sniffer {
    pub fn new(app_state: Arc<Mutex<AppState>>) -> Sniffer {
        Sniffer {
            app_state
        }
    }

    pub fn run() -> JoinHandle<()> {
        thread::spawn(|| {

        })
    }
}