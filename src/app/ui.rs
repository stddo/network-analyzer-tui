use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::AppState;

pub mod views;

pub struct Window {
    app_state: Arc<Mutex<AppState>>
}

impl Window {
    pub fn new(app_state: Arc<Mutex<AppState>>) -> Window {
        Window {
            app_state
        }
    }

    pub fn run(&self) -> JoinHandle<()> {
        let app_state = self.app_state.clone();
        thread::spawn(move || {
            enable_raw_mode().unwrap();
            let mut stdout = stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend).unwrap();

            let mut last_time = Instant::now();

            while app_state.lock().unwrap().running {
                terminal.draw(|f| {
                    let mut lock = app_state.lock().unwrap();
                    lock.view.draw(f);
                }).unwrap();

                let delay = Duration::from_millis(100).saturating_sub(last_time.elapsed());
                thread::sleep(delay);
                last_time = Instant::now();
            }

            disable_raw_mode().unwrap();
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            ).unwrap();
            terminal.show_cursor().unwrap();
        })
    }
}
