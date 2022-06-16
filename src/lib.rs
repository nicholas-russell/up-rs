mod query_builder;
mod types;
use reqwest;
use serde;

use serde_json::json;
use types::{Response,Account,Category,Tag,Transaction,PingSuccessful,PingNotAuthorized};
use serde_json::Result;

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
