use axum::Router;
use challenges::challenge0;
use challenges::challenge1;

mod challenges;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let mut router = Router::new();
    router = challenge0::route(router);
    router = challenge1::route(router);

    Ok(router.into())
}
