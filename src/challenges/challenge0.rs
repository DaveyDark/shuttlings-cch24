use axum::{
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/seek", get(vibe_of_the_day))
}

pub async fn hello_world() -> &'static str {
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
