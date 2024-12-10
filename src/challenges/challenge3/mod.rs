use axum::{routing::post, Router};
use leaky_bucket::RateLimiter;
use milk::{get_milk, refill_milk};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

mod milk;
mod units;

// Conversion factors
const LITERS_PER_GALLON: f32 = 3.78541; // Liters per US gallon
const GALLONS_PER_LITER: f32 = 0.264172060; // US gallons per liter
const PINTS_PER_LITRE: f32 = 1.759754; // UK pints per liter
const LITRES_PER_PINT: f32 = 0.56826125; // Liters per UK pint

pub fn new_limiter() -> RateLimiter {
    RateLimiter::builder()
        .initial(5)
        .interval(Duration::from_millis(1000))
        .refill(1)
        .max(5)
        .build()
}

pub fn router() -> Router {
    // Create the limiter, wrapped in an Arc and Mutex
    let limiter = Arc::new(Mutex::new(new_limiter()));

    // Create the router
    Router::new()
        .route("/milk", post(get_milk))
        .route("/refill", post(refill_milk))
        .with_state(limiter)
}
