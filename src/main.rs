use std::env;

use axum::routing::get;
use axum::Router;
use challenges::challenge0::{self, hello_world};
use challenges::{challenge1, challenge2, challenge3, challenge4, challenge5, challenge6};
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_shared_db::Postgres;
use sqlx::PgPool;

mod challenges;

#[shuttle_runtime::main]
async fn main(
    #[Postgres] pool: PgPool,
    #[Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Set secret to env var
    env::set_var(
        "JWT_SECRET",
        secrets.get("JWT_SECRET").expect("JWT_SECRET not set"),
    );

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/-1", challenge0::router())
        .nest("/2", challenge1::router())
        .nest("/5", challenge2::router())
        .nest("/9", challenge3::router())
        .nest("/12", challenge4::router())
        .nest("/16", challenge5::router())
        .nest("/19", challenge6::router(&pool).await);

    Ok(router.into())
}
