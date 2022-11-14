use std::collections::HashMap;

use crate::common::app::{LocalProcess, pid_by_name};
use crate::windows::tcp_table::collect_open_ports_by_app;

mod tcp_table;

pub fn get_apps() -> HashMap<u32, LocalProcess> {
    let names = pid_by_name();
    let mut apps = collect_open_ports_by_app().unwrap();
    apps.iter_mut().filter_map(|(pid, app)| {
        let name = names.get(pid);
        return if let Some(name) = name {
            app.name = name.clone();
            Some(app)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    apps
}