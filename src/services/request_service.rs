use diesel::{insert_into, prelude::*};

use crate::{
    establish_connection,
    models::{NewRequest, Request},
    types::HttpMethod,
};

pub fn get_requests_for_resource(related_id: i32) -> Result<Vec<Request>, String> {
    use crate::schema::request::dsl::*;
    let conn = establish_connection();

    let result = request
        .filter(resource_id.eq(related_id))
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

pub fn update_request(target: &Request, changes: &NewRequest) {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    diesel::update(target).set(changes).execute(&conn).unwrap();
}

pub fn update_request_method(obj: &Request, new_method: HttpMethod) -> Request {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    diesel::update(obj)
        .set(method.eq(new_method))
        .execute(&conn)
        .unwrap();

    request.find(obj.id).first(&conn).unwrap()
}

pub fn update_request_route(obj: &Request, new_route: &str) -> Request {
    use crate::schema::request::dsl::*;

    let conn = establish_connection();

    diesel::update(obj)
        .set(route.eq(new_route))
        .execute(&conn)
        .unwrap();

    request.find(obj.id).first(&conn).unwrap()
}
