use crate::schema::*;
use crate::types::HttpMethod;

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

#[derive(Queryable, Identifiable)]
#[table_name = "request"]
pub struct Request {
    pub id: i32,
    pub route: String,
    pub method: HttpMethod,
    pub body: Option<String>,
    pub resource_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "request"]
pub struct NewRequest<'a> {
    pub resource_id: i32,
    pub route: &'a str,
    pub method: HttpMethod,
    pub body: Option<&'a str>,
}

#[derive(Queryable, Identifiable)]
#[table_name = "header"]
pub struct Header {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub request_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "header"]
pub struct NewHeader<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub request_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "param"]
pub struct Param {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub request_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "param"]
pub struct NewParam<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub request_id: i32,
}
