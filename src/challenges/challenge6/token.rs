use rand::{distributions::Alphanumeric, Rng};
use sqlx::{prelude::FromRow, Error, PgPool};

#[derive(FromRow)]
pub struct Token {
    pub token: String,
    pub page: i32,
}

pub async fn generate_token(pool: &PgPool) -> Result<Token, Error> {
    // Create token
    let token_id: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(16)
        .map(|x| x as char)
        .collect();

    // Save to DB
    sqlx::query("INSERT INTO tokens (token, page) VALUES ($1, $2)")
        .bind(&token_id)
        .bind(1)
        .execute(pool)
        .await?;

    Ok(Token {
        token: token_id,
        page: 0,
    })
}

pub async fn advance_token(pool: &PgPool, token: &str) -> Result<i32, Error> {
    // Get page
    let page: i32 = sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE token = $1")
        .bind(token)
        .fetch_one(pool)
        .await?
        .page;

    // Increment page
    sqlx::query("UPDATE tokens SET page = $1 WHERE token = $2")
        .bind(page + 1)
        .bind(token)
        .execute(pool)
        .await?;

    Ok(page)
}

pub async fn discard_token(pool: &PgPool, token: &str) -> Result<(), Error> {
    // Delete token
    sqlx::query("DELETE FROM tokens WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn validate_token(pool: &PgPool, token: &str) -> Result<(), Error> {
    // Check if token exists
    sqlx::query("SELECT * FROM tokens WHERE token = $1")
        .bind(token)
        .fetch_one(pool)
        .await
        .map(|_| ())
}
