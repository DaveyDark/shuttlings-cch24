use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use board::Board;
use endpoints::{get_board, place_tile, random_board, reset_board};
use rand::{rngs::StdRng, SeedableRng};
use tokio::sync::RwLock;

mod board;
mod endpoints;

#[derive(Clone)]
struct FactoryState {
    board: Board,
    rng: rand::rngs::StdRng,
}

type AppState = Arc<RwLock<FactoryState>>;

pub fn router() -> Router {
    let rng = StdRng::seed_from_u64(2024);
    let f_state = FactoryState {
        board: Board::new(),
        rng,
    };
    let state = Arc::new(RwLock::new(f_state));

    Router::new()
        .route("/board", get(get_board))
        .route("/reset", post(reset_board))
        .route("/random-board", get(random_board))
        .route("/place/:team/:col", post(place_tile))
        .with_state(state)
}
