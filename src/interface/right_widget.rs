use std::collections::HashMap;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};
use crossterm::event::KeyCode;

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, TableState, Tabs},
};

use crate::{
    models::{Api, Header, Request, Resource, NewHeader},
    services::{api_service, request_service, resource_service, header_service},
};

pub enum RightType {
    Api(i32),
    Resource(i32),
    Request(i32),
    None, //Request(i32)
}

pub enum RightInputResult {
    Exit,
    LoseFocus,
    None
}

enum Content {
    Api(ApiWidget),
    Resource(ResourceWidget),
    Request(RequestWidget),
}

trait Drawable {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect);
}

struct ApiWidget {
    api: Api,
    resources: Vec<Resource>,
}

impl Drawable for ApiWidget {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        let right_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(rect)[0];

        let list: Vec<_> = self
            .resources
            .iter()
            .map(|resource| {
                ListItem::new(Spans::from(vec![Span::styled(
                    resource.name.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        let right_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("API Details")
            .border_type(BorderType::Plain);

        let widget = List::new(list).block(right_block);

        frame.render_widget(widget, right_chunk);
    }
}

struct ResourceWidget {
    resource: Resource,
    requests: Vec<Request>,
}

impl Drawable for ResourceWidget {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        let right_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(rect)[0];

        let list: Vec<_> = self
            .requests
            .iter()
            .map(|request| {
                ListItem::new(Spans::from(vec![Span::styled(
                    request.route.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        let right_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Resource Details")
            .border_type(BorderType::Plain);

        let widget = List::new(list).block(right_block);

        frame.render_widget(widget, right_chunk);
    }
}

struct RequestWidget {
    tab_index: usize,
    request: Request,
    header_list_state: TableState,
    param_list_state: TableState,
    headers: Vec<Header>,
    input: Option<String>
}

impl RequestWidget {
    fn new(request: Request, headers: Vec<Header>) -> Self {
        Self {
            tab_index: 0,
            request,
            header_list_state: TableState::default(),
            param_list_state: TableState::default(),
            headers: vec![],
            input: None
        }
    }

    fn next_tab(&mut self) {
        if self.tab_index < 3 {
            self.tab_index += 1;
        } else {
            self.tab_index = 0;
        }
    }

    fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = 3;
        }
    }

    fn move_down(&mut self) {
        match self.tab_index {
            0 => {
                if self.headers.len() > 0 {
                    match self.header_list_state.selected() {
                        Some(current) if current < self.headers.len() - 2 => {
                            self.header_list_state.select(Some(current + 1))
                        }
                        None => (),
                        _ => self.header_list_state.select(Some(0)),
                    }
                }
            }
            _ => ()
        }
    }

    fn move_up(&mut self) {
        match self.tab_index {
            0 => {
                if !self.headers.is_empty() {
                    match self.header_list_state.selected() {
                        Some(current) if current > 0  => {
                            self.header_list_state.select(Some(current - 1));
                        }
                        None => (),
                        _ => self.header_list_state.select(Some(self.headers.len() - 1))
                    }
                }
            }
            _ => ()
        }
    }

    fn start_edit(&mut self) {
        self.input.replace(String::new());
    }

    fn handle_input(&mut self, key: KeyCode) -> RightInputResult {
        match key {
            KeyCode::Char(character) => {
                if let Some(input) = &mut self.input {
                    input.push(character);
                    RightInputResult::None
                } else {
                    match character {
                        'q' => RightInputResult::Exit,
                        'e' => {
                            match self.tab_index {
                                0 => if let Some(_) = self.header_list_state.selected() {
                                    self.input.replace(String::new());
                                }
                                _ => ()
                            }
                            RightInputResult::None
                        }
                        'a' => {
                            match self.tab_index {
                                0 => if let None = self.header_list_state.selected() {
                                    self.input.replace(String::new());
                                }
                                _ => ()
                            }
                            RightInputResult::None
                        }
                        _ => RightInputResult::None
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(input) = &mut self.input {
                    input.pop();
                    RightInputResult::None
                } else {
                    RightInputResult::LoseFocus
                }

            }
            KeyCode::Enter => {
                if let Some(input) = &self.input {
                    match self.tab_index {
                        0 => {
                            if let Some(selected_header_index) = self.header_list_state.selected() {
                                let selected_header = &self.headers[selected_header_index];
                                let updated_header = NewHeader {key: selected_header.key.as_str(), value: input.as_str(), request_id: selected_header.request_id };
                                header_service::update_header(&updated_header);
                            } else {
                                header_service::create_new_header(NewHeader { key: &input, value: "", request_id: self.request.id });
                                let headers = header_service::get_headers_for_request(self.request.id);
                                self.headers = headers;
                            }
                        }
                        _ => ()
                    }
                    RightInputResult::None
                } else {
                    RightInputResult::None
                }
            }
            _ => RightInputResult::None,
        }
    }
}

impl Drawable for RequestWidget {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) { 
        let right_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        let request_chunk = right_chunks[0];
        let response_chunk = right_chunks[1];

        let request_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(request_chunk);

        let tabs_block = Block::default().style(Style::default()).title("Request Details").borders(Borders::ALL);

        let titles = ["Headers", "Params", "Body"].iter().map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled(rest, Style::default())
            ])
        }).collect() ;

        let tabs = Tabs::new(titles)
            .block(tabs_block)
            .select(self.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Red));

        frame.render_widget(tabs, request_chunks[0]);

        let inner = match self.tab_index {
            0 => Some(Block::default().title("Request Headers")),
            1 => Some(Block::default().title("Request Params")),
            2 => Some(Block::default().title("Request Body")),
            _ => None
        };

        if let Some(inner) = inner {
            frame.render_widget(inner.borders(Borders::ALL), request_chunks[1]);
        }

        let response_block = Block::default().title("Response").borders(Borders::ALL);

        frame.render_widget(response_block, response_chunk);
    }
}

pub struct RightWidget {
    content: Option<Content>,
}

impl RightWidget {
    pub fn new(right_type: RightType) -> Self {
        let content = Self::new_content(&right_type);

        Self { content }
    }

    pub fn set_type(&mut self, new_type: RightType) {
        let content = Self::new_content(&new_type);
        if let Some(content) = content {
            self.content.replace(content);
        } else {
            self.content.take();
        }
    }

    fn new_content(new_type: &RightType) -> Option<Content> {
        match new_type {
            RightType::Api(api_id) => {
                let api = api_service::get_api_by_id(*api_id).unwrap();
                let resources = resource_service::get_resources_for_api(*api_id);

                Some(Content::Api(ApiWidget { api, resources }))
            }
            RightType::Resource(resource_id) => {
                let resource = resource_service::get_resource_by_id(*resource_id).unwrap();
                let requests = request_service::get_requests_for_resource(*resource_id).unwrap();

                Some(Content::Resource(ResourceWidget { resource, requests }))
            }
            RightType::Request(request_id) => {
                let request = request_service::get_request_by_id(*request_id).unwrap();
                let headers = header_service::get_headers_for_request(*request_id);

                Some(Content::Request(RequestWidget::new(request, headers)))
            }
            RightType::None => None,
        }
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        if let Some(content) = &self.content {
            match content {
                Content::Api(api_widget) => api_widget.draw(frame, rect),
                Content::Resource(resource_widget) => resource_widget.draw(frame, rect),
                Content::Request(request_widget) => request_widget.draw(frame, rect),
            }
        }
    }
}
