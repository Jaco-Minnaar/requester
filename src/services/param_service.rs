use crate::{
    establish_connection,
    models::{NewParam, Param},
};
use diesel::insert_into;
use diesel::prelude::*;

pub fn get_params_for_request(related_id: i32) -> Vec<Param> {
    use crate::schema::param::dsl::*;

    let conn = establish_connection();

    let result = param.filter(request_id.eq(related_id)).load::<Param>(&conn);

    if let Ok(params) = result {
        params
    } else {
        vec![]
    }
}

pub fn get_param_by_id(param_id: i32) -> Option<Param> {
    use crate::schema::param::dsl::*;

    let conn = establish_connection();

    param.find(param_id).first(&conn).ok()
}

pub fn create_new_param(new_param: NewParam) {
    use crate::schema::param::dsl::*;

    let conn = establish_connection();

    insert_into(param)
        .values(&new_param)
        .execute(&conn)
        .unwrap();
}

pub fn update_param(obj: &NewParam) {
    use crate::schema::param::dsl::*;

    let conn = establish_connection();

    diesel::update(param).set(obj).execute(&conn).unwrap();
}
