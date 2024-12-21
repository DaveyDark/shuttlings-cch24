use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Error, PgPool};

use crate::challenges::challenge6::token::validate_token;

use super::{
    db::reset_db,
    quotes::{
        cite_quote, draft_quote, get_quotes_list, remove_quote, undo_quote, Quote, QuoteData,
    },
    token::{advance_token, discard_token, generate_token},
};

pub async fn reset(State(pool): State<PgPool>) -> Result<(), StatusCode> {
    reset_db(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
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

#[derive(Serialize, Debug, Clone)]
pub struct ListResponse {
    quotes: Vec<Quote>,
    page: i32,
    next_token: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenQuery {
    token: Option<String>,
}

pub async fn list(
    State(pool): State<PgPool>,
    Query(token): Query<TokenQuery>,
) -> Result<Json<ListResponse>, StatusCode> {
    let mut token = token.token;
    if let Some(t) = token.clone() {
        // Token is provided, validate it
        if t.len() != 16 || t.chars().into_iter().any(|c| !c.is_alphanumeric()) {
            return Err(StatusCode::BAD_REQUEST);
        }
        validate_token(&pool, &t)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
    } else {
        // Token is not provided, generate one
        token = Some(
            generate_token(&pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .token,
        );
    }
    // Token is valid, get quotes
    let token = token.unwrap();
    let page = advance_token(&pool, token.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut quotes = get_quotes_list(&pool, page)
        .await
        .ok_or(StatusCode::NO_CONTENT)?;

    // Check if token is last
    let mut next_token = None;
    if quotes.len() >= 4 {
        // Token is not last
        next_token = Some(token);
        quotes.pop();
    } else {
        // Discard token
        discard_token(&pool, &token)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    // Return response
    let resp = ListResponse {
        quotes,
        page,
        next_token,
    };
    Ok(Json(resp))
}
