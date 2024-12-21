use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Error, PgPool,
};

pub async fn init_db(pool: &PgPool) {
    // Create table
    match sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS quotes (
            id UUID PRIMARY KEY,
            author TEXT NOT NULL,
            quote TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            version INT NOT NULL DEFAULT 1
        ); ",
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            println!("Database Connected")
        }
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
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

pub async fn reset_quotes(pool: &PgPool) -> Result<(), Error> {
    // Delete all entries in table
    sqlx::query("DELETE FROM quotes;")
        .execute(pool)
        .await
        .map(|_| ())
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
