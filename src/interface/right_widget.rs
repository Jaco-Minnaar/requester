use std::collections::HashMap;

use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::{
    models::{Api, Header, Request, Resource},
    services::{api_service, resource_service},
};

pub enum RightType {
    Api(i32),
    None
    //Resource(i32),
    //Request(i32)
}

enum Content {
    Api(Api, Vec<Resource>),
    Resource(Resource, Vec<Request>),
    Request(Request, HashMap<String, Header>),
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

                Some(Content::Api(api, resources))
            }
            RightType::None => None
        };

        Self { content }
    }

    pub fn set_type(&mut self, new_type: RightType) {
        let content = match new_type {
            RightType::Api(api_id) => {
                let api = api_service::get_api_by_id(api_id).unwrap();
                let resources = resource_service::get_resources_for_api(api_id);

                Some(Content::Api(api, resources))
            }
            RightType::None => None
        };

        if let Some(content) = content {
            self.content.replace(content);
        } else {
            self.content.take();
        }
        
    }

    pub fn render(&self) -> impl Widget {
        let text = if let Some(Content::Api(api, resources)) = &self.content {
            format!("{} is selected", api.name)
        } else {
            "Nothing is selected".to_owned()
        };

        let right = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                text,
                Style::default().fg(Color::LightYellow),
            )]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Nothin")
                .border_type(BorderType::Plain),
        );



        right
    }
}
