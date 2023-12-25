use std::collections::HashMap;

use axum::{
    extract::{self, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use mongodb::{
    bson::{doc, Document},
    Client,
};
use serde_json::{json, Value};

use crate::models::{DelTypes, Model, Types};

pub fn create_route() -> Router<Client> {
    Router::new()
        .route("/gettypes", post(get_types))
        .route("/addtype", post(add_type))
        .route("/deltype", post(del_type))
}

async fn get_types(State(client): State<Client>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Types> = database.collection("types");
    let mut cursor: mongodb::Cursor<Types> = collection.find(doc! {}, None).await.unwrap();
    let mut types_list: Vec<Types> = vec![];
    while cursor.advance().await.unwrap() {
        let temp: Types = cursor.deserialize_current().unwrap();
        types_list.push(temp);
    }
    Json(json!({ "data": types_list }))
}

async fn add_type(State(client): State<Client>, Json(params): Json<Types>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Types> = database.collection("types");
    let cursor: Option<Types> = collection
        .find_one(doc! { "t_eng": &params.t_eng }, None)
        .await
        .unwrap();
    if cursor.is_some() {
        Json(json!({ "data": "exist" }))
    } else {
        collection.insert_one(params, None).await.unwrap();
        Json(json!({ "data": "success" }))
    }
}

async fn del_type(State(client): State<Client>, Json(params): Json<DelTypes>) {
    let database: mongodb::Database = client.database("company");

    let collection: mongodb::Collection<Types> = database.collection("types");
    let d: Document = doc! {"t_eng": &params.t_eng,};
    collection
        .find_one_and_delete(d.clone(), None)
        .await
        .unwrap();

    let collection: mongodb::Collection<Model> = database.collection("models");
    collection.find_one_and_delete(d, None).await.unwrap();
}
