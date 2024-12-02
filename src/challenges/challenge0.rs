use axum::{
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};

pub fn route(router: Router) -> Router {
    router
        .route("/", get(hello_world))
        .route("/-1/seek", get(vibe_of_the_day))
}

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn vibe_of_the_day() -> (StatusCode, HeaderMap) {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Location",
        "https://www.youtube.com/watch?v=9Gc4QTqslN4"
            .parse()
            .unwrap(),
    );
    (StatusCode::FOUND, headers)
}
