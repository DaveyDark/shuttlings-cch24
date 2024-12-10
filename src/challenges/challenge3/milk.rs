use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use leaky_bucket::RateLimiter;
use tokio::sync::Mutex;

use super::{new_limiter, units::convert_units};

pub async fn get_milk(
    headers: HeaderMap,
    State(limiter): State<Arc<Mutex<RateLimiter>>>,
    body: String,
) -> (StatusCode, Response) {
    let limiter = limiter.lock().await;
    // Try to acquire a token from the rate limiter
    if limiter.try_acquire(1) {
        if let Some(content_type) = headers.get("Content-Type") {
            if content_type == "application/json" {
                // Convert the units
                return convert_units(body);
            }
        }
        // Success
        (
            StatusCode::OK,
            "Milk withdrawn\n".to_string().into_response(),
        )
    } else {
        // Rate limit exceeded
        (
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_string().into_response(),
        )
    }
}

pub async fn refill_milk(State(limiter): State<Arc<Mutex<RateLimiter>>>) -> StatusCode {
    // Reset the rate limiter
    // This is a bit of a hack, but it's the only way to change the rate limiter
    let mut limiter = limiter.lock().await;
    *limiter = new_limiter();
    StatusCode::OK
}
