#[allow(dead_code)]
mod util;
extern crate serialport;

use crate::util::{
    event::{Event, Events},
    TabsState,
    StatefulList,
    AsVec,
};
use std::{error::Error, io};
use std::fmt::{self, Debug, Display};
use termion::{
    raw::IntoRawMode,
    event::Key,
    input::MouseTerminal,
    screen::AlternateScreen
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
    Terminal,
};
use serialport::SerialPortInfo;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Tab {
    Welcome,
    Monitor,
    Console,
    Settings,
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl AsVec<Tab> for Tab {
    fn as_vec() -> Vec<Tab> {
        vec![Tab::Welcome, Tab::Monitor, Tab::Console, Tab::Settings]
    }
}

struct App {
    tabs: TabsState<Tab>,
    serialPorts: StatefulList<SerialPortInfo>,
}


fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let ports = serialport::available_ports().expect("No ports found!");

    // App
    let mut app = App {
        tabs: TabsState::new(Tab::Welcome),
        serialPorts: StatefulList::with_items(ports),
    };

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
            f.render_widget(block, size);
            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    Spans::from(Span::styled(t.to_string(), Style::default().fg(Color::Yellow)))
                })
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);
            let inner = match app.tabs.index {
                0 => {
                    let block = Block::default().title("Welcome").borders(Borders::ALL);
                    let items: Vec<ListItem> = app.serialPorts.items.iter()
                        .map(|i| {
                            let mut lines = vec![Spans::from(i.port_name.clone())];
                            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                        })
                        .collect();

                    let items = List::new(items)
                        .block(block.clone())
                        .highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ");

                    f.render_stateful_widget(items, chunks[1], &mut app.serialPorts.state);
                    block
                },
                1 => Block::default().title("Inner 1").borders(Borders::ALL),
                2 => Block::default().title("Inner 2").borders(Borders::ALL),
                3 => Block::default().title("Inner 3").borders(Borders::ALL),
                _ => unreachable!(),
            };
            f.render_widget(inner, chunks[1]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Down => {
                    if (app.tabs.current == Tab::Welcome) {
                        app.serialPorts.next();
                    }
                }
                Key::Up => {
                    if app.tabs.current == Tab::Welcome {
                        app.serialPorts.previous();
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

