use crossterm::event::KeyCode;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

use crate::{models::{Api, Resource, Request, NewApi}, services::{api_service, resource_service}};

pub enum LeftType {
    Apis,
    Resources(i32),
    Requests(i32)
}

enum LeftContent {
    Apis(Vec<Api>),
    Resources(Vec<Resource>, i32),
    Requests(Vec<Request>, i32)
}

pub enum LeftInputResult {
    Exit,
    None,
    ShowApi(i32),
    ShowResource(i32),
    ShowRequest(i32),
    IntoApi(i32),
    IntoResource(i32),
    EditRequest(i32)
}

pub enum SelectedItem {
    None,
    Api(i32),
    Resource(i32),
    Request(i32)
}

impl LeftContent {
    fn length(&self) -> usize {
        match self {
            LeftContent::Apis(apis) => apis.len(),
            LeftContent::Resources(resources, _) => resources.len(),
            LeftContent::Requests(requests, _) => requests.len()
        } 
    }
}

pub struct LeftList {
    content: LeftContent,
    pub list_state: ListState,
    input: Option<String>,
    selected_item: Option<usize>
}

impl<'a> LeftList {
    pub fn new() -> Self {
        let apis = api_service::get_all_apis();
        
        Self {
            content: LeftContent::Apis(apis),
            list_state: ListState::default(),
            input: None,
            selected_item: None
        }
    }

    fn set_selected(&mut self) {
        self.list_state.select(self.selected_item);
    }

    pub fn select_down(&mut self) {
        if let Some(selected_item) = self.list_state.selected() {
            if selected_item < self.content.length() - 1 {
                self.selected_item.replace(selected_item + 1);
                self.set_selected();
            } else {
                self.select_first();
            }
        } else {
            self.select_first();
        }
    }

    pub fn select_up(&mut self) {
        if let Some(selected_item) = self.list_state.selected() {
            if selected_item > 0 {
                self.selected_item.replace(selected_item - 1);
                self.set_selected();
            } else {
                self.select_last();
            }
        } else {
            self.select_first();
        }
    }

    pub fn select(&mut self, index: usize) {
        if index < self.content.length() {
            self.list_state.select(Some(index));
        }
    }

    pub fn select_first(&mut self) {
        if self.content.length() > 0 {
            self.selected_item.replace(0);
            self.set_selected();
        }
    }

    pub fn select_last(&mut self) {
        if self.content.length() > 0 {
            self.selected_item.replace(self.content.length() - 1);
            self.set_selected();
        }
    }

    fn changed_show(&self) -> LeftInputResult {
        if let Some(selected_index) = self.selected_item {
            match &self.content {
                LeftContent::Apis(apis) => LeftInputResult::ShowApi(apis[selected_index].id),
                LeftContent::Resources(resources, _) => LeftInputResult::ShowResource(resources[selected_index].id),
                LeftContent::Requests(requests, _) => LeftInputResult::ShowRequest(requests[selected_index].id)
            }
        } else {
            LeftInputResult::None
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) -> LeftInputResult {
        match key {
            KeyCode::Char(character) => {
                if let Some(input) = &mut self.input {
                   input.push(character);
                   LeftInputResult::None
                } else {
                    match character {
                        'j' => {
                            self.select_down();
                            self.changed_show()
                        },
                        'k' => {
                            self.select_up();
                            self.changed_show()
                        },
                        'q' => {
                            LeftInputResult::Exit
                        }
                        'a' => {
                            self.input.replace(String::new());
                            LeftInputResult::None
                        }
                        _ => LeftInputResult::None

                    }
                }
            }
            KeyCode::Enter => {
                if let Some(input) = &mut self.input {
                    match &self.content {
                        LeftContent::Apis(_) => {
                            api_service::create_new_api(NewApi { name: &input });
                            let new_api_list = api_service::get_all_apis();
                            self.content = LeftContent::Apis(new_api_list);
                        }
                        _ => {}
                    }
                    self.input.take();
                    LeftInputResult::None
                } else {
                    if let Some(selected_index) = self.selected_item {
                       match &self.content {
                           LeftContent::Requests(requests, _) => LeftInputResult::EditRequest(requests[selected_index].id),
                           LeftContent::Apis(apis) => {
                               let selected_api = &apis[selected_index];
                               let resources = resource_service::get_resources_for_api(selected_api.id);
                               let new_content = LeftContent::Resources(resources, selected_api.id);
                               self.content = new_content;

                               LeftInputResult::None
                           } 
                           _ => LeftInputResult::None

                       } 
                    } else {
                        LeftInputResult::None
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(input) = &mut self.input {
                    input.pop();
                    LeftInputResult::None
                } else {
                    match &self.content {
                        LeftContent::Resources(_, _) => {
                            let apis = api_service::get_all_apis();
                            let new_content = LeftContent::Apis(apis);
                            self.content = new_content;
                            LeftInputResult::None
                        } 
                        _ => LeftInputResult::None
                    }
                }
            }
            _ => LeftInputResult::None 
        }
    }

    pub fn render(&'a self) -> List<'a> {
        let list_title = match self.content {
            LeftContent::Apis(_) => "APIs",
            LeftContent::Resources(_, _) => "Resources",
            LeftContent::Requests(_, _) => "Requests",
        };

        let left_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(list_title)
            .border_type(BorderType::Plain);

        let mut items: Vec<_> = match &self.content{
            LeftContent::Apis(apis) => {
                let list = apis.iter().map(|api| {
                    ListItem::new(Spans::from(vec![Span::styled(
                        api.name.clone(),
                        Style::default(),
                    )]))
                });
                list.collect()
            }
            LeftContent::Resources(resources, _) => {
                let list = resources.iter().map(|resource| {
                    ListItem::new(Spans::from(vec![Span::styled(
                        resource.name.clone(),
                        Style::default(),
                    )]))
                });
                list.collect()
            }
            LeftContent::Requests(requests, _) => {
                let list = requests.iter().map(|request| {
                    ListItem::new(Spans::from(vec![
                        Span::styled(
                            format!("{}", request.method),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(&request.route, Style::default()),
                    ]))
                });
                list.collect()
            }
        };

        if let Some(input) = &self.input {
            items.push(
                ListItem::new(Spans::from(vec![Span::raw(input)])).style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        }

        let list = List::new(items).block(left_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        list
    }
}
