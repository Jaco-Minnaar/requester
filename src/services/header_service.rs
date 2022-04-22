use crate::{models::{Header, NewHeader}, establish_connection};
use diesel::prelude::*;
use diesel::insert_into;


pub fn get_headers_for_request(related_id: i32) -> Vec<Header> {
    use crate::schema::header::dsl::*;

    let conn = establish_connection();

    let result = header.filter(request_id.eq(related_id)).load::<Header>(&conn);

    if let Ok(headers) = result {
        headers
    } else {
        vec![]
    }
}

pub fn get_header_by_id(header_id: i32) -> Option<Header> {
    use crate::schema::header::dsl::*;

    let conn = establish_connection();
    
    header.find(header_id).first(&conn).ok()
}

pub fn create_new_header(new_header: NewHeader) {
    use crate::schema::header::dsl::*;

    let conn = establish_connection();
    
    insert_into(header).values(&new_header).execute(&conn).unwrap();
}

pub fn update_header(obj: &NewHeader) {
    use crate::schema::header::dsl::*;

    let conn = establish_connection();

    diesel::update(header).set(obj).execute(&conn).unwrap();
}
