pub mod api_client;
pub mod request_sender;
pub mod types;



#[cfg(test)]
mod tests {
    use crate::types;
    use std::fs;

    #[tokio::test]
    async fn response_has_next() {
        let json: String = fs::read_to_string("tests/example_json/list_accounts.json").unwrap();
        let des: types::ApiResponse<Vec<types::Account>> = serde_json::from_str(&*json).unwrap();
        assert_eq!(types::ApiResponse::has_next(&des), true);
    }
}