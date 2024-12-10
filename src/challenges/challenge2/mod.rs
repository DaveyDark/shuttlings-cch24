use axum::{routing::post, Router};
use manifest::parse_manifest;

mod manifest;

pub fn router() -> Router {
    Router::new().route("/manifest", post(parse_manifest))
}
