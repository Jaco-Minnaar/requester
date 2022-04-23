use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, Row, Table, TableState, Tabs, Widget, Paragraph
    },
};

use crate::{
    models::{Api, Header, NewHeader, Request, Resource},
    services::{api_service, header_service, request_service, resource_service},
    types::HttpMethod,
    http
};

use reqwest::blocking::Response;

pub enum RightType {
    Api(i32),
    Resource(i32),
    Request(i32),
    None, //Request(i32)
}

pub enum RightInputResult {
    Exit,
    LoseFocus,
    RefreshRequests,
    None,
}

enum Content {
    Api(ApiWidget),
    Resource(ResourceWidget),
    Request(RequestWidget),
}

trait Drawable {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect);
}

struct ApiWidget {
    api: Api,
    resources: Vec<Resource>,
}

impl Drawable for ApiWidget {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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
    request_details_table_state: TableState,
    header_table_state: TableState,
    param_table_state: TableState,
    headers: Vec<Header>,
    input: Option<String>,
    response: Option<String>
}

impl RequestWidget {
    fn new(request: Request, headers: Vec<Header>) -> Self {
        Self {
            tab_index: 0,
            request,
            request_details_table_state: TableState::default(),
            header_table_state: TableState::default(),
            param_table_state: TableState::default(),
            headers,
            input: None,
            response: None
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
            0 => match self.request_details_table_state.selected() {
                Some(0) => self.request_details_table_state.select(Some(1)),
                Some(_) | None => self.request_details_table_state.select(Some(0)),
            },
            1 => {
                if self.headers.len() > 0 {
                    match self.header_table_state.selected() {
                        Some(current) if current < self.headers.len() - 2 => {
                            self.header_table_state.select(Some(current + 1))
                        }
                        None => (),
                        _ => self.header_table_state.select(Some(0)),
                    }
                }
            }
            _ => (),
        }
    }

    fn move_up(&mut self) {
        match self.tab_index {
            0 => match self.request_details_table_state.selected() {
                Some(0) => self.request_details_table_state.select(Some(1)),
                Some(_) | None => self.request_details_table_state.select(Some(0)),
            },
            1 => {
                if !self.headers.is_empty() {
                    match self.header_table_state.selected() {
                        Some(current) if current > 0 => {
                            self.header_table_state.select(Some(current - 1));
                        }
                        None => (),
                        _ => self.header_table_state.select(Some(self.headers.len() - 1)),
                    }
                }
            }
            _ => (),
        }
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
                                0 => {
                                    if let Some(0) = self.request_details_table_state.selected() {
                                        self.input
                                            .replace(String::from(self.request.route.as_str()));
                                    }
                                }
                                _ => (),
                            }
                            RightInputResult::None
                        }
                        'a' => {
                            match self.tab_index {
                                0 => {
                                    if let None = self.header_table_state.selected() {
                                        self.input.replace(String::new());
                                    }
                                }
                                _ => (),
                            }
                            RightInputResult::None
                        }
                        'j' => {
                            self.move_down();
                            RightInputResult::None
                        }
                        'k' => {
                            self.move_up();
                            RightInputResult::None
                        }
                        'l' => {
                            self.next_tab();
                            RightInputResult::None
                        }
                        'h' => {
                            self.previous_tab();
                            RightInputResult::None
                        }
                        'r' => {
                            let response = http::make_request(self.request.id, "https://dummyjson.com");
                            
                            self.response = Some(response.text().unwrap());
                            RightInputResult::None
                        }
                        'D' | 'H' | 'P' | 'B' => {
                            let options = ['D', 'H', 'P', 'B'];
                            self.tab_index = options.iter().position(|x| x == &character).unwrap();
                            RightInputResult::None
                        }
                        _ => RightInputResult::None,
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
                    let result = match self.tab_index {
                        0 => {
                            if let Some(0) = self.request_details_table_state.selected() {
                                let updated_request = request_service::update_request_route(
                                    &self.request,
                                    input.as_str(),
                                );
                                self.request = updated_request;
                                RightInputResult::RefreshRequests
                            } else {
                                RightInputResult::None
                            }
                        }
                        1 => {
                            if let Some(selected_header_index) = self.header_table_state.selected()
                            {
                                let selected_header = &self.headers[selected_header_index];
                                let updated_header = NewHeader {
                                    key: selected_header.key.as_str(),
                                    value: input.as_str(),
                                    request_id: selected_header.request_id,
                                };
                                header_service::update_header(&selected_header, &updated_header);
                            } else {
                                header_service::create_new_header(NewHeader {
                                    key: &input,
                                    value: "",
                                    request_id: self.request.id,
                                });
                            }
                            let headers = header_service::get_headers_for_request(self.request.id);
                            self.headers = headers;
                            RightInputResult::None
                        }
                        _ => RightInputResult::None,
                    };
                    self.input.take();
                    result
                } else {
                    match self.tab_index {
                        0 => match self.request_details_table_state.selected() {
                            Some(0) => {
                                self.input
                                    .replace(String::from(self.request.route.as_str()));
                                RightInputResult::None
                            }
                            Some(1) => {
                                let new_method = match self.request.method {
                                    HttpMethod::Get => HttpMethod::Post,
                                    HttpMethod::Post => HttpMethod::Put,
                                    HttpMethod::Put => HttpMethod::Patch,
                                    HttpMethod::Patch => HttpMethod::Delete,
                                    HttpMethod::Delete => HttpMethod::Get,
                                };
                                let updated_request = request_service::update_request_method(
                                    &self.request,
                                    new_method,
                                );
                                self.request = updated_request;

                                RightInputResult::RefreshRequests
                            }
                            _ => RightInputResult::None,
                        },
                        _ => RightInputResult::None,
                    }
                }
            }
            _ => RightInputResult::None,
        }
    }

    pub fn request_details(&self) -> Table {
        let block = Block::default()
            .title("Request Details")
            .borders(Borders::ALL);

        let route_value = if let Some(input) = &self.input {
            input.clone()
        } else {
            self.request.route.clone()
        };

        let highlight_style = if self.input.is_some() { Style::default().bg(Color::Cyan).fg(Color::Black)} else { Style::default().fg(Color::Black).bg(Color::Yellow) };


        let route_row = Row::new([
            Cell::from("Route"),
            Cell::from(route_value)
        ])
        .height(1);
        let method_row = Row::new([
            Cell::from("Http Method"),
            Cell::from(self.request.method.to_string()),
        ])
        .height(1);

        Table::new([route_row, method_row])
            .block(block)
            .highlight_style(highlight_style)
            .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
    }
}

