use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    routing::post,
    Json, Router,
};
use mongodb::{
    bson::{doc, Document},
    Client,
};
use serde_json::{json, Value};

use crate::models::{DelModel, DelTypes, Model};

pub fn create_route() -> Router<Client> {
    Router::new()
        .route("/getProducts", post(get_products))
        .route("/getItem", post(get_item))
        .route("/addProducts", post(add_products))
        .route("/delProducts", post(del_products))
}

async fn add_products(State(client): State<Client>, Json(params): Json<Model>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Model> = database.collection("models");
    let this_one = collection
        .find_one(
            doc! {"t_eng": &params.t_eng, "name_eng": &params.name_eng },
            None,
        )
        .await
        .unwrap()
        .is_some();
    if this_one {
        return Json(json!({ "data": "exist" }));
    };
    collection.insert_one(params, None).await.unwrap();
    Json(json!({ "data": "success" }))
}

async fn get_products(State(client): State<Client>, Json(params): Json<DelTypes>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Model> = database.collection("models");
    let info_doc: Document = doc! {
        "t_eng": params.t_eng,
    };
    let mut cursor: mongodb::Cursor<Model> = collection.find(info_doc, None).await.unwrap();
    let mut res: Vec<Model> = Vec::new();
    while cursor.advance().await.unwrap() {
        let temp: Model = cursor.deserialize_current().unwrap();
        res.push(temp);
    }
    Json(json!({ "data": res }))
}

async fn get_item(State(client): State<Client>, Json(params): Json<DelModel>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Model> = database.collection("models");
    let info_doc: Document = doc! {
        "t_eng": params.t_eng,
        "name_eng": params.name_eng,
    };
    let cursor: mongodb::Cursor<Model> = collection.find(info_doc, None).await.unwrap();
    let res: Model = cursor.deserialize_current().unwrap();
    Json(json!({ "data": res }))
}

async fn del_products(State(client): State<Client>, Json(params): Json<DelModel>) -> Json<Value> {
    let database: mongodb::Database = client.database("company");
    let collection: mongodb::Collection<Model> = database.collection("models");
    let del_doc: Document = doc! {"t_eng": params.t_eng,"name_eng": params.name_eng };
    collection.find_one_and_delete(del_doc, None).await.unwrap();
    Json(json!({ "data": "success" }))
}
