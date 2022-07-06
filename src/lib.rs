#![allow(dead_code, unused_imports)]

pub mod api_client;
pub mod request_sender;
pub mod types;

use reqwest;
use serde;
use serde::ser::Error;
use serde_json::{json, Result};
use std::env;
use std::env::VarError;
use types::{Account, ApiResponse, Category, PingNotAuthorized, PingSuccessful, Tag, Transaction};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
