use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct Api {
    resources: Vec<Resource>,
    name: String,
}

impl Api {
    pub fn new(name: &str) -> Self {
        Self {
            resources: vec![],
            name: name.to_owned(),
        }
    }

    pub fn push_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    pub fn remove_resource(&mut self, index: usize) {
        self.resources.remove(index);
    }
}

impl From<Vec<Resource>> for Api {
    fn from(resources: Vec<Resource>) -> Self {
        Self {
            resources,
            name: "".to_owned(),
        }
    }
}

impl HasName for Api {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct Resource {
    requests: Vec<Request>,
    name: String,
}

impl Resource {
    pub fn new(name: &str) -> Self {
        Self {
            requests: vec![],
            name: name.to_owned(),
        }
    }

    pub fn push_request(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub fn remove_request(&mut self, index: usize) {
        self.requests.remove(index);
    }

    pub fn requests(&self) -> &[Request] {
        &self.requests
    }
}

impl From<Vec<Request>> for Resource {
    fn from(requests: Vec<Request>) -> Self {
        Self {
            requests,
            name: "".to_owned(),
        }
    }
}

impl HasName for Resource {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let repr = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Clone)]
pub struct Request {
    route: String,
    method: HttpMethod,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            route: String::new(),
            method: HttpMethod::GET,
            body: None,
            headers: HashMap::new(),
        }
    }

    pub fn route(&self) -> &str {
        &self.route
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }
}
