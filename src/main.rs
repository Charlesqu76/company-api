use std::net::SocketAddr;
mod models;
mod routes;
use axum::{Router, Server};
use mongodb::Client;

#[tokio::main]
async fn main() {
    let client: Client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let app: Router<Client> = Router::new()
        .merge(routes::types::create_route())
        .merge(routes::productions::create_route());
    let app: Router = app.with_state(client);

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3001));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
