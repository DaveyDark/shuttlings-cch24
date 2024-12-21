use axum::{
    routing::{delete, get, post, put},
    Router,
};
use endpoints::{cite, draft, remove, reset, undo};
use quotes::init_db;
use sqlx::PgPool;

mod endpoints;
mod quotes;

pub async fn router(pool: &PgPool) -> Router {
    // Init DB pool
    let pool = pool.clone();

    init_db(&pool).await;

    Router::new()
        .route("/reset", post(reset))
        .route("/cite/:id", get(cite))
        .route("/remove/:id", delete(remove))
        .route("/undo/:id", put(undo))
        .route("/draft", post(draft))
        .with_state(pool)
}
