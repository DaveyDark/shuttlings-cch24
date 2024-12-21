use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Error, PgPool,
};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteData {
    pub author: Option<String>,
    pub quote: Option<String>,
}

pub async fn cite_quote(pool: &PgPool, id: Uuid) -> Result<Quote, Error> {
    // Return Quote
    sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn remove_quote(pool: &PgPool, id: Uuid) -> Result<Quote, Error> {
    // Check if row exists
    sqlx::query("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    println!("delete: {}", id.to_string());
    // Remove delete
    sqlx::query_as::<_, Quote>("DELETE FROM quotes WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn undo_quote(
    pool: &PgPool,
    id: Uuid,
    author: Option<String>,
    quote: Option<String>,
) -> Result<Quote, Error> {
    // Replace missing fields with existing fields if they exist
    let existing_quote = cite_quote(pool, id.clone()).await?;
    let author = author.unwrap_or(existing_quote.author);
    let quote = quote.unwrap_or(existing_quote.quote);
    // Update quote
    sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET author = $1, quote = $2, version=version+1,
        created_at=CURRENT_TIMESTAMP WHERE id = $3 RETURNING *",
    )
    .bind(author)
    .bind(quote)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn draft_quote(pool: &PgPool, author: String, quote: String) -> Result<Quote, Error> {
    // Generate ID
    let id = Uuid::new_v4();
    println!("add: {}", id.to_string());
    // Insert new quote
    sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (id, author, quote) VALUES ($1,$2,$3) RETURNING *",
    )
    .bind(id)
    .bind(author)
    .bind(quote)
    .fetch_one(pool)
    .await
}

pub async fn get_quotes_list(pool: &PgPool, page: i32) -> Option<Vec<Quote>> {
    // Calculate offset
    let offset = (page - 1) * 3;
    // Get quotes; Limits to 4 instead of 3 to check if there are more pages
    let quotes = sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes ORDER BY version DESC, created_at ASC OFFSET $1 LIMIT 4",
    )
    .bind(offset)
    .fetch_all(pool)
    .await
    .ok()?;
    if quotes.is_empty() {
        return None;
    }
    Some(quotes)
}
