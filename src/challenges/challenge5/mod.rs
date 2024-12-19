use axum::{
    routing::{get, post},
    Router,
};
use endpoints::{unwrap, wrap};

mod endpoints;

pub fn router() -> Router {
    Router::new()
        .route("/wrap", post(wrap))
        .route("/unwrap", get(unwrap))
}
