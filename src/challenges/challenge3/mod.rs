use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use milk::get_milk;
use std::time::Duration;

mod milk;

pub fn router() -> Router {
    let _limiter = RateLimiter::builder()
        .initial(5)
        .interval(Duration::from_millis(200))
        .build();
    Router::new().route("/9/milk", get(get_milk))
}
