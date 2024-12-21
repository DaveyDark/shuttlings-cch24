use sqlx::{Error, PgPool};

const CREATE_QUOTES_QUERY: &str = "
        CREATE TABLE IF NOT EXISTS quotes (
            id UUID PRIMARY KEY,
            author TEXT NOT NULL,
            quote TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            version INT NOT NULL DEFAULT 1
        );
        ";
const CREATE_TOKENS_QUERY: &str = "
        CREATE TABLE IF NOT EXISTS tokens (
            token VARCHAR(16) PRIMARY KEY,
            page INT NOT NULL
        );
        ";

pub async fn init_db(pool: &PgPool) -> Result<(), Error> {
    // Create table
    let mut transaction = pool.begin().await?;
    sqlx::query(CREATE_QUOTES_QUERY)
        .execute(&mut *transaction)
        .await?;
    sqlx::query(CREATE_TOKENS_QUERY)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await
}

pub async fn reset_db(pool: &PgPool) -> Result<(), Error> {
    // Delete all entries in table
    sqlx::query("DELETE FROM quotes;")
        .execute(pool)
        .await
        .map(|_| ())
}
