use std::collections::HashMap;
use std::io::Stdout;

use crossterm::event::{Event, KeyCode};
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};

use network_analyzer::{get_apps, LocalProcess};

pub trait View {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>>;

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
}

#[derive(Default)]
pub struct AppsTableView {
    items: HashMap<u32, LocalProcess>,
    pub table_state: TableState
}

impl View for AppsTableView {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>> {
        match event {
            Event::Key(key_event) => {
                 match key_event.code {
                     KeyCode::Char(key) => {
                         match key {
                             'l' => {
                                 let a = get_apps();
                                 self.items(a);
                             }
                             _ => {}
                         }
                     }
                     KeyCode::Up => {
                         self.prev();
                     }
                     KeyCode::Down => {
                         self.next();
                     }
                     _ => {}
                }
            }
            _ => {}
        }

        None
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["pid", "name"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = self.items.iter().map(|item| {
            let cells = vec![
                Cell::from(item.0.to_string()),
                Cell::from(item.1.name.clone())
            ];
            Row::new(cells).height(2).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]);
        f.render_stateful_widget(t, rects[0], &mut self.table_state);
    }
}

impl AppsTableView {
    pub fn new() -> AppsTableView {
        AppsTableView {
            items: get_apps(),
            table_state: Default::default()
        }
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn items(&mut self, items: HashMap<u32, LocalProcess>) {
        self.items = items;
        self.table_state.select(Some(0))
    }
}

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
