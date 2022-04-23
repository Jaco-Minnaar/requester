use crate::{services::*, types::HttpMethod};
use reqwest::blocking::{Client, Response};

pub fn make_request(request_id: i32, hostname: &str) -> Response {
    let request = request_service::get_request_by_id(request_id).unwrap();

    let client = Client::new();

    let resp = match request.method {
        HttpMethod::Get => {
            let resp = client.get(format!("{}{}", hostname, request.route)).send().unwrap();

            resp
        }
        HttpMethod::Post => {
            let resp = client.post(format!("{}{}", hostname, request.route)).send().unwrap();

            resp
        }
        HttpMethod::Put => {
            let resp = client.put(format!("{}{}", hostname, request.route)).send().unwrap();

            resp
        }
        HttpMethod::Patch => {
            let resp = client.patch(format!("{}{}", hostname, request.route)).send().unwrap();

            resp
        }
        HttpMethod::Delete => {
            let resp = client.delete(format!("{}{}", hostname, request.route)).send().unwrap();

            resp
        }
    };

    resp
}
