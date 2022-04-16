table! {
    api (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    resource (id) {
        id -> Integer,
        name -> Text,
        api_id -> Integer,
    }
}

joinable!(resource -> api (api_id));

allow_tables_to_appear_in_same_query!(api, resource,);
