use axum::{extract::Path, http::StatusCode};
use handlebars::Handlebars;
use serde_json::json;
struct State {
    on: bool,
}

impl ToString for State {
    fn to_string(&self) -> String {
        if self.on {
            "on".to_string()
        } else {
            "off".to_string()
        }
    }
}

impl State {
    fn invert(&self) -> State {
        State { on: !self.on }
    }
    fn format(&self) -> String {
        if self.on {
            " on".to_string()
        } else {
            "".to_string()
        }
    }
}

fn parse_state(state: String) -> Option<State> {
    match state.as_str() {
        "on" => Some(State { on: true }),
        "off" => Some(State { on: false }),
        _ => None,
    }
}

pub async fn ornament(Path((state, id)): Path<(String, String)>) -> Result<String, StatusCode> {
    let state = parse_state(state).ok_or(StatusCode::IM_A_TEAPOT)?;
    let mut hb = Handlebars::new();
    let response_str = r#"<div class="ornament{{state}}" id="ornament{{id}}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{{next}}/{{id}}" hx-swap="outerHTML"></div>"#;
    hb.register_template_string("response", response_str)
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    let response = hb
        .render(
            "response",
            &json!({
                "state": state.format(),
                "id": id,
                "next": state.invert().to_string(),
            }),
        )
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    Ok(response)
}
