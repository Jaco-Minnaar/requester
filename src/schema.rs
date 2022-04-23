table! {
    api (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    header (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
        request_id -> Integer,
    }
}

table! {
    param (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
        request_id -> Integer,
    }
}

table! {
    use diesel::sql_types::{Integer, Text, Nullable};
    use crate::types::HttpMethodMapping;
    request (id) {
        id -> Integer,
        route -> Text,
        method -> HttpMethodMapping,
        body -> Nullable<Text>,
        resource_id -> Integer,
    }
}

table! {
    resource (id) {
        id -> Integer,
        name -> Text,
        api_id -> Integer,
    }
}

joinable!(header -> request (request_id));
joinable!(param -> request (request_id));
joinable!(request -> resource (resource_id));
joinable!(resource -> api (api_id));

allow_tables_to_appear_in_same_query!(api, header, param, request, resource,);
