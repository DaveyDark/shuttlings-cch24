use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use board::Board;
use endpoints::{get_board, place_tile, reset_board};
use tokio::sync::RwLock;

mod board;
mod endpoints;

type AppState = Arc<RwLock<Board>>;

pub fn router() -> Router {
    let state = Arc::new(RwLock::new(Board::new()));

    Router::new()
        .route("/board", get(get_board))
        .route("/reset", post(reset_board))
        .route("/place/:team/:col", post(place_tile))
        .with_state(state)
}
