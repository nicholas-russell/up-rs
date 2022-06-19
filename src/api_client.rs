use std::collections::HashMap;
use reqwest::{header, RequestBuilder, StatusCode};
use reqwest::{Error, Response};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use crate::{Account, PingNotAuthorized, PingSuccessful};
use crate::types::{AccountType, ApiResponse, OwnershipType};
use core::option::Option;
use std::borrow::Borrow;
use serde_json::Value;

const BASE_URL: &str = "https://api.up.com.au/api/v1";

pub struct ListAccounts {
    url: String,
    headers: HeaderMap,
    params: Vec<(String, String)>,
    pub response: Option<String>
}

impl ListAccounts {
    pub fn new(api_key: String) -> ListAccounts {
        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
        return ListAccounts { url: format!("{}/accounts",BASE_URL).to_string(),
            headers: header_map,
            params: Vec::new(),
            response: None};
    }

    pub fn page_size(mut self, page_size: i32) -> ListAccounts {
        if page_size > 0 && page_size <= 30 {
            self.params.push(("page[size]".to_string(), page_size.to_string()));
        } else {
            eprintln!("Page size has to be between 1 and 30.");
        }
        return self
    }

    pub fn account_type(mut self, account_type: AccountType) -> ListAccounts {
        self.params.push(("filter[accountType]".to_string(), account_type.to_string()));
        return self
    }

    pub fn ownership_type(mut self, ownership_type: OwnershipType) -> ListAccounts {
        self.params.push((String::from("filter[ownershipType]"), ownership_type.to_string()));
        return self
    }

    pub async fn send(self) -> Result<Vec<Account>, String> {
        let client = reqwest::Client::new();
        let res =  client
            .get(self.url)
            .headers(self.headers.to_owned())
            .query(&self.params)
            .send().await;

        match res {
            Ok(v) => {
                if v.status().is_success() {
                    let mut rtn: Vec<Account> = Vec::new();
                    let mut json: ApiResponse<Vec<Account>> = v.json().await.unwrap();
                    rtn.append(&mut json.data);
                    println!("{:?}", rtn);
                    while ApiResponse::has_next(&json) {
                        let next = json.links.as_ref().unwrap().get("next").unwrap().to_owned().unwrap();
                        let client = reqwest::Client::new();
                        let res =  client
                            .get(next)
                            .headers(self.headers.to_owned())
                            .send().await;
                        match res {
                            Ok(v) => {
                                if v.status().is_success() {
                                    json = v.json().await.unwrap();
                                    rtn.append(&mut json.data);
                                } else {
                                    return Err(v.text().await.unwrap());
                                }
                            },
                            Err(e) => return Err(e.to_string())
                        }
                    }
                    return Ok(rtn)
                } else {
                    return Err(String::from(v.text().await.unwrap()));
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }
}