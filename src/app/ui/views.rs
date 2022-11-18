use std::io::Stdout;

use crossterm::event::Event;
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;

pub mod apps_table;
pub mod welcome_screen;

pub trait View {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>>;

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
}