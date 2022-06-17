use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub(crate) data : T,
    links: Option<HashMap<String, Option<String>>>
}

#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "type")]
    resource_type: String,
    pub(crate) id: String,
    pub(crate) attributes: AccountAttributes,
    relationships: AccountRelationships,
    links: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
struct AccountRelationships {
    transactions: AccountTransactionsRelationships
}

#[derive(Deserialize, Debug)]
pub(crate) struct AccountAttributes {
    #[serde(rename = "displayName")]
    pub(crate) display_name: String,
    #[serde(rename = "accountType")]
    account_type: AccountType,
    #[serde(rename = "ownershipType")]
    ownership_type: OwnershipType,
    balance: Money,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
enum AccountType {
    SAVER,
    TRANSACTIONAL,
}

#[derive(Deserialize, Debug)]
enum OwnershipType {
    INDIVIDUAL,
    JOINT,
}

#[derive(Deserialize, Debug)]
struct Money {
    #[serde(rename = "currencyCode")]
    currency_code: String,
    value: String,
    #[serde(rename = "valueInBaseUnits")]
    value_in_base_units: i128
}

#[derive(Deserialize, Debug)]
struct AccountTransactionsRelationships {
    links: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub(crate) struct Category {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
    attributes: CategoryAttributes,
    relationships: CategoryRelationships,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct CategoryAttributes {
    name: String
}

#[derive(Deserialize, Debug)]
struct CategoryRelationships {
    parent: ParentRelationship,
    children: ChildRelationship
}

#[derive(Deserialize, Debug)]
struct ChildRelationship {
    data: Option<Vec<HashMap<String, String>>>,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct ParentRelationship {
    data: Option<HashMap<String, String>>,
    links: Option<HashMap<String, String>>
}

#[derive(Serialize, Debug)]
struct Payload<T> {
    data: T
}

#[derive(Serialize, Debug)]
struct CategorizeTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
pub(crate) struct Tag {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
struct TagRelationships {
    transactions: TransactionRelationships
}

#[derive(Serialize, Debug)]
struct CategorizeTransaction {
    data: CategorizeTransactionData
}

#[derive(Serialize, Debug)]
struct TagTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
pub(crate) struct Transaction {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
    attributes: TransactionAttributes,
    relationships: TransactionRelationships,
    links: Option<HashMap<String,String>>
}

#[derive(Deserialize, Debug)]
struct TransactionAttributes {
    status: TransactionStatus,
    #[serde(rename = "rawText")]
    raw_text: Option<String>,
    description: String,
    message: Option<String>,
    #[serde(rename = "isCategorizable")]
    is_categorizable: bool,
    #[serde(rename = "holdInfo")]
    hold_info: Option<HoldInfo>,
    #[serde(rename = "roundUp")]
    round_up: Option<RoundUp>,
    cashback: Option<Cashback>,
    amount: Money,
    #[serde(rename = "foreignAmount")]
    foreign_amount: Option<Money>,
    #[serde(rename = "cardPurchaseMethod")]
    card_purchase_method: Option<CardPurchaseMethod>,
    #[serde(rename = "settledAt")]
    settled_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
enum TransactionStatus {
    HELD,
    SETTLED,
}

#[derive(Deserialize, Debug)]
struct HoldInfo {
    amount: Money,
    #[serde(rename = "foreignAmount")]
    foreign_amount: Option<Money>
}

#[derive(Deserialize, Debug)]
struct RoundUp {
    amount: Money,
    #[serde(rename = "boostPortion")]
    boost_portion: Option<Money>
}

#[derive(Deserialize, Debug)]
struct Cashback {
    description: String,
    amount: Money
}

#[derive(Deserialize, Debug)]
struct CardPurchaseMethod {
    method: CardPurchaseMethodType,
    #[serde(rename = "cardNumberSuffix")]
    card_number_suffix: Option<String>
}

#[derive(Deserialize, Debug)]
enum CardPurchaseMethodType {
    BAR_CODE,
    OCR,
    CARD_PIN,
    CARD_DETAILS,
    CARD_ON_FILE,
    ECOMMERCE,
    MAGNETIC_STRIPE,
    CONTACTLESS
}

#[derive(Deserialize, Debug)]
struct TransactionRelationships {
    account: TransactionAccountRelationship,
    #[serde(rename = "transferAccount")]
    transfer_account: TransactionTransferAccountRelationship,
    category: TransactionCategoryRelationship,
    #[serde(rename= "parentCategory")]
    parent_category: TransactionCategoryRelationship,
    tags: TransactionTagsRelationship
}

#[derive(Deserialize, Debug)]
struct TransactionAccountRelationship {
    data: TransactionAccountRelationshipData,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct TransactionAccountRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
struct TransactionTransferAccountRelationship {
    data: Option<TransactionTransferAccountRelationshipData>,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct TransactionTransferAccountRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
struct TransactionCategoryRelationship {
    data: Option<TransactionCategoryRelationshipData>,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct TransactionCategoryRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
struct TransactionTagsRelationship {
    data: Vec<TransactionTagsRelationshipData>,
    links: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
struct TransactionTagsRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String
}

#[derive(Deserialize, Debug)]
pub(crate) struct PingSuccessful {
    meta: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
pub(crate) struct PingNotAuthorized {
    errors: Vec<HashMap<String, String>>
}