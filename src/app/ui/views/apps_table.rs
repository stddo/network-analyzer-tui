use std::collections::HashMap;
use std::io::Stdout;

use crossterm::event::{Event, KeyCode};
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};

use network_analyzer::{get_apps, LocalProcess};

use crate::app::ui::views::View;

#[derive(Default)]
pub struct AppsTableView {
    items: HashMap<u32, LocalProcess>,
    pub table_state: TableState
}

impl View for AppsTableView {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>> {
        match event {
            Event::Key(key_event) => {
                if let KeyCode::Char(key) = key_event.code {
                    match key {
                        'l' => {
                            let a = get_apps();
                            self.items(a);
                        }
                        'n' => {
                            self.next();
                        }
                        'p' => {
                            self.prev();
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
