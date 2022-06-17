use std::collections::HashMap;
use reqwest::header::HeaderMap;
use reqwest::{Error, Response};

#[derive(Debug)]
pub struct RustApiClient {
    root_url: String,
    api_key: String,
}

impl RustApiClient {
    pub fn new(root_url: String, api_key: String) -> RustApiClient {
        return RustApiClient {root_url, api_key};
    }

    pub fn ping() -> Result<Response, Error> {

    }
}

#[derive(Debug)]
struct RustApiRequest<'a, T> {
    client: RustApiClient,
    response_data: T,
    endpoint: &'a str,
    headers: HeaderMap,
    body: Option<HashMap<String, String>>
}

impl RustApiRequest<T> {
    pub(crate) fn new(response_data: T, endpoint: &str, headers: HeaderMap,
                      body: Option<HashMap<String, String>>) {

    }
}