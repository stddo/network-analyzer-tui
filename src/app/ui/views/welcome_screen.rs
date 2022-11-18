use std::io::Stdout;
use crossterm::event::{Event, KeyCode};
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Layout};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use crate::app::ui::views::apps_table::AppsTableView;
use crate::app::ui::views::View;

#[derive(Default)]
pub struct WelcomeScreen {}

impl View for WelcomeScreen {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>> {
        match event {
            Event::Key(key_event) => {
                if let KeyCode::Char(key) = key_event.code {
                    match key {
                        'l' => {
                            return Some(Box::new(AppsTableView::new()));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        None
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let text = vec![
            Spans::from(Span::raw("Legend:")),
            Spans::from(Span::raw("l - show list of apps with open ports"))
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::raw("Welcome")));

        f.render_widget(paragraph, Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(f.size())[0]);
    }
}