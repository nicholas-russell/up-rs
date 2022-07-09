
    use crate::request_sender::*;
    use crate::models::*;
    use async_trait::async_trait;
    use chrono::{DateTime, FixedOffset};

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
    pub fn new(api_key: &String, account_id: String) -> RetrieveAccount {
        return RetrieveAccount {
            url: format!("{}/accounts/{}", BASE_URL, account_id).to_string(),
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

    pub fn parent(mut self, parent: Category) -> ListCategories {
        self.params
            .push(("filter[parent]".to_string(), parent.to_param().to_string()));
        return self;
    }
}

pub struct RetrieveCategory {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for RetrieveCategory {
    type T = Category;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send::<Category, RetrieveCategory>(self).await
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

impl RetrieveCategory {
    pub fn new(api_key: &String, category_id: String) -> RetrieveCategory {
        return RetrieveCategory {
            url: format!("{}/categories/{}", BASE_URL.to_string(), category_id).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }
}
/* Todo
struct CategorizeTransaction {}

impl CategorizeTransaction {}
*/
pub struct ListTags {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for ListTags {
    type T = Vec<Tag>;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send_paginate::<Tag, ListTags>(self).await
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

impl ListTags {
    pub fn new(api_key: &String) -> ListTags {
        return ListTags {
            url: format!("{}/tags", BASE_URL).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }

    pub fn page_size(mut self, page_size: i32) -> ListTags {
        if page_size > 0 && page_size <= 50 {
            self.params
                .push(("page[size]".to_string(), page_size.to_string()));
        } else {
            eprintln!("Page size has to be between 1 and 50.");
        }
        return self;
    }
}
/* Todo
struct AddTagToTransaction {}

impl AddTagToTransaction {}

struct RemoveTagFromTransaction {}

impl RemoveTagFromTransaction {}
*/
pub struct ListTransactions {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for ListTransactions {
    type T = Vec<Transaction>;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send_paginate::<Transaction, ListTransactions>(self).await
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

impl ListTransactions {
    pub fn new(api_key: &String) -> ListTransactions {
        return ListTransactions {
            url: format!("{}/transactions", BASE_URL).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }

    pub fn page_size(mut self, page_size: i32) -> ListTransactions {
        if page_size > 0 && page_size <= 30 {
            self.params
                .push(("page[size]".to_string(), page_size.to_string()));
        } else {
            eprintln!("Page size has to be between 1 and 30.");
        }
        return self;
    }

    pub fn status(mut self, transaction_status: TransactionStatus) -> ListTransactions {
        self.params
            .push(("filter[status]".to_string(), transaction_status.to_string()));
        self
    }

    pub fn since(mut self, date_time: DateTime<FixedOffset>) -> ListTransactions {
        self.params
            .push(("filter[since]".to_string(), date_time.to_rfc3339()));
        self
    }

    pub fn until(mut self, date_time: DateTime<FixedOffset>) -> ListTransactions {
        self.params
            .push(("filter[until]".to_string(), date_time.to_rfc3339()));
        self
    }

    pub fn category(mut self, category: Category) -> ListTransactions {
        self.params.push((
            "filter[category]".to_string(),
            category.to_param().to_string(),
        ));
        self
    }

    pub fn tag(mut self, tag: Tag) -> ListTransactions {
        self.params
            .push(("filter[tag".to_string(), tag.to_param().to_string()));
        self
    }
}

pub struct RetrieveTransaction {
    url: String,
    api_key: String,
    params: Vec<(String, String)>,
}

#[async_trait]
impl ApiRequest for RetrieveTransaction {
    type T = Transaction;

    async fn send(self) -> Result<Self::T, String> {
        RequestSender::send::<Transaction, RetrieveTransaction>(self).await
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

impl RetrieveTransaction {
    pub fn new(api_key: &String, transaction_id: String) -> RetrieveTransaction {
        return RetrieveTransaction {
            url: format!("{}/transactions/{}", BASE_URL, transaction_id).to_string(),
            api_key: api_key.to_string(),
            params: Vec::new(),
        };
    }
}
