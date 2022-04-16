use crate::schema::*;
use std::fmt;
use std::fmt::{Display, Formatter};

use diesel::Queryable;

#[derive(Queryable)]
pub struct Api {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "api"]
pub struct NewApi<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Resource {
    pub id: i32,
    pub name: String,
    pub api_id: i32,
}

#[derive(Insertable)]
#[table_name = "resource"]
pub struct NewResource<'a> {
    pub name: &'a str,
    pub api_id: i32,
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

#[derive(Queryable)]
pub struct Request {
    pub id: i32,
    pub resource_id: i32,
    pub route: String,
    pub method: HttpMethod,
    pub body: Option<String>,
}

#[derive(Queryable)]
pub struct Header {
    key: String,
    value: String,
    request_id: u32,
}
