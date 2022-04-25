use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::api;
use crate::config::{update_config, CONFIG_PATH};
use crate::template::{self, article_file_path};
use crate::validator;

pub(crate) fn exec(title: &String, devto_token: String) -> Result<()> {
    validator::basic(title)?;

    let article_file: PathBuf = article_file_path(title);
    if article_file.exists() {
        bail!("Article already exists");
    }

    let article_id = api::create_draft(title, devto_token)?;
    println!("Created an article with id: {article_id}");

    update_config(article_id, article_file.clone())?;
    println!("Updated config file: {CONFIG_PATH}");

    template::generate(title, article_file.clone())?;
    println!("Generated template file: {:?}", article_file);

    Ok(())
}
