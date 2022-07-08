#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset};
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use uprs::types::*;

    #[tokio::test]
    async fn account_deserialization() {
        let json: String = fs::read_to_string("tests/example_json/list_accounts.json").unwrap();
        let des: ApiResponse<Vec<Account>> = serde_json::from_str(&*json).unwrap();

        assert_eq!(
            des,
            ApiResponse {
               data: vec![
                   Account {
                       resource_type: "accounts".to_string(),
                       id: "1bcc9d36-ccdf-457d-9e40-cecde788abb4".to_string(),
                       attributes: AccountAttributes {
                           display_name: "Spending".to_string(),
                           account_type: AccountType::Transactional,
                           ownership_type: OwnershipType::Individual,
                           balance: Money {
                               currency_code: "AUD".to_string(),
                               value: "1.00".to_string(),
                               value_in_base_units: 100
                           },
                           created_at: DateTime::<FixedOffset>::parse_from_rfc3339("2022-07-05T17:52:25+10:00")
                               .unwrap()
                       },
                       relationships: AccountRelationships {
                           transactions: AccountTransactionsRelationships {
                               links: HashMap::<String, String>::from([("related".to_string(), "https://api.up.com.au/api/v1/accounts/1bcc9d36-ccdf-457d-9e40-cecde788abb4/transactions".to_string())])
                           }
                       },
                       links: HashMap::<String, String>::from([("self".to_string(), "https://api.up.com.au/api/v1/accounts/1bcc9d36-ccdf-457d-9e40-cecde788abb4".to_string())])
                   }
                ],
                links: Some(HashMap::<String, Option<String>>::from([
                    ("prev".to_string(), None),
                    ("next".to_string(), Some("https://api.up.com.au/api/v1/accounts?page%5Bafter%5D=WyIyMDIyLTA3LTA1VDA3OjUyOjI1LjI5NDU0NjAwMFoiLCIxYmNjOWQzNi1jY2RmLTQ1N2QtOWU0MC1jZWNkZTc4OGFiYjQiXQ%3D%3D&page%5Bsize%5D=1".to_string()))
                ]))
            }
        )
    }

    #[tokio::test]
    async fn category_deserialization() {
        let json: String = fs::read_to_string("tests/example_json/list_categories.json").unwrap();
        let des: ApiResponse<Vec<Category>> = serde_json::from_str(&*json).unwrap();

        assert_eq!(
            des,
            ApiResponse {
                data: vec![
                        Category {
                            resource_type: "categories".to_string(),
                            id: "hobbies".to_string(),
                            attributes: CategoryAttributes {
                                name: "Hobbies".to_string()
                            },
                            relationships: CategoryRelationships {
                                parent: ParentRelationship {
                                    data: Some(HashMap::<String, String>::from([
                                        ("id".to_string(), "good-life".to_string()),
                                        ("type".to_string(), "categories".to_string())
                                    ])),
                                    links: Some(HashMap::<String, String>::from([
                                        ("related".to_string(), "https://api.up.com.au/api/v1/categories/good-life".to_string())
                                    ])),
                                },
                                children: ChildRelationship {
                                    data: Some(vec![]),
                                    links: Some(HashMap::<String, String>::from([
                                        ("related".to_string(), "https://api.up.com.au/api/v1/categories?filter%5Bparent%5D=hobbies".to_string())
                                    ])),
                                }
                            },
                            links: Some(HashMap::<String, String>::from([
                                ("self".to_string(), "https://api.up.com.au/api/v1/categories/hobbies".to_string())
                            ])),
                    },
                    Category {
                        resource_type: "categories".to_string(),
                        id: "restaurants-and-cafes".to_string(),
                        attributes: CategoryAttributes {
                            name: "Restaurants & Cafes".to_string()
                        },
                        relationships: CategoryRelationships {
                            parent: ParentRelationship {
                                data: Some(HashMap::<String, String>::from([
                                    ("type".to_string(), "categories".to_string()),
                                    ("id".to_string(), "good-life".to_string())
                                ])),
                                links: Some(HashMap::<String, String>::from([
                                    ("related".to_string(), "https://api.up.com.au/api/v1/categories/good-life".to_string())
                                ])),
                            },
                            children: ChildRelationship {
                                data: Some(vec![]),
                                links: Some(HashMap::<String, String>::from([
                                    ("related".to_string(), "https://api.up.com.au/api/v1/categories?filter%5Bparent%5D=restaurants-and-cafes".to_string())
                                ])),
                            }
                        },
                        links: Some(HashMap::<String, String>::from([
                            ("self".to_string(), "https://api.up.com.au/api/v1/categories/restaurants-and-cafes".to_string())
                        ])),
                    },
                ],
                links: None
            }
        )
    }

    #[tokio::test]
    async fn tag_deserialization() {
        let json: String = fs::read_to_string("tests/example_json/list_tags.json").unwrap();
        let des: ApiResponse<Vec<Tag>> = serde_json::from_str(&*json).unwrap();

        assert_eq!(
            des,
            ApiResponse { data: vec![
                Tag {
                    resource_type: "tags".to_string(),
                    id: "Holiday".to_string(),
                    relationships: TagRelationships { transactions: TagTransactionRelationships { links: HashMap::<String, String>::from([
                        ("related".to_string(), "https://api.up.com.au/api/v1/transactions?filter%5Btag%5D=Holiday".to_string())
                    ]) } }
                },
                Tag {
                    resource_type: "tags".to_string(),
                    id: "Pizza Night".to_string(),
                    relationships: TagRelationships { transactions: TagTransactionRelationships { links: HashMap::<String, String>::from([
                        ("related".to_string(), "https://api.up.com.au/api/v1/transactions?filter%5Btag%5D=Pizza+Night".to_string())
                    ]) } }
                }
            ], links: Some(HashMap::<String, Option<String>>::from([
                ("prev".to_string(), None),
                ("next".to_string(), Some("https://api.up.com.au/api/v1/tags?page%5Bafter%5D=WyJQaXp6YSBOaWdodCJd&page%5Bsize%5D=2".to_string()))
                ])) }
        )
    }

    #[tokio::test]
    async fn transaction_deserialization() {
        let json: String = fs::read_to_string("tests/example_json/list_transactions.json").unwrap();
        let des: ApiResponse<Vec<Transaction>> = serde_json::from_str(&*json).unwrap();

        assert_eq!(
            des,
            ApiResponse {
                data: vec![Transaction {
                    resource_type: "transactions".to_string(),
                    id: "13a25bb2-7290-43ce-a9e8-c9901c58895d".to_string(),
                    attributes: TransactionAttributes {
                        status: TransactionStatus::Settled,
                        raw_text: None,
                        description: "David Taylor".to_string(),
                        message: Some("Money for the pizzas last night.".to_string()),
                        is_categorizable: true,
                        hold_info: None,
                        round_up: None,
                        cashback: None,
                        amount: Money {
                            currency_code: "AUD".to_string(),
                            value: "-59.98".to_string(),
                            value_in_base_units: -5998
                        },
                        foreign_amount: None,
                        card_purchase_method: None,
                        settled_at: Some(
                            DateTime::<FixedOffset>::parse_from_rfc3339(
                                "2022-07-07T05:20:36+10:00"
                            )
                            .unwrap()
                        ),
                        created_at: DateTime::<FixedOffset>::parse_from_rfc3339(
                            "2022-07-07T05:20:36+10:00"
                        )
                        .unwrap()
                    },
                    relationships: TransactionRelationships {
                        account: TransactionAccountRelationship {
                            data: TransactionAccountRelationshipData {
                                resource_type: "accounts".to_string(),
                                id: "0707a248-76c7-4fb8-8858-18b6489fded3".to_string()
                            },
                            links: Some(HashMap::<String, String>::from([
                                ("related".to_string(), "https://api.up.com.au/api/v1/accounts/0707a248-76c7-4fb8-8858-18b6489fded3".to_string())
                            ]))
                        },
                        transfer_account: TransactionTransferAccountRelationship {
                            data: None,
                            links: None
                        },
                        category: TransactionCategoryRelationship {
                            data: None,
                            links: Some(HashMap::<String, String>::from([
                                ("self".to_string(), "https://api.up.com.au/api/v1/transactions/13a25bb2-7290-43ce-a9e8-c9901c58895d/relationships/category".to_string())
                            ]))
                        },
                        parent_category: TransactionCategoryRelationship {
                            data: None,
                            links: None
                        },
                        tags: TransactionTagsRelationship {
                            data: vec![
                                TransactionTagsRelationshipData { resource_type: "tags".to_string(), id: "Pizza Night".to_string() }
                            ],
                            links: Some(HashMap::<String, String>::from([
                                ("self".to_string(), "https://api.up.com.au/api/v1/transactions/13a25bb2-7290-43ce-a9e8-c9901c58895d/relationships/tags".to_string())
                            ]))
                        }
                    },
                    links: Some(HashMap::<String, String>::from([
                        ("self".to_string(), "https://api.up.com.au/api/v1/transactions/13a25bb2-7290-43ce-a9e8-c9901c58895d".to_string())
                    ]))
                }],
                links: Some(HashMap::<String, Option<String>>::from([
                    ("prev".to_string(), None),
                    ("next".to_string(), None)
                ]))
            }
        )
    }
}
