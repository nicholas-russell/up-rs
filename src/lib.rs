//! # Up Bank API wrapper
//! This crate is an API wrapper for the [Up Bank API](https://developer.up.com.au/).
//! ## Example Usage
//! ```
//! use uprs::api_endpoints::ListAccounts;
//! use uprs::models::Account;
//! use uprs::request_sender::ApiRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key: String = "$your_access_token".parse().unwrap();
//!
//!     let list_accounts: Vec<Account> = ListAccounts::new(&api_key).send().await.unwrap();
//!
//!     for account in list_accounts {
//!         println!("{}: ${}", account.attributes.display_name, account.attributes.balance.value)
//!     }
//! }
//! ```

/// Contains the different models as structs used in the API (e.g. Account, Transaction).
pub mod models;
/// Contains structs that represent the endpoints within the API.
pub mod api_endpoints;
/// Contains structs and traits that handle sending requests to the API.
pub mod request_sender;
