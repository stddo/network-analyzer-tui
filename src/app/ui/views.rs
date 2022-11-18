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
    items: Vec<LocalProcess>,
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
                                 self.items(get_apps());
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
                     KeyCode::Enter => {
                         if let Some(selected) = self.table_state.selected() {
                             let a = &(selected as u32);
                             if let Some(process) = self.items.get(selected) {
                                 return Some(Box::new(ProcessPacketsView::new(process.clone())));
                             }
                         }
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
                Cell::from(item.pid.to_string()),
                Cell::from(item.name.clone())
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

    pub fn items(&mut self, items: Vec<LocalProcess>) {
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

pub struct ProcessPacketsView {
    process: LocalProcess,
    table_state: TableState
}

impl View for ProcessPacketsView {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn View + Send>> {
        match event {
            Event::Key(key_event) => {
                if let KeyCode::Backspace = key_event.code {
                    return Some(Box::new(AppsTableView::new()));
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
        let header_cells = ["src ip", "src port", "dst ip", "dst port", "protocol"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let t = Table::new([])
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(format!("{} - {}", self.process.pid, self.process.name)))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]);
        f.render_stateful_widget(t, rects[0], &mut self.table_state);
    }
}

impl ProcessPacketsView {
    pub fn new(process: LocalProcess) -> ProcessPacketsView {
        ProcessPacketsView {
            process,
            table_state: Default::default()
        }
    }
}