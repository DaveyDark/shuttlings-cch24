use std::net::Ipv6Addr;

use axum::{extract::Query, http::StatusCode};

use super::{DestQuery, KeyQuery};

pub async fn dest_v6(Query(query): Query<DestQuery>) -> (StatusCode, String) {
    // Parse the query parameters
    let from = query.from.parse::<Ipv6Addr>();
    let key = query.key.parse::<Ipv6Addr>();
    if from.is_err() || key.is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid IP address".to_string());
    }
    let from = from.unwrap().to_bits();
    let key = key.unwrap().to_bits();

    let result = Ipv6Addr::from_bits(from ^ key);

    (StatusCode::OK, result.to_string())
}

pub async fn key_v6(Query(query): Query<KeyQuery>) -> (StatusCode, String) {
    // Parse the query parameters
    let from = query.from.parse::<Ipv6Addr>();
    let to = query.to.parse::<Ipv6Addr>();
    if from.is_err() || to.is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid IP address".to_string());
    }
    let from = from.unwrap().to_bits();
    let to = to.unwrap().to_bits();

    let result = Ipv6Addr::from_bits(from ^ to);

    (StatusCode::OK, result.to_string())
}
