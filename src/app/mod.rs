use std::io;
use std::sync::{Arc, Mutex};

use ui::event::EventHandler;
use ui::views::WelcomeScreen;
use ui::Window;

use crate::app::ui::views::View;

mod core;
mod ui;

pub struct App {
    window: Window,
    event_handler: EventHandler
}

impl App {
    pub fn new() -> App {
        let state = Arc::new(AppState::new());
        App {
            window: Window::new(state.clone()),
            event_handler: EventHandler::new(state.clone())
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        let window = self.window.run();
        let event_handler = self.event_handler.run();
        window.join().unwrap();
        event_handler.join().unwrap();
        Ok(())
    }
}

pub struct AppState {
    running: Mutex<bool>,
    view: Mutex<Box<dyn View + Send>>
}

impl AppState {
    fn new() -> AppState {
        AppState {
            running: Mutex::new(true),
            view: Mutex::new(Box::new(WelcomeScreen::default()))
        }
    }

    fn stop(&self) {
        let mut lock = self.running.lock().unwrap();
        *lock = false;
    }
}
