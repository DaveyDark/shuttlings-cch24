use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{types::Uuid, Error, PgPool};

use super::quotes::{
    cite_quote, draft_quote, remove_quote, reset_quotes, undo_quote, Quote, QuoteData,
};

pub async fn reset(State(pool): State<PgPool>) -> Result<(), StatusCode> {
    reset_quotes(&pool).await.map_err(|e| {
        eprintln!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn cite(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Quote>, StatusCode> {
    // Convert ID
    let id = Uuid::from_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    cite_quote(&pool, id)
        .await
        .map(|q| Json(q))
        .map_err(|e| match e {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn remove(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Quote>, StatusCode> {
    // Convert ID
    let id = Uuid::from_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    remove_quote(&pool, id)
        .await
        .map(|q| Json(q))
        .map_err(|e| match e {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn undo(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(quote_data): Json<QuoteData>,
) -> Result<Json<Quote>, StatusCode> {
    // Convert ID
    let id = Uuid::from_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    undo_quote(&pool, id, quote_data.author, quote_data.quote)
        .await
        .map(|q| Json(q))
        .map_err(|e| match e {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn draft(
    State(pool): State<PgPool>,
    Json(quote_data): Json<QuoteData>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    if quote_data.author.is_none() || quote_data.quote.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    draft_quote(&pool, quote_data.author.unwrap(), quote_data.quote.unwrap())
        .await
        .map(|q| (StatusCode::CREATED, Json(q)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
