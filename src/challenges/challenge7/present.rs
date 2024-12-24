use axum::{extract::Path, http::StatusCode};
use handlebars::Handlebars;
use serde_json::json;
enum Color {
    Red,
    Blue,
    Purple,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Red => "red".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Purple => "purple".to_string(),
        }
    }
}

impl Color {
    fn next(&self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Purple,
            Color::Purple => Color::Red,
        }
    }
}

fn parse_color(color: &str) -> Option<Color> {
    match color {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        _ => None,
    }
}

pub async fn present(Path(color): Path<String>) -> Result<String, StatusCode> {
    let color = parse_color(&color).ok_or(StatusCode::IM_A_TEAPOT)?;
    let next_color = color.next();
    let mut hb = Handlebars::new();
    let response_str = r#"
    <div class="present {{color}}" hx-get="/23/present/{{next}}" hx-swap="outerHTML">
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
    </div>
    "#;
    hb.register_template_string("response", response_str)
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    let response = hb
        .render(
            "response",
            &json! ({
                "color": color.to_string(),
                "next": next_color.to_string()
            }),
        )
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;

    Ok(response)
}
