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

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct DevToUser {
    pub(crate) username: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Article {
    id: u32,
    pub(crate) slug: String,
    pub(crate) user: DevToUser,
}

pub(crate) fn get_matched_article(article_id: u32, devto_token: String) -> Result<Article> {
    let mut response = Request::get("https://dev.to/api/articles/me/unpublished")
        .header("api-key", devto_token)
        .body(())?
        .send()?;

    if response.status() != StatusCode::OK {
        bail!("Could not retrieve article info: {}", response.text()?);
    }

    let articles = response.json::<Vec<Article>>()?;
    let matched = articles.iter().find(|&x| x.id == article_id);
    if matched.is_none() {
        bail!("Could not find the article with id: {article_id}");
    }

    Ok(matched.unwrap().clone())
}
