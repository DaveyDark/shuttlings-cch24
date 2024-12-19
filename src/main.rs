use axum::routing::get;
use axum::Router;
use challenges::challenge0;
use challenges::challenge0::hello_world;
use challenges::challenge1;
use challenges::challenge2;
use challenges::challenge3;
use challenges::challenge4;

mod challenges;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/-1", challenge0::router())
        .nest("/2", challenge1::router())
        .nest("/5", challenge2::router())
        .nest("/9", challenge3::router())
        .nest("/12", challenge4::router());

    Ok(router.into())
}
