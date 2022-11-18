use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crossterm::event::{Event, KeyCode, read};

use crate::app::AppState;
use crate::app::ui::views::View;

pub struct EventHandler {
    app_state: Arc<Mutex<AppState>>
}

impl EventHandler {
    pub fn new(app_state: Arc<Mutex<AppState>>) -> EventHandler {
        EventHandler {
            app_state
        }
    }

    pub fn run(&self) -> JoinHandle<()> {
        let app_state = self.app_state.clone();
        thread::spawn(move || {
            while app_state.lock().unwrap().running {
                if let Ok(event) = read() {
                    match event {
                        Event::Key(key_event) => {
                            if let KeyCode::Char(key) = key_event.code {
                                match key {
                                    'q' => {
                                        let mut lock = app_state.lock().unwrap();
                                        lock.running = false;
                                    }
                                    _ => {
                                        delegate_event(&app_state, event);
                                    }
                                }
                            } else {
                                delegate_event(&app_state, event);
                            }
                        }
                        _ => {
                            delegate_event(&app_state, event);
                        }
                    }
                } else {
                    break;
                }
            }
        })
    }
}

fn delegate_event(app_state: &Arc<Mutex<AppState>>, event: Event) {
    let mut lock = app_state.lock().unwrap();
    if let Some(view) = lock.view.handle_event(event) {
        lock.view = view;
    }
}