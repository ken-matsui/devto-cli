use anyhow::{bail, Result};
use std::fs;

use crate::api::update_article;
use crate::config::get_article_index;
use crate::template::article_file_path;

pub(crate) fn exec(title: &String, devto_token: String) -> Result<()> {
    crate::validator::basic(title)?;
    let article_file_path = article_file_path(title);
    if !article_file_path.exists() {
        bail!("Could not find the article with title: {title}");
    }

    // The article should be draft
    let article_content = fs::read_to_string(article_file_path.clone())?;
    let fm = frontmatter::parse(&article_content)?.unwrap();
    if let Some(published) = fm["published"].as_bool() {
        if published {
            bail!("You cannot preview published articles");
        }
    } else {
        bail!("You should have `published` as boolean in front matter");
    }

    // Update the article on dev.to with PUT method
    let article_file = article_file_path.into_os_string().into_string().unwrap();
    let config = crate::config::read_config()?;
    match get_article_index(&config, article_file) {
        None => bail!("Could not find the article with title: {title}"),
        Some(found) => {
            let article = update_article(config[found].id, article_content, devto_token)?;
            println!("Updated the article.");

            println!("Opening a browser ...");
            webbrowser::open(
                format!(
                    "https://dev.to/{}/{}/edit",
                    article.user.username, article.slug
                )
                .as_str(),
            )?;
        }
    };

    Ok(())
}
