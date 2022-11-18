use std::collections::HashMap;

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[derive(Clone)]
pub struct LocalProcess {
    pub local_ip: u32,
    pub local_port: u16,
    pub remote_ip: u32,
    pub remote_port: u16,
    pub pid: u32,
    pub name: String
}

pub fn pid_by_name() -> HashMap<u32, String> {
    let mut system = System::new();
    system.refresh_all();

    let ps = system.processes();
    let mut result = HashMap::new();
    for (pid, process) in ps {
        result.insert(pid.as_u32(), String::from(process.name()));
    }
    return result;
}