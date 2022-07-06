use std::collections::HashMap;
use async_trait::async_trait;
use reqwest::{header, RequestBuilder, StatusCode};
use reqwest::{Error, Response};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use crate::{Account, PingNotAuthorized, PingSuccessful};
use crate::types::{AccountType, ApiResponse, OwnershipType};
use core::option::Option;
use std::borrow::Borrow;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use crate::request_sender::RequestSender;

const BASE_URL: &str = "https://api.up.com.au/api/v1";

#[async_trait]
pub trait ApiRequest {
    type T;
    async fn send(self) -> Result<Self::T, String>;
    fn get_url(&self) -> &String;
    fn get_params(&self) -> &Vec<(String, String)>;
    fn get_api_key(&self) -> &String;
}

pub struct ListAccounts {
    url: String,
    params: Vec<(String, String)>,
    api_key: String,
}

#[async_trait]
impl ApiRequest for ListAccounts {
    type T = Vec<Account>;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send_paginate::<Account, ListAccounts>(self).await
    }

    fn get_url(&self) -> &String {
        &self.url
    }

    fn get_api_key(&self) -> &String {
        &self.api_key
    }

    fn get_params(&self) -> &Vec<(String, String)> {
        &self.params
    }


}

impl ListAccounts {
    pub fn new(api_key: &String) -> ListAccounts {
        return ListAccounts { url: format!("{}/accounts",BASE_URL).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new()
        };
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

    /*pub async fn send(self) -> Result<Vec<Account>, String> {
        RequestSender::send_paginate::<Account>(self.url,self.headers,self.params).await
    }*/
}

pub struct RetrieveAccount {
    url: String,
    headers: HeaderMap,
    params: Vec<(String, String)>
}

impl RetrieveAccount {
    pub fn new(api_key: &String, id: String) -> RetrieveAccount {
        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
        return RetrieveAccount { url: format!("{}/accounts/{}",BASE_URL,id).to_string(),
            headers: header_map,
            params: Vec::new()
        };
    }

    pub async fn send(self) -> Result<Account, String> {
        RequestSender::send::<Account>(self.url, self.headers, self.params).await
    }
}
