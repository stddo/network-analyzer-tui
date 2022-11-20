use std::io::Stdout;
use std::sync::Arc;

use crossterm::event::{Event, KeyCode};
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};

use network_analyzer::app::LocalProcess;
use network_analyzer::get_apps;
use network_analyzer::network::link::internet::IPHeader;
use network_analyzer::network::link::internet::transport::TransportHeader;

use crate::app::AppState;
use crate::app::core::PacketRetriever;

pub trait View {
    fn handle_event(&mut self, event: Event, app_state: Arc<AppState>) -> Option<Box<dyn View + Send>>;

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_state: Arc<AppState>);
}

#[derive(Default)]
pub struct AppsTableView {
    items: Vec<LocalProcess>,
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
                            if let Some(process) = self.items.get(selected) {
                                let mut lock = app_state.packet_retriever.lock().unwrap();
                                let mut packet_retriever = PacketRetriever::new(process.clone());
                                packet_retriever.run();
                                *lock = Some(packet_retriever);
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

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, _: Arc<AppState>) {
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
                Cell::from(item.name.clone()),
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
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        AppsTableView {
            items: get_apps(),
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

    pub fn items(&mut self, items: Vec<LocalProcess>) {
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
    process: LocalProcess,
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

        let rows = if let Some(ref packet_retriever) = *app_state.packet_retriever.lock().unwrap() {
            let packet_retriever = packet_retriever.packets.clone();
            let packets = packet_retriever.lock().unwrap();

            packets.iter().filter_map(|packet| {
                match &packet.payload.header {
                    IPHeader::V4Header(v4) => {
                        match &packet.payload.payload.header {
                            TransportHeader::TCP(tcp) => {
                                Some(Row::new([
                                    Cell::from(format_ipv4_address(v4.src_addr)),
                                    Cell::from(tcp.src_port.to_string()),
                                    Cell::from(format_ipv4_address(v4.dst_addr)),
                                    Cell::from(tcp.dst_port.to_string()),
                                    Cell::from("TCP")
                                ]))
                            }
                            TransportHeader::UDP(_) => {
                                Some(Row::new([
                                    Cell::from(format_ipv4_address(v4.src_addr)),
                                    Cell::from("none"),
                                    Cell::from(format_ipv4_address(v4.dst_addr)),
                                    Cell::from("none"),
                                    Cell::from("TCP")
                                ]))
                            }
                            TransportHeader::Default(_) => {
                                None
                            }
                        }
                    }
                    IPHeader::V6Header(_) => {
                        None
                    }
                }
            }).collect()
        } else {
            vec![]
        };

        let t = Table::new(rows)
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

fn format_ipv4_address(address: u32) -> String {
    format!("{}.{}.{}.{}", (address >> 24) as u8, (address >> 16) as u8, (address >> 8) as u8, address as u8)
}