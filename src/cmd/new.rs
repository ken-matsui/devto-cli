use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use inflections::case::is_kebab_case;

use crate::config::{update_config, CONFIG_PATH};
use crate::consts::BASE_DIR;

pub(crate) fn exec(title: &String, devto_token: String) -> Result<()> {
    if !is_kebab_case(title) {
        bail!("Title must be in kebab-case");
    }
    if !Path::new(CONFIG_PATH).exists() {
        bail!("Could not find config file: {CONFIG_PATH}");
    }

    let article_file: PathBuf = [BASE_DIR, title, format!("{}.md", title).as_str()]
        .iter()
        .collect();
    if article_file.exists() {
        bail!("Article already exists");
    }

    let article_id = crate::api::create_draft(title, devto_token)?;
    println!("Created an article with id: {article_id}");

    update_config(article_id, article_file.clone())?;
    println!("Updated config file: {CONFIG_PATH}");

    crate::template::generate(title, article_file.clone())?;
    println!("Generated template file: {:?}", article_file);

    Ok(())
}
