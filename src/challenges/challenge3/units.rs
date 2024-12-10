use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use super::{GALLONS_PER_LITER, LITERS_PER_GALLON, LITRES_PER_PINT, PINTS_PER_LITRE};

#[derive(Deserialize)]
struct VolumeUnits {
    gallons: Option<f32>,
    liters: Option<f32>,
    litres: Option<f32>,
    pints: Option<f32>,
}

pub fn convert_units(body_str: String) -> (StatusCode, Response<Body>) {
    // Parse the body as JSON into a struct
    let body = serde_json::from_str::<VolumeUnits>(&body_str);
    if body.is_err() {
        return (StatusCode::BAD_REQUEST, Response::default());
    }
    let body = body.unwrap();

    // Check if more than 1 keys are provided
    let mut count = 0;
    count += body.gallons.map(|_| 1).unwrap_or(0);
    count += body.liters.map(|_| 1).unwrap_or(0);
    count += body.litres.map(|_| 1).unwrap_or(0);
    count += body.pints.map(|_| 1).unwrap_or(0);
    if count != 1 {
        return (StatusCode::BAD_REQUEST, Response::default());
    }

    // Convert the units
    if let Some(gallons) = body.gallons {
        // Gallons to liters
        let liters = gallons * LITERS_PER_GALLON;
        return (
            StatusCode::OK,
            Json::from(json!({"liters": liters })).into_response(),
        );
    }
    if let Some(liters) = body.liters {
        // Liters to gallons
        let gallons = liters * GALLONS_PER_LITER;
        return (
            StatusCode::OK,
            Json::from(json!({"gallons": gallons })).into_response(),
        );
    }
    if let Some(litres) = body.litres {
        // Litres to pints
        let pints = litres * PINTS_PER_LITRE;
        return (
            StatusCode::OK,
            Json::from(json!({"pints": pints })).into_response(),
        );
    }
    if let Some(pints) = body.pints {
        // Pints to litres
        let litres = pints * LITRES_PER_PINT;
        return (
            StatusCode::OK,
            Json::from(json!({"litres": litres })).into_response(),
        );
    }

    // This should never be reached
    (StatusCode::BAD_REQUEST, Response::default())
}
