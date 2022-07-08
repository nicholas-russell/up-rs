use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ApiResponse<T> {
    pub data: T,
    pub links: Option<HashMap<String, Option<String>>>,
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Account {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
    pub links: HashMap<String, String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountRelationships {
    pub transactions: AccountTransactionsRelationships,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountAttributes {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[serde(rename = "ownershipType")]
    pub ownership_type: OwnershipType,
    pub balance: Money,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug, strum_macros::Display, PartialEq, Eq)]
pub enum AccountType {
    #[serde(rename = "SAVER")]
    Saver,
    #[serde(rename = "TRANSACTIONAL")]
    Transactional,
}

#[derive(Deserialize, Debug, strum_macros::Display, PartialEq, Eq)]
pub enum OwnershipType {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "JOINT")]
    Joint,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Money {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub value_in_base_units: i128,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountTransactionsRelationships {
    pub links: HashMap<String, String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CategoryRelationships {
    pub parent: ParentRelationship,
    pub children: ChildRelationship,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ChildRelationship {
    pub data: Option<Vec<HashMap<String, String>>>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ParentRelationship {
    pub data: Option<HashMap<String, String>>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct Payload<T> {
    data: T,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct CategorizeTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct TagTransactionData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Tag {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub relationships: TagRelationships
}

impl Tag {
    pub(crate) fn to_param(&self) -> &String {
        &self.id
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TagRelationships {
    pub transactions: TagTransactionRelationships,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct CategorizeTransaction {
    data: CategorizeTransactionData,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct TagTransaction {
    data: TagTransactionData,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TagTransactionRelationships {
    pub links: HashMap<String, String>
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: TransactionAttributes,
    pub relationships: TransactionRelationships,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionAttributes {
    pub status: TransactionStatus,
    #[serde(rename = "rawText")]
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    #[serde(rename = "isCategorizable")]
    pub is_categorizable: bool,
    #[serde(rename = "holdInfo")]
    pub hold_info: Option<HoldInfo>,
    #[serde(rename = "roundUp")]
    pub round_up: Option<RoundUp>,
    pub cashback: Option<Cashback>,
    pub amount: Money,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<Money>,
    #[serde(rename = "cardPurchaseMethod")]
    pub card_purchase_method: Option<CardPurchaseMethod>,
    #[serde(rename = "settledAt")]
    pub settled_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug, strum_macros::Display, PartialEq, Eq)]
pub enum TransactionStatus {
    #[serde(rename="HELD")]
    Held,
    #[serde(rename="SETTLED")]
    Settled,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct HoldInfo {
    amount: Money,
    #[serde(rename = "foreignAmount")]
    foreign_amount: Option<Money>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct RoundUp {
    amount: Money,
    #[serde(rename = "boostPortion")]
    boost_portion: Option<Money>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Cashback {
    description: String,
    amount: Money,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CardPurchaseMethod {
    method: CardPurchaseMethodType,
    #[serde(rename = "cardNumberSuffix")]
    card_number_suffix: Option<String>,
}

#[derive(Deserialize, Debug, strum_macros::Display, PartialEq, Eq)]
pub enum CardPurchaseMethodType {
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionRelationships {
    pub account: TransactionAccountRelationship,
    #[serde(rename = "transferAccount")]
    pub transfer_account: TransactionTransferAccountRelationship,
    pub category: TransactionCategoryRelationship,
    #[serde(rename = "parentCategory")]
    pub parent_category: TransactionCategoryRelationship,
    pub tags: TransactionTagsRelationship,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionAccountRelationship {
    pub data: TransactionAccountRelationshipData,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionAccountRelationshipData {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionTransferAccountRelationship {
    pub data: Option<TransactionTransferAccountRelationshipData>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionTransferAccountRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionCategoryRelationship {
    pub data: Option<TransactionCategoryRelationshipData>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionCategoryRelationshipData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionTagsRelationship {
    pub data: Vec<TransactionTagsRelationshipData>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TransactionTagsRelationshipData {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct PingSuccessful {
    meta: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct PingNotAuthorized {
    errors: Vec<HashMap<String, String>>,
}
