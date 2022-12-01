use std::collections::HashMap;
use std::io::Stdout;
use std::sync::Arc;

use crossterm::event::{Event, KeyCode};
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};

use network_analyzer::app::App;
use network_analyzer::network::link::internet::transport::TransportHeader;

use crate::app::AppState;
use crate::app::core::PacketRetriever;

pub trait View {
    fn handle_event(&mut self, event: Event, app_state: Arc<AppState>) -> Option<Box<dyn View + Send>>;

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_state: Arc<AppState>);
}

#[derive(Default)]
pub struct AppsTableView {
    items: HashMap<u32, App>,
    pub table_state: TableState,
}

impl View for AppsTableView {
    fn handle_event(&mut self, event: Event, app_state: Arc<AppState>) -> Option<Box<dyn View + Send>> {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char(key) => {
                        match key {
                            'l' => {
                                self.items(App::all_by_pids());
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
                            if let Some(app) = self.items.values().nth(selected) {
                                let mut lock = app_state.packet_retriever.lock().unwrap();
                                let mut packet_retriever = PacketRetriever::new(app.clone());
                                packet_retriever.run();
                                *lock = Some(packet_retriever);
                                return Some(Box::new(ProcessPacketsView::new(app.clone())));
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

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, _: Arc<AppState>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["pid", "name", "remote port", "local port"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = self.items.iter().map(|(pid, app)| {
            let cells = vec![
                Cell::from(pid.to_string()),
                Cell::from(app.name.clone()),
                Cell::from(app.processes.iter()
                    .map(|process| { process.remote_port.to_string() })
                    .reduce(|a, b| format!("{}, {}", a, b)).unwrap_or(String::new())),
                Cell::from(app.processes.iter()
                    .map(|process| { process.local_port.to_string() })
                    .reduce(|a, b| format!("{}, {}", a, b)).unwrap_or(String::new())),
            ];
            Row::new(cells).height(2).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]);
        f.render_stateful_widget(t, rects[0], &mut self.table_state);
    }
}

impl AppsTableView {
    pub fn new() -> AppsTableView {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        AppsTableView {
            items: App::all_by_pids(),
            table_state,
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

    pub fn items(&mut self, items: HashMap<u32, App>) {
        self.items = items;
        self.table_state.select(Some(0))
    }
}

#[derive(Default)]
pub struct WelcomeScreen {}

impl View for WelcomeScreen {
    fn handle_event(&mut self, event: Event, _: Arc<AppState>) -> Option<Box<dyn View + Send>> {
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

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, _: Arc<AppState>) {
        let text = vec![
            Spans::from(Span::raw("Legend:")),
            Spans::from(Span::raw("l - show list of apps with open ports")),
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
    app: App,
    table_state: TableState
}

impl View for ProcessPacketsView {
    fn handle_event(&mut self, event: Event, _: Arc<AppState>) -> Option<Box<dyn View + Send>> {
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

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_state: Arc<AppState>) {
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

        let mut rows = vec![];
        if let Some(ref packet_retriever) = *app_state.packet_retriever.lock().unwrap() {
            let packets = packet_retriever.packets.clone();
            let mut i = 0;
            let y = f.size().height - 2;
            for packet in &*packets {
                if i == y {
                    break;
                }

                rows.push(Row::new([
                    Cell::from(packet.ip_header.formatted_src_ip()),
                    Cell::from(packet.tp_header.src_port().to_string()),
                    Cell::from(packet.ip_header.formatted_src_ip()),
                    Cell::from(packet.tp_header.dst_port().to_string()),
                    match &packet.tp_header {
                        TransportHeader::TCP(_) => Cell::from("TCP"),
                        TransportHeader::UDP(_) => Cell::from("UDP"),
                        TransportHeader::Default(_) => Cell::from("UNKNOWN")
                    }
                ]));

                i += 1;
            }
        }

        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(format!("{}", self.app.name)))
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
    pub fn new(app: App) -> ProcessPacketsView {
        ProcessPacketsView {
            app,
            table_state: Default::default()
        }
    }
}