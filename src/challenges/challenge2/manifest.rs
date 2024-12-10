use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
};
use cargo_manifest::{Manifest, Value};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Order {
    item: String,
    quantity: u32,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    orders: Option<Vec<Value>>,
}

pub async fn parse_manifest(headers: HeaderMap, body: Bytes) -> (StatusCode, String) {
    // Parse manifest
    let mut manifest: Option<Manifest<Metadata>> = None;
    if let Some(content_type) = headers.get("Content-Type") {
        if content_type == "application/toml" {
            let _manifest = Manifest::from_slice_with_metadata(body.trim_ascii());
            if _manifest.is_err() {
                return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
            }
            manifest = Some(_manifest.unwrap());
        } else if content_type == "application/yaml" {
            let body_str = String::from_utf8(body.trim_ascii().to_vec());
            let _manifest = serde_yaml::from_str::<Manifest<Metadata>>(&body_str.unwrap());
            if _manifest.is_err() {
                return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
            }
            manifest = Some(_manifest.unwrap());
        } else if content_type == "application/json" {
            let body_str = String::from_utf8(body.trim_ascii().to_vec());
            let _manifest = serde_json::from_str::<Manifest<Metadata>>(&body_str.unwrap());
            if _manifest.is_err() {
                return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
            }
            manifest = Some(_manifest.unwrap());
        }
    }

    if manifest.is_none() {
        return (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Invalid manifest".to_string(),
        );
    }
    let manifest = manifest.unwrap();

    // Check keywords
    if manifest
        .package
        .as_ref()
        .and_then(|pkg| pkg.keywords.as_ref())
        .and_then(|kw| kw.as_ref().as_local())
        .and_then(|keys| keys.contains(&"Christmas 2024".to_string()).then_some(()))
        .is_none()
    {
        return (
            StatusCode::BAD_REQUEST,
            "Magic keyword not provided".to_string(),
        );
    }

    // Make list of orders
    let orders = manifest
        .package
        .and_then(|pkg| pkg.metadata)
        .and_then(|meta| meta.orders);
    if let None = orders {
        return (StatusCode::NO_CONTENT, String::new());
    }
    let orders = orders.unwrap();
    let mut order_list: Vec<Order> = vec![];
    for order in orders {
        // Check if order is valid
        if let Ok(order) = order.try_into::<Order>() {
            order_list.push(order);
        }
    }

    // If not valid orders are found, return 204
    if order_list.is_empty() {
        return (StatusCode::NO_CONTENT, String::new());
    }

    // Convert to plain string
    let mut list = String::new();
    for item in order_list {
        list += format!("{}: {}\n", item.item, item.quantity).as_str()
    }
    list = list.trim_end_matches('\n').to_string();
    (StatusCode::OK, list)
}
