use axum::http::StatusCode;

pub async fn get_milk() -> StatusCode {
    StatusCode::OK
}
