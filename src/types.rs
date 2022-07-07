use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub(crate) data: T,
    pub(crate) links: Option<HashMap<String, Option<String>>>,
}

impl<T> ApiResponse<T> {
    pub(crate) fn has_next(api_response: &ApiResponse<T>) -> bool {
        return match &api_response.links {
            None => false,
            Some(v) => match v.get("next") {
                None => false,
                Some(v2) => v2.is_some(),
            },
        };
    }
}

#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
    pub links: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct AccountRelationships {
    transactions: AccountTransactionsRelationships,
}

#[derive(Deserialize, Debug)]
pub struct AccountAttributes {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[serde(rename = "ownershipType")]
    pub ownership_type: OwnershipType,
    pub balance: Money,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum AccountType {
    SAVER,
    TRANSACTIONAL,
}

#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum OwnershipType {
    INDIVIDUAL,
    JOINT,
}

#[derive(Deserialize, Debug)]
pub struct Money {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub value_in_base_units: i128,
}

#[derive(Deserialize, Debug)]
struct AccountTransactionsRelationships {
    links: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: CategoryAttributes,
    pub relationships: CategoryRelationships,
    pub links: Option<HashMap<String, String>>,
}

impl Category {
    pub(crate) fn to_param(&self) -> &String {
        &self.id
    }
}

#[derive(Deserialize, Debug)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryRelationships {
    pub parent: ParentRelationship,
    pub children: ChildRelationship,
}

#[derive(Deserialize, Debug)]
pub struct ChildRelationship {
    pub data: Option<Vec<HashMap<String, String>>>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct ParentRelationship {
    pub data: Option<HashMap<String, String>>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
struct Payload<T> {
    data: T,
}

#[derive(Serialize, Debug)]
struct CategorizeTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

impl Tag {
    pub(crate) fn to_param(&self) -> &String {
        &self.id
    }
}

#[derive(Deserialize, Debug)]
struct TagRelationships {
    transactions: TransactionRelationships,
}

#[derive(Serialize, Debug)]
struct CategorizeTransaction {
    data: CategorizeTransactionData,
}

#[derive(Serialize, Debug)]
struct TagTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "type")]
    resource_type: String,
    pub id: String,
    attributes: TransactionAttributes,
    relationships: TransactionRelationships,
    links: Option<HashMap<String, String>>,
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

#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum TransactionStatus {
    HELD,
    SETTLED,
}

#[derive(Deserialize, Debug)]
struct HoldInfo {
    amount: Money,
    #[serde(rename = "foreignAmount")]
    foreign_amount: Option<Money>,
}

#[derive(Deserialize, Debug)]
struct RoundUp {
    amount: Money,
    #[serde(rename = "boostPortion")]
    boost_portion: Option<Money>,
}

#[derive(Deserialize, Debug)]
struct Cashback {
    description: String,
    amount: Money,
}

#[derive(Deserialize, Debug)]
struct CardPurchaseMethod {
    method: CardPurchaseMethodType,
    #[serde(rename = "cardNumberSuffix")]
    card_number_suffix: Option<String>,
}

#[derive(Deserialize, Debug, strum_macros::Display)]
enum CardPurchaseMethodType {
    #[serde(rename="BAR_CODE")]
    BarCode,
    #[serde(rename="OCR")]
    OCR,
    #[serde(rename="CARD_PIN")]
    CardPin,
    #[serde(rename="CARD_DETAILS")]
    CardDetails,
    #[serde(rename="CARD_ON_FILE")]
    CardOnFile,
    #[serde(rename="ECOMMERCE")]
    Ecommerce,
    #[serde(rename="MAGNETIC_STRIPE")]
    MagneticStripe,
    #[serde(rename="CONTACTLESS")]
    Contactless,
}

#[derive(Deserialize, Debug)]
struct TransactionRelationships {
    account: TransactionAccountRelationship,
    #[serde(rename = "transferAccount")]
    transfer_account: TransactionTransferAccountRelationship,
    category: TransactionCategoryRelationship,
    #[serde(rename = "parentCategory")]
    parent_category: TransactionCategoryRelationship,
    tags: TransactionTagsRelationship,
}

#[derive(Deserialize, Debug)]
struct TransactionAccountRelationship {
    data: TransactionAccountRelationshipData,
    links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct TransactionAccountRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
struct TransactionTransferAccountRelationship {
    data: Option<TransactionTransferAccountRelationshipData>,
    links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct TransactionTransferAccountRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
struct TransactionCategoryRelationship {
    data: Option<TransactionCategoryRelationshipData>,
    links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct TransactionCategoryRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
struct TransactionTagsRelationship {
    data: Vec<TransactionTagsRelationshipData>,
    links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct TransactionTagsRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PingSuccessful {
    meta: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PingNotAuthorized {
    errors: Vec<HashMap<String, String>>,
}
