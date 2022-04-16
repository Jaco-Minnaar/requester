use crate::{
    establish_connection,
    models::{Api, NewApi},
};
use diesel::{insert_into, prelude::*};

pub fn get_all_apis() -> Vec<Api> {
    use crate::schema::api::dsl::*;

    let connection = establish_connection();
    let results = api.load::<Api>(&connection).unwrap();

    results
}

pub fn get_api_by_id(api_id: i32) -> Option<Api> {
    use crate::schema::api::dsl::*;

    let connection = establish_connection();

    let result = api.find(api_id).first(&connection);

    if let Ok(matching) = result {
        Some(matching)
    } else {
        None
    }
}

pub fn create_new_api(new_api: NewApi) {
    use crate::schema::api::dsl::*;

    let connection = establish_connection();

    insert_into(api)
        .values(&new_api)
        .execute(&connection)
        .unwrap();
}
