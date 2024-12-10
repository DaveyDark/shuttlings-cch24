use axum::{routing::get, Router};
use ipv4::{dest, key};
use ipv6::{dest_v6, key_v6};

mod ipv4;
mod ipv6;

#[derive(serde::Deserialize)]
struct KeyQuery {
    from: String,
    to: String,
}

#[derive(serde::Deserialize)]
struct DestQuery {
    from: String,
    key: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/dest", get(dest))
        .route("/key", get(key))
        .route("/v6/dest", get(dest_v6))
        .route("/v6/key", get(key_v6))
}
