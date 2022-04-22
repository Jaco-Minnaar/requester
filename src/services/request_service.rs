use crate::schema::request;
use diesel::{insert_into, prelude::*};

use crate::{
    establish_connection,
    models::{NewRequest, Request},
};

pub fn get_requests_for_resource(related_id: i32) -> Result<Vec<Request>, String> {
    let conn = establish_connection();

    let result = request::table
        .filter(request::dsl::resource_id.eq(related_id))
        .load::<Request>(&conn);

    match result {
        Ok(requests) => Ok(requests),
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn get_request_by_id(relevant_id: i32) -> Option<Request> {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    request.find(relevant_id).first(&conn).ok()
}

pub fn create_new_request(new_request: NewRequest) {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    insert_into(request)
        .values(&new_request)
        .execute(&conn)
        .unwrap();
}
