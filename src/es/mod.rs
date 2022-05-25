use db::es::{create_index_if_not_exists, es_operation, es_query, get_client, update_mapping};
use models::products::{self, Product};
use serde_json::{json, Value};
use std::any::Any;

pub async fn init() {
    create_product_index().await;
    create_user_index().await;
}

async fn create_product_index() {
    let client = get_client();
    create_index_if_not_exists(&client, "products").await;
    update_mapping(
        &client,
        json!({
            "properties" : {
                "id" : { "type" : "text" },
                "name" : { "type" : "text" },
                "description" : { "type" : "text" },
                "quantity" : { "type" : "integer" },
                "stock" : { "type" : "integer" },
                "created_at" : { "type" : "string" },
                "seller" : { "type" : "text" },
            }
        }),
        "products",
    )
    .await;
}

async fn create_user_index() {
    let client = get_client();
    create_index_if_not_exists(&client, "users").await;
    update_mapping(
        &client,
        json!({
            "properties" : {
                "email" : { "type" : "text" },
                "name" : { "type" : "text" },
            }
        }),
        "users",
    )
    .await;
}

pub async fn insert_record(data: serde_json::Value, index_name: &str) {
    let client = get_client();
    es_operation(&client, data, index_name).await;
}

pub async fn get_record(data: serde_json::Value, index_name: &str) -> Vec<Product> {
    let client = get_client();
    let res = es_query(&client, data, index_name).await.unwrap();
    let json: Value = res.json().await.unwrap();
    let records: Vec<Product> = json["hits"]["hits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| serde_json::from_value(h["_source"].clone()).unwrap())
        .collect();
    records
}
