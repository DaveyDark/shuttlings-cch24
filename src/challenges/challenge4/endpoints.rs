use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use super::{board::Board, AppState};

pub async fn get_board(State(state): State<AppState>) -> (StatusCode, String) {
    // Return board as a string
    let board = state.read().await;
    (StatusCode::OK, board.to_string())
}

pub async fn reset_board(State(state): State<AppState>) -> (StatusCode, String) {
    // Reset board to a new one
    let mut board = state.write().await;
    *board = Board::new();
    (StatusCode::OK, board.to_string())
}

pub async fn place_tile(
    State(state): State<AppState>,
    Path((team, col)): Path<(String, usize)>,
) -> (StatusCode, String) {
    // Get write access to board
    let mut board = state.write().await;

    // Check if board is already complete
    if let Some(_) = board.get_result() {
        return (StatusCode::SERVICE_UNAVAILABLE, board.to_string());
    }

    // Try to place tile
    if let Err(err) = board.place_tile(team, col) {
        return match err.as_str() {
            // Return generated error
            "Invalid team" | "Invalid column" => (StatusCode::BAD_REQUEST, board.to_string()),
            "Column full" => (StatusCode::SERVICE_UNAVAILABLE, board.to_string()),
            // Unexpected error
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error: ".to_string() + &err,
            ),
        };
    }

    // Check if board is complete
    let result = board.get_result();

    if let Some(_) = result {
        (StatusCode::OK, board.to_string())
    } else {
        (StatusCode::OK, board.to_string())
    }
}
