use diesel::{insert_into, prelude::*};

use crate::{
    establish_connection,
    models::{NewRequest, Request},
};

pub fn get_requests_for_resource(related_id: i32) -> Vec<Request> {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    let results = request
        .filter(resource_id.eq(related_id))
        .load(&conn);

    if let Ok(requests) = results {
        requests
    } else {
        vec![]
    }
}

pub fn create_new_request(new_request: NewRequest) {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    insert_into(request)
        .values(&new_request)
        .execute(&conn)
        .unwrap();
}
