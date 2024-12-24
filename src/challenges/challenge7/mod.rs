use crate::challenge7::ornament::ornament;
use crate::challenge7::present::present;
use axum::{
    routing::{get, post},
    Router,
};
use lockfile::lockfile;
use star::star;

mod lockfile;
mod ornament;
mod present;
mod star;

pub fn router() -> Router {
    Router::new()
        .route("/star", get(star))
        .route("/present/:color", get(present))
        .route("/ornament/:state/:id", get(ornament))
        .route("/lockfile", post(lockfile))
}
