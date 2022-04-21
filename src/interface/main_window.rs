use super::left_widget::{LeftInputResult, LeftList};
use super::right_widget::{RightType, RightWidget};
use crossterm::event::{self, Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Stdout};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

enum Event<I> {
    Input(I),
    Tick,
}

enum Focus {
    Left,
    Right,
}

pub struct MainWindow {
    input_mode: bool,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    left: LeftList,
    right: RightWidget,
    focus: Focus,
}

impl<'a> MainWindow {
    pub fn new() -> Self {
        enable_raw_mode().expect("Could not enable raw mode");
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        let right = RightWidget::new(RightType::None);

        Self {
            input_mode: false,
            terminal,
            left: LeftList::new(),
            right,
            focus: Focus::Left,
        }
    }

    pub fn run(&'a mut self) {
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

                let left_list_state = &self.left.list_state;
                let left = self.left.render();
                self.right.draw(rect, main_chunks[1]);
                rect.render_stateful_widget(left, main_chunks[0], &mut left_list_state.clone());
            })
            .unwrap();
    }

    pub fn handle_input(&'a mut self, key_code: KeyCode) -> bool {
        match self.focus {
            Focus::Left => {
                let result = self.left.handle_input(key_code);
                match result {
                    LeftInputResult::ShowApi(_)
                    | LeftInputResult::ShowRequest(_)
                    | LeftInputResult::ShowResource(_) => {
                        self.set_right(result);
                        false
                    }
                    LeftInputResult::Exit => {
                        disable_raw_mode().unwrap();
                        self.terminal.show_cursor().unwrap();
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn set_right(&mut self, input_result: LeftInputResult) {
        let right_type = match input_result {
            LeftInputResult::ShowApi(api) => RightType::Api(api),
            //LeftInputResult::ShowResource(resource) => RightType::Resource(resource),
            //LeftInputResult::ShowRequest(request) => RightType::Request(request),
            _ => RightType::None,
        };

        self.right.set_type(right_type);
    }
}
