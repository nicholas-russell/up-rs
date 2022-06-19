#![allow(dead_code, unused_imports)]

mod query_builder;
pub mod types;
pub mod api_client;

use reqwest;
use serde;
use types::{ApiResponse,Account,Category,Tag,Transaction,PingSuccessful,PingNotAuthorized};
use serde_json::{Result, json};
use std::env;
use std::env::VarError;
use serde::ser::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
