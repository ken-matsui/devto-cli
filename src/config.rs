use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub(crate) const CONFIG_PATH: &str = "./dev-to-git.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) id: u32,
    pub(crate) relative_path_to_article: String,
}

pub(crate) fn write_config(config: Vec<Config>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CONFIG_PATH)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &config)?;
    Ok(())
}

pub(crate) fn read_config() -> Result<Vec<Config>> {
    let file = File::open(CONFIG_PATH)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

pub(crate) fn update_config(article_id: u32, article_file: PathBuf) -> Result<()> {
    let mut config = read_config()?;
    config.push(Config {
        id: article_id,
        relative_path_to_article: article_file.into_os_string().into_string().unwrap(),
    });

    write_config(config)?;
    Ok(())
}
