use anyhow::{bail, Result};
use dialoguer::Confirm;
use std::fs;
use std::path::PathBuf;

use crate::api::{get_matched_article, Article};
use crate::config::{write_config, Config};
use crate::template::article_file_path;

fn delete_online(article: Article) -> Result<()> {
    if Confirm::new()
        .with_prompt("Do you want to delete this article from dev.to? (will open a web browser)")
        .interact()?
    {
        println!("Opening the default OS web browser ...");
        webbrowser::open(
            format!(
                "https://dev.to/{}/{}/delete_confirm",
                article.user.username, article.slug
            )
            .as_str(),
        )?;
    }

    Ok(())
}

fn delete_locally(article_file_path: PathBuf, mut config: Vec<Config>, found: usize) -> Result<()> {
    if !Confirm::new()
        .with_prompt("Are you sure you want to locally delete this article?")
        .interact()?
    {
        bail!("Aborted by user");
    }
    fs::remove_dir_all(article_file_path.parent().unwrap())?;
    config.remove(found);
    write_config(config)?;
    println!("Deleted this article.\n");

    Ok(())
}

pub(crate) fn exec(title: &String, devto_token: String) -> Result<()> {
    crate::validator::basic(title)?;
    let article_file_path = article_file_path(title);
    if !article_file_path.exists() {
        bail!("Could not find the article with title: {title}");
    }

    let article_file = article_file_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    let config = crate::config::read_config()?;
    match config
        .iter()
        .position(|x| x.relative_path_to_article == article_file)
    {
        None => bail!("Could not find the article with title: {title}"),
        Some(found) => {
            println!(
                "WARNING: Associated assets will be also deleted, and this action cannot be undone."
            );
            println!("WARNING: DEV API does not support deleting an article via API, so you still need to delete it from dev.to.\n");

            let article = get_matched_article(config[found].id, devto_token)?;
            delete_locally(article_file_path, config, found)?;
            delete_online(article)?;
        }
    };

    Ok(())
}
