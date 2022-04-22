use diesel::{insert_into, prelude::*};

use crate::{
    establish_connection,
    models::{NewResource, Resource},
};

pub fn get_resources_for_api(related_id: i32) -> Vec<Resource> {
    use crate::schema::resource::dsl::*;

    let conn = establish_connection();

    let results = resource
        .filter(api_id.eq(related_id))
        .load::<Resource>(&conn);

    if let Ok(resources) = results {
        resources
    } else {
        vec![]
    }
}

pub fn get_resource_by_id(resource_id: i32) -> Option<Resource> {
    use crate::schema::resource::dsl::*;

    let conn = establish_connection();

    resource.find(resource_id).first(&conn).ok()
}

pub fn create_new_resource(new_resource: NewResource) {
    use crate::schema::resource::dsl::*;

    let conn = establish_connection();

    insert_into(resource)
        .values(&new_resource)
        .execute(&conn)
        .unwrap();
}
