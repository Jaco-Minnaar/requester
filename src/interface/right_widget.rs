use std::collections::HashMap;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{ListItem, List, Block, Borders, BorderType}
};

use crate::{
    models::{Api, Header, Request, Resource},
    services::{api_service, resource_service},
};

pub enum RightType {
    Api(i32),
    None, //Resource(i32),
          //Request(i32)
}

enum Content {
    Api(ApiWidget),
    Resource(Resource, Vec<Request>),
    Request(Request, HashMap<String, Header>),
}

struct ApiWidget {
    api: Api,
    resources: Vec<Resource>,
}

impl ApiWidget {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        let right_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(rect)[0];

        let list: Vec<_> = self.resources.iter().map(|resource| {
            ListItem::new(Spans::from(vec![Span::styled(
                resource.name.clone(),
                Style::default(),
            )]))
        }).collect();

        let right_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("API Details")
            .border_type(BorderType::Plain);

        let widget = List::new(list).block(right_block);

        frame.render_widget(widget, right_chunk);
    }
}

pub struct RightWidget {
    content: Option<Content>,
}

impl RightWidget {
    pub fn new(right_type: RightType) -> Self {
        let content = match right_type {
            RightType::Api(api_id) => {
                let api = api_service::get_api_by_id(api_id).unwrap();
                let resources = resource_service::get_resources_for_api(api_id);

                Some(Content::Api(ApiWidget { api, resources }))
            }
            RightType::None => None,
        };

        Self { content }
    }

    pub fn set_type(&mut self, new_type: RightType) {
        let content = match new_type {
            RightType::Api(api_id) => {
                let api = api_service::get_api_by_id(api_id).unwrap();
                let resources = resource_service::get_resources_for_api(api_id);

                Some(Content::Api(ApiWidget { api, resources }))
            }
            RightType::None => None,
        };

        if let Some(content) = content {
            self.content.replace(content);
        } else {
            self.content.take();
        }
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
        if let Some(content) = &self.content {
            match content {
                Content::Api(api_widget) => api_widget.draw(frame, rect),
                _ => {}
            }
        }
    }
}
