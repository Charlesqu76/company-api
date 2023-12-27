use std::net::SocketAddr;
use std::time::Duration;
mod models;
mod routes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::{http::Request, middleware::Next, response::Response};
use axum::{response::Html, routing::get, Router, Server};
use mongodb::Client;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse};
use tower_http::LatencyUnit;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn handle_test() -> Html<String> {
    Html("<div>test</div>".to_string())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

#[tokio::main]
async fn main() {
    println!("start");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client: Client = Client::with_uri_str("mongodb://43.143.254.158:27017")
        .await
        .unwrap();
    let app: Router<Client> = Router::new()
        .merge(routes::types::create_route())
        .merge(routes::productions::create_route())
        .route("/test", get(handle_test))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        tracing::debug!("something went wrong")
                    },
                ),
        )
        .fallback(fallback);

    let app: Router = app.with_state(client);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3002));

    println!("start server");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
