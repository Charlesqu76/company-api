use std::net::SocketAddr;
mod models;
mod routes;
use axum::{response::Html, routing::get, Router, Server};
use mongodb::Client;

async fn handle_test() -> Html<String> {
    println!("test");
    Html("<div>test</div>".to_string())
}

#[tokio::main]
async fn main() {
    println!("start");
    let client: Client = Client::with_uri_str("mongodb://43.143.254.158:27017")
        .await
        .unwrap();
    let app: Router<Client> = Router::new()
        .merge(routes::types::create_route())
        .merge(routes::productions::create_route())
        .route("/test", get(handle_test));
    let app: Router = app.with_state(client);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3002));

    println!("start server");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
