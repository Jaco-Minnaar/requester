use crate::schema::*;
use std::fmt;
use std::fmt::{Display, Formatter};

use diesel::Queryable;
use diesel_derive_enum::DbEnum;

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

#[derive(Debug, Clone, DbEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let repr = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
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

#[derive(Insertable)]
#[table_name = "request"]
pub struct NewRequest<'a> {
    pub resource_id: i32,
    pub route: &'a str,
    pub method: HttpMethod,
    pub body: Option<&'a str>
}

#[derive(Queryable)]
pub struct Header {
    id: i32,
    key: String,
    value: String,
    request_id: i32,
}

#[derive(Insertable)]
#[table_name = "header"]
pub struct NewHeader<'a> {
    key: &'a str,
    value: &'a str,
    request_id: i32
}
