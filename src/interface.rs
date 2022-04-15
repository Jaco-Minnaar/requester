use crate::api::{Api, HasName, Request, Resource};
use crossterm::event::{self, Event as CEvent, KeyCode};
use crossterm::terminal::disable_raw_mode;
use std::io::{self, Stdout};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    Widget,
};
use tui::Terminal;

enum Event<I> {
    Input(I),
    Tick,
}

pub enum ListType {
    API(Vec<Api>),
    Resource(Vec<Resource>),
    Request(Vec<Request>),
}

pub struct Interface {
    active_list: ListType,
    input_mode: bool,
    input_tx: Option<Sender<char>>,
    left_list_state: ListState,
    input: String,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

enum InputMode {
    ParentAdd,
    ChildAdd
}

impl Interface {
    pub fn new(list: ListType) -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        Self {
            active_list: list,
            input_mode: false,
            input_tx: None,
            left_list_state: ListState::default(),
            input: String::new(),
            terminal,
        }
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll doesn't work") {
                    if let CEvent::Key(key) = event::read().expect("can't read event") {
                        tx.send(Event::Input(key)).expect("can't send event");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        loop {
            self.draw();

            match rx.recv().unwrap() {
                Event::Input(event) => {
                    if self.handle_input(event.code) {
                        break;
                    }
                }
                Event::Tick => {}
            }
        }
    }

    pub fn draw(&mut self) {
        self.terminal
            .draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(2)].as_ref())
                    .split(size);

                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(chunks[0]);
                let left = render_left(&self.active_list);
                let right = render_right(&self.active_list);
                rect.render_stateful_widget(left, main_chunks[0], &mut self.left_list_state);
                rect.render_widget(right, main_chunks[1]);
            })
            .unwrap();
    }

    pub fn handle_input(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char(key) => {
                if self.input_mode {
                    self.input.push(key);
                    false
                } else {
                    match key {
                        'q' => {
                            disable_raw_mode().unwrap();
                            self.terminal.show_cursor().unwrap();
                            true
                        }
                        'a' => {
                            self.input.clear();
                            self.input_mode = true;
                            false
                        }
                        _ => false,
                    }
                }
            }
            KeyCode::Enter => {
                if self.input_mode {
                    match &mut self.active_list {
                        ListType::API(apis) => {
                            let new_api = Api::new(&self.input);
                            apis.push(new_api);
                        }
                        _ => {}
                    }

                    self.input_mode = false;
                }
                false
            }
            _ => false,
        }
    }

    pub fn start_input_mode(&mut self) -> Receiver<char> {
        let (tx, rx) = mpsc::channel();
        self.input_mode = true;
        self.input_tx = Some(tx);

        rx
    }

    pub fn active_list(&self) -> &ListType {
        &self.active_list
    }

    pub fn set_active_list(&mut self, list: ListType) {
        self.active_list = list
    }
}

fn render_left<'a>(list: &'a ListType) -> List<'a> {
    let list_title = match list {
        ListType::API(_) => "APIs",
        ListType::Resource(_) => "Resources",
        ListType::Request(_) => "Requests",
    };

    let left_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(list_title)
        .border_type(BorderType::Plain);

    let items: Vec<_> = match list {
        ListType::API(apis) => {
            let list = apis.iter().map(|api| {
                ListItem::new(Spans::from(vec![Span::styled(
                    api.name().clone(),
                    Style::default(),
                )]))
            });
            list.collect()
        }
        ListType::Resource(resources) => {
            let list = resources.iter().map(|resource| {
                ListItem::new(Spans::from(vec![Span::styled(
                    resource.name().clone(),
                    Style::default(),
                )]))
            });
            list.collect()
        }
        ListType::Request(requests) => {
            let list = requests.iter().map(|request| {
                ListItem::new(Spans::from(vec![
                    Span::styled(
                        format!("{}", request.method()),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(request.route(), Style::default()),
                ]))
            });
            list.collect()
        }
    };

    let list = List::new(items).block(left_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}

fn render_right<'a>(list: &'a ListType) -> impl Widget {
    let right_title = match list {
        ListType::API(_) => "Resources",
        ListType::Resource(_) => "Request",
        ListType::Request(_) => "Request Info",
    };

    let right = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "There is currently nothing selected",
            Style::default().fg(Color::LightYellow),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(right_title)
            .border_type(BorderType::Plain),
    );

    right
}
