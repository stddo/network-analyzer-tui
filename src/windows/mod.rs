use crate::common::app::{LocalProcess, pid_by_name};
use crate::windows::tcp_table::collect_open_ports_by_app;

mod tcp_table;

pub fn get_apps() -> Vec<LocalProcess> {
    let names = pid_by_name();
    let mut apps = collect_open_ports_by_app().unwrap();
    apps.iter_mut().for_each(|app| {
        let name = names.get(&app.pid);
        if let Some(name) = name {
            app.name = name.clone();
        }
    });
    apps.into_iter().filter(|app| {
        app.pid != 0 && app.pid != 4
    }).collect()
}