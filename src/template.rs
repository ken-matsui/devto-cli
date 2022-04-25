use convert_case::{Case, Casing};
use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::consts::BASE_DIR;

pub(crate) fn article_file_path(title: &String) -> PathBuf {
    [".", BASE_DIR, title, format!("{}.md", title).as_str()]
        .iter()
        .collect()
}

pub(crate) fn generate(title: &String, article_file: PathBuf) -> Result<()> {
    fs::create_dir_all(article_file.parent().unwrap())?;

    fs::write(
        article_file,
        format!(
            r#"---
title: '{}'
published: false
description: ''
tags:
---
"#,
            title.to_case(Case::Title)
        ),
    )?;

    Ok(())
}
