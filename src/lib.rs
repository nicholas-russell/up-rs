mod query_builder;
mod types;
pub mod api_client;

use reqwest;
use serde;
use types::{Response,Account,Category,Tag,Transaction,PingSuccessful,PingNotAuthorized};
use serde_json::{Result, json};
use std::env;
use std::env::VarError;
use serde::ser::Error;
use crate::api_client::RustApiClient;

const API_KEY_ENV: &'static str = "UP_API_KEY";

pub fn get_client(root_url: String) -> RustApiClient {
    let mut client: RustApiClient;
    match env::var(API_KEY_ENV) {
        Ok(v) => client = RustApiClient::new(root_url, v),
        Err(e) => panic!("No API key in environment.")
    }
    return client;
}

pub fn get_accounts() -> Result<Vec<Account>> {
    return Err(serde_json::Error::custom("Text"))
}

pub fn testing(input: &str) {
    let content: PingNotAuthorized = serde_json::from_str(input).unwrap();
    println!("{:?}", content);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
