use crate::types::ApiResponse;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait ApiRequest {
    type T;
    async fn send(self) -> Result<Self::T, String>;
    fn get_url(&self) -> &String;
    fn get_params(&self) -> &Vec<(String, String)>;
    fn get_api_key(&self) -> &String;
}

pub(crate) struct RequestSender {}

impl RequestSender {
    pub(crate) async fn send_paginate<T: DeserializeOwned, K: ApiRequest>(
        base: K,
    ) -> Result<Vec<T>, String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base.get_url())
            .bearer_auth(base.get_api_key())
            .query(base.get_params())
            .send()
            .await
            .unwrap();

        return match res.status() {
            StatusCode::OK => {
                let mut rtn: Vec<T> = Vec::new();
                let mut json: ApiResponse<Vec<T>> = res.json().await.unwrap();
                rtn.append(&mut json.data);
                while ApiResponse::has_next(&json) {
                    let next = json
                        .links
                        .as_ref()
                        .unwrap()
                        .get("next")
                        .unwrap()
                        .to_owned()
                        .unwrap();
                    let client = reqwest::Client::new();
                    let res = client
                        .get(next)
                        .bearer_auth(base.get_api_key())
                        .send()
                        .await
                        .unwrap();
                    match res.status() {
                        StatusCode::OK => {
                            json = res.json().await.unwrap();
                            rtn.append(&mut json.data);
                        }
                        _ => {
                            return Err(res.text().await.unwrap());
                        }
                    }
                }
                Ok(rtn)
            }
            _ => Err(res.text().await.unwrap()),
        };
    }

    pub async fn send<T: DeserializeOwned, K: ApiRequest>(base: K) -> Result<T, String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base.get_url())
            .bearer_auth(base.get_api_key())
            .query(base.get_params())
            .send()
            .await
            .unwrap();

        return match res.status() {
            StatusCode::OK => {
                let json: ApiResponse<T> = res.json().await.unwrap();
                Ok(json.data)
            }
            _ => Err(res.text().await.unwrap()),
        };
    }
}
