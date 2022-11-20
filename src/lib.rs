#[cfg(target_os = "linux")]
pub use crate::linux::run;
#[cfg(target_os = "windows")]
pub use crate::windows::get_apps;

mod common;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;


pub mod network {
    pub use crate::common::network::*;
}

pub mod pcap {
    pub use crate::common::pcap::*;
}

pub mod app {
    pub use crate::common::app::*;
}