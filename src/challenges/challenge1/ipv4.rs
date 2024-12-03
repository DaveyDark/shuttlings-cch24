use crate::challenge1::{DestQuery, KeyQuery};
use axum::{extract::Query, http::StatusCode};

fn split_ip(ip: &String) -> Result<Vec<u8>, String> {
    ip.split('.')
        .map(|i| {
            i.parse::<u8>()
                .map_err(|_| "Failed to parse IP part :".to_string() + i)
        })
        .collect::<Result<Vec<u8>, String>>()
}

pub async fn dest(Query(query): Query<DestQuery>) -> (StatusCode, String) {
    // Parse the query parameters
    let from = query.from;
    let key = query.key;

    // Split IP address into octets
    let ip = split_ip(&from);
    let key = split_ip(&key);
    if ip.is_err() || key.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            ip.err().unwrap() + "\n Invalid IP address",
        );
    }
    let ip = ip.unwrap();
    let key = key.unwrap();

    // Perform overflowing addition
    let result: Vec<String> = ip
        .iter()
        .zip(key.iter())
        .map(|(i, k)| i.overflowing_add(*k).0.to_string())
        .collect();

    // Convert back to string
    (StatusCode::OK, result.join("."))
}

pub async fn key(Query(params): Query<KeyQuery>) -> (StatusCode, String) {
    // Parse the query parameters
    let from = params.from;
    let to = params.to;

    // Split IP address into octets
    let from = split_ip(&from);
    let to = split_ip(&to);
    if from.is_err() || to.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            from.err().unwrap() + "\n Invalid IP address",
        );
    }
    let from = from.unwrap();
    let to = to.unwrap();

    // Find difference between the two IPs
    let result: Vec<String> = from
        .iter()
        .zip(to.iter())
        .map(|(f, t)| t.overflowing_sub(*f).0.to_string())
        .collect();

    (StatusCode::OK, result.join("."))
}
