use crate::request_sender::{ApiRequest, RequestSender};
use crate::types::{AccountType, ApiResponse, OwnershipType};
use crate::{Account, Category, PingNotAuthorized, PingSuccessful};
use async_trait::async_trait;
use core::option::Option;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{header, RequestBuilder, StatusCode};
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.up.com.au/api/v1";

pub struct ListAccounts {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
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

    fn get_params(&self) -> &Vec<(String, String)> {
        &self.params
    }

    fn get_api_key(&self) -> &String {
        &self.api_key
    }
}

impl ListAccounts {
    pub fn new(api_key: &String) -> ListAccounts {
        return ListAccounts {
            url: format!("{}/accounts", BASE_URL).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }

    pub fn page_size(mut self, page_size: i32) -> ListAccounts {
        if page_size > 0 && page_size <= 30 {
            self.params
                .push(("page[size]".to_string(), page_size.to_string()));
        } else {
            eprintln!("Page size has to be between 1 and 30.");
        }
        return self;
    }

    pub fn account_type(mut self, account_type: AccountType) -> ListAccounts {
        self.params
            .push(("filter[accountType]".to_string(), account_type.to_string()));
        return self;
    }

    pub fn ownership_type(mut self, ownership_type: OwnershipType) -> ListAccounts {
        self.params.push((
            String::from("filter[ownershipType]"),
            ownership_type.to_string(),
        ));
        return self;
    }
}

pub struct RetrieveAccount {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for RetrieveAccount {
    type T = Account;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send::<Account, RetrieveAccount>(self).await
    }

    fn get_url(&self) -> &String {
        &self.url
    }

    fn get_params(&self) -> &Vec<(String, String)> {
        &self.params
    }

    fn get_api_key(&self) -> &String {
        &self.api_key
    }
}

impl RetrieveAccount {
    pub fn new(api_key: &String, id: String) -> RetrieveAccount {
        return RetrieveAccount {
            url: format!("{}/accounts/{}", BASE_URL, id).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }
}

pub struct ListCategories {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for ListCategories {
    type T = Vec<Category>;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send_paginate::<Category, ListCategories>(self).await
    }

    fn get_url(&self) -> &String {
        &self.url
    }

    fn get_params(&self) -> &Vec<(String, String)> {
        &self.params
    }

    fn get_api_key(&self) -> &String {
        &self.api_key
    }
}

impl ListCategories {
    pub fn new(api_key: &String) -> ListCategories {
        return ListCategories {
            url: format!("{}/categories", BASE_URL).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }

    pub fn parent(mut self, parent: String) -> ListCategories {
        self.params
            .push(("filter[parent]".to_string(), parent));
        return self;
    }
}