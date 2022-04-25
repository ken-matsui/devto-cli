use anyhow::{bail, Result};
use webbrowser;

use crate::api::get_matched_article;
use crate::template::article_file_path;

pub(crate) fn exec(title: &String, devto_token: String) -> Result<()> {
    crate::validator::basic(title)?;
    let article_file_path = article_file_path(title)
        .into_os_string()
        .into_string()
        .unwrap();

    let config = crate::config::read_config()?;
    let matched = config
        .iter()
        .find(|&x| x.relative_path_to_article == article_file_path);
    if matched.is_none() {
        bail!("Could not find the article with title: {title}");
    }
    let article = get_matched_article(matched.unwrap().id, devto_token)?;

    println!("Opening the default OS web browser due to no delete DEV API ...");
    webbrowser::open(
        format!(
            "https://dev.to/{}/{}/delete_confirm",
            article.user.username, article.slug
        )
        .as_str(),
    )?;

    Ok(())
}
