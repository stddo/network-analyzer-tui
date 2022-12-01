use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crossterm::event::{Event, KeyCode, read};

use crate::app::AppState;

pub struct EventHandler {
    app_state: Arc<AppState>
}

impl EventHandler {
    pub fn new(app_state: Arc<AppState>) -> EventHandler {
        EventHandler {
            app_state
        }
    }

    pub fn run(&self) -> JoinHandle<()> {
        let app_state = self.app_state.clone();
        thread::spawn(move || {
            while *app_state.running.lock().unwrap() {
                if let Ok(event) = read() {
                    match event {
                        Event::Key(key_event) => {
                            if let KeyCode::Char(key) = key_event.code {
                                match key {
                                    'q' => {
                                        app_state.stop();
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

fn delegate_event(app_state: &Arc<AppState>, event: Event) {
    let mut lock = app_state.view.lock().unwrap();
    if let Some(view) = lock.handle_event(event) {
        *lock = view;
    }
}