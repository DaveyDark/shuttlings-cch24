use std::env;

use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Claim {
    exp: i64,
    body: Value,
}

pub async fn wrap(Json(body): Json<Value>) -> Result<(StatusCode, CookieJar), StatusCode> {
    // Get secret from .env
    let secret = env::var("JWT_SECRET");
    if secret.is_err() {
        println!("{:?}", secret.err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let secret = secret.unwrap();

    // Create claim
    let claim = Claim {
        exp: 10000000000,
        body,
    };

    // Create and sign JWT with secret
    let key = EncodingKey::from_secret(secret.as_bytes());
    let jwt = encode(&Header::default(), &claim, &key);
    if jwt.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let jwt = jwt.unwrap();

    // Set Cookie and return
    let cookie = Cookie::new("gift", jwt);
    let jar = CookieJar::new().add(cookie);
    return Ok((StatusCode::OK, jar));
}

pub async fn unwrap(jar: CookieJar) -> Result<Json<Value>, StatusCode> {
    // Get cookie from request
    let cookie = jar.get("gift");
    if cookie.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let cookie = cookie.unwrap().value();

    // Get secret from .env
    let secret = env::var("JWT_SECRET");
    if secret.is_err() {
        println!("{:?}", secret.err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let secret = secret.unwrap();
    let key = DecodingKey::from_secret(secret.as_bytes());

    let claim = decode::<Claim>(cookie, &key, &Validation::default());
    if claim.is_err() {
        println!("{:?}", claim.err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let body = claim.unwrap().claims.body;
    Ok(Json(body))
}
