use axum::{routing::post, Router};
use manifest::parse_manifest;

mod manifest;

pub fn route(router: Router) -> Router {
    router.route("/5/manifest", post(parse_manifest))
}
