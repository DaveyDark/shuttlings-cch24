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

pub fn route(router: Router) -> Router {
    router
        .route("/2/dest", get(dest))
        .route("/2/key", get(key))
        .route("/2/v6/dest", get(dest_v6))
        .route("/2/v6/key", get(key_v6))
}
