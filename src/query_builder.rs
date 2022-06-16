use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://api.up.com.au/api";
const API_VER: &str = "v1";

async fn build<T>(url:String) -> Result<T, StatusCode>
where
    T:DeserializeOwned,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let content = response.unwrap().json::<T>().await;

    return match content {
        Ok(s) => Ok(s),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}