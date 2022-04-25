use http::StatusCode;

use anyhow::{bail, Result};
use convert_case::{Case, Casing};
use isahc::{prelude::*, Request};
use serde::Deserialize;
use serde_json::json;

pub(crate) fn create_draft(title: &String, devto_token: String) -> Result<u32> {
    let data = json!({
        "article": {
            "title": title.to_case(Case::Title),
            "body_markdown": "",
            "published": false,
            "tags": []
        }
    })
    .to_string();

    let mut response = Request::post("https://dev.to/api/articles")
        .header("Content-Type", "application/json")
        .header("api-key", devto_token)
        .body(data.as_str())?
        .send()?;

    if response.status() != StatusCode::CREATED {
        bail!("Could not create an article: {}", response.text()?);
    }

    #[derive(Deserialize)]
    struct Response {
        id: u32,
    }
    Ok(response.json::<Response>()?.id)
}
