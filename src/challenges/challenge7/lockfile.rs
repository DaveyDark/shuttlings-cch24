use axum::http::StatusCode;
use axum_extra::extract::Multipart;
use handlebars::Handlebars;
use serde_json::json;
use toml::Table;

pub async fn lockfile(mut multipart: Multipart) -> Result<String, StatusCode> {
    // Response vector
    let mut res: Vec<String> = Vec::new();

    // Loop through all fields in the multipart
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        // Check if the field is named "lockfile"
        let name = field.name().ok_or(StatusCode::BAD_REQUEST)?.to_string();
        if name.as_str() != "lockfile" {
            continue;
        }
        // Get the data from the field
        let data = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Parse the data as a TOML table
        let toml = toml::from_str::<Table>(&data).map_err(|_| StatusCode::BAD_REQUEST)?;

        // Get the "package" array from the TOML table
        let packages = toml
            .get("package")
            .and_then(|p| p.as_array())
            .ok_or(StatusCode::BAD_REQUEST)?;

        for val in packages {
            // Get the "checksum" field from the package as a string
            let checksum_val = val.get("checksum");
            if checksum_val.is_none() {
                continue;
            }
            let checksum_val = checksum_val.unwrap();
            let checksum = checksum_val.as_str().ok_or(StatusCode::BAD_REQUEST)?;

            // Parse the checksum and add it to the response vector if successful
            let parsed_checksum = parse_checksum(checksum);
            if parsed_checksum.is_err() {
                return Err(StatusCode::UNPROCESSABLE_ENTITY);
            }
            let parsed_checksum = parsed_checksum.unwrap();
            res.push(parsed_checksum);
        }
    }

    // If no checksums were found, return a bad request
    if res.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Return the response vector as a string
    Ok(res.join("\n"))
}

fn parse_checksum(checksum: &str) -> Result<String, ()> {
    // Remove quotes from the checksum
    let checksum = checksum.trim_matches('"');

    // Create an iterator over the checksum characters
    let mut chars_iter = checksum.chars();

    // Try to use the first 6 characters as a color
    let mut color = String::new();
    for _ in 0..6 {
        let next = chars_iter.next().ok_or(())?;
        if !next.is_ascii_hexdigit() {
            return Err(());
        }
        color.push(next);
    }

    // Try to use the next 2 characters as a top value
    let top_str = format!(
        "{}{}",
        chars_iter.next().ok_or(())?,
        chars_iter.next().ok_or(())?
    );
    let top = u32::from_str_radix(&top_str, 16).map_err(|_| ())?;

    // Try to use the next 2 characters as a left value
    let left_str = format!(
        "{}{}",
        chars_iter.next().ok_or(())?,
        chars_iter.next().ok_or(())?
    );
    let left = u32::from_str_radix(&left_str, 16).map_err(|_| ())?;

    // Create a new Handlebars instance and render the response with the color, top, and left values
    let mut hb = Handlebars::new();
    let response_str =
        r#"<div style="background-color:#{{color}};top:{{top}}px;left:{{left}}px;"></div>"#;
    hb.register_template_string("response", response_str)
        .map_err(|_| ())?;
    let response = hb
        .render(
            "response",
            &json!({
                "color": color,
                "top": top,
                "left": left,
            }),
        )
        .map_err(|_| ())?;
    Ok(response)
}