impl Drawable for RequestWidget {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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

        let tabs_block = Block::default()
            .style(Style::default())
            .title("Request Details")
            .borders(Borders::ALL);

        let first_title = Spans::from(vec![Span::styled("Request ", Style::default()), Span::styled("D", Style::default().add_modifier(Modifier::UNDERLINED)), Span::raw("etails")]);
        let mut titles: Vec<Spans> = ["Headers", "Params", "Body"]
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
                    Span::styled(rest, Style::default()),
                ])
            })
            .collect();

        titles.insert(0, first_title);

        let tabs = Tabs::new(titles)
            .block(tabs_block)
            .select(self.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Yellow)
                    .fg(Color::Black),
            );

        frame.render_widget(tabs, request_chunks[0]);

        match self.tab_index {
            0 => {
                let widget = self.request_details();
                frame.render_stateful_widget(
                    widget,
                    request_chunks[1],
                    &mut self.request_details_table_state.clone(),
                );
            }
            1 => {
                let widget = Block::default()
                    .title("Request Headers")
                    .borders(Borders::ALL);
                frame.render_widget(widget, request_chunks[1]);
            }
            2 => {
                let widget = Block::default()
                    .title("Request Params")
                    .borders(Borders::ALL);
                frame.render_widget(widget, request_chunks[1]);
            }
            3 => {
                let widget = Block::default().title("Request Body").borders(Borders::ALL);
                frame.render_widget(widget, request_chunks[1]);
            }
            _ => (),
        };

        let response_block = Block::default().title("Response").borders(Borders::ALL);

        if let Some(response) = &mut self.response {
            let response_para = Paragraph::new(response.as_str()).block(response_block);
            frame.render_widget(response_para, response_chunk);
            
        }

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

    pub fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        if let Some(content) = &mut self.content {
            match content {
                Content::Api(api_widget) => api_widget.draw(frame, rect),
                Content::Resource(resource_widget) => resource_widget.draw(frame, rect),
                Content::Request(request_widget) => request_widget.draw(frame, rect),
            }
        }
    }

    pub fn handle_input(&mut self, key_code: KeyCode) -> RightInputResult {
        match &mut self.content {
            Some(Content::Request(request_widget)) => request_widget.handle_input(key_code),
            _ => RightInputResult::None,
        }
    }
}
