use std::collections::HashMap;

use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

#[cfg(target_os = "windows")]
use crate::library::windows::collect_open_ports_by_app;

#[derive(Clone)]
pub struct App {
    pub name: String,
    pub processes: Vec<LocalProcess>
}

impl App {
    pub fn all_by_pids() -> HashMap<u32, App> {
        let mut apps: HashMap<u32, App> = HashMap::new();
        let processes_info = processes_info();
        let open_processes = collect_open_ports_by_app().unwrap();
        open_processes.into_iter().for_each(|process| {
            if process.pid == 0 || process.pid == 4 { return; }

            let process_info = processes_info.get(&Pid::from_u32(process.pid));
            if let Some((name, parent_pid)) = process_info {
                let parent_pid = if let Some(parent_pid) = parent_pid { parent_pid.as_u32() } else { process.pid };

                if let Some(parent) = apps.get_mut(&parent_pid) {
                    parent.processes.push(process);
                } else {
                    apps.insert(parent_pid, App {
                        name: name.clone(),
                        processes: vec![process],
                    });
                }
            }
        });
        apps
    }
}

#[derive(Clone)]
pub struct LocalProcess {
    pub local_port: u16,
    pub pid: u32
}

pub fn processes_info() -> HashMap<Pid, (String, Option<Pid>)> {
    let mut system = System::new();
    system.refresh_all();

    let ps = system.processes();

    let mut result = HashMap::new();
    for (pid, process) in ps {
        result.insert(pid.clone(), (String::from(process.name()), process.parent()));
    }
    return result;
}