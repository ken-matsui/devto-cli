use std::path::Path;

use anyhow::{bail, Result};
use inflections::case::is_kebab_case;

use crate::config::CONFIG_PATH;

pub(crate) fn basic(title: &String) -> Result<()> {
    if !is_kebab_case(title) {
        bail!("Title must be in kebab-case");
    }
    if !Path::new(CONFIG_PATH).exists() {
        bail!("Could not find config file: {CONFIG_PATH}");
    }
    Ok(())
}
