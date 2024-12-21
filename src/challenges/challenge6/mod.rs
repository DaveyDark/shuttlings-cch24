use axum::{
    routing::{delete, get, post, put},
    Router,
};
use db::init_db;
use endpoints::{cite, draft, list, remove, reset, undo};
use sqlx::PgPool;

mod db;
mod endpoints;
mod quotes;
mod token;

pub async fn router(pool: &PgPool) -> Router {
    // Init DB pool
    let pool = pool.clone();

    if let Err(e) = init_db(&pool).await {
        eprintln!("{:?}", e);
        println!("Challenge 19 router init failed!");
        return Router::new();
    }

    Router::new()
        .route("/reset", post(reset))
        .route("/cite/:id", get(cite))
        .route("/remove/:id", delete(remove))
        .route("/undo/:id", put(undo))
        .route("/draft", post(draft))
        .route("/list", get(list))
        .with_state(pool)
}
