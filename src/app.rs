use std::io;
use std::sync::{Arc, Mutex};

use event::EventHandler;
use ui::Window;

use crate::app::ui::views::apps_table::AppsTableView;
use crate::app::ui::views::View;

mod core;
mod ui;
mod event;

pub struct App {
    window: Window,
    event_handler: EventHandler,
    state: Arc<Mutex<AppState>>
}

impl App {
    pub fn new() -> App {
        let state = Arc::new(Mutex::new(AppState::new()));
        App {
            window: Window::new(state.clone()),
            event_handler: EventHandler::new(state.clone()),
            state
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
    running: bool,
    view: Box<dyn View + Send>
}

impl AppState {
    fn new() -> AppState {
        AppState {
            running: true,
            view: Box::new(AppsTableView::default())
        }
    }

    fn stop(&mut self) {
        self.running = false
    }
}
