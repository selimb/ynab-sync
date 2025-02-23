use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context as _};
use serde::Deserialize;
use tracing::info;

const CONFIG_FILENAME: &str = "config.json5";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database_file: String,
}

impl AppConfig {
    pub fn from_file() -> Result<AppConfig, anyhow::Error> {
        let config_path = find_file()?;

        let config_text = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file from {config_path:?}"))?;

        info!("Read config from {config_path:?}");
        let config = json5::from_str::<AppConfig>(&config_text).context("Invalid config")?;

        Ok(config)
    }

    pub fn database_url(&self) -> String {
        format!("sqlite://{}?mode=rwc", self.database_file)
    }
}

fn find_file() -> Result<PathBuf, anyhow::Error> {
    let cwd = std::env::current_dir().context("CWD is invalid")?;
    let mut dir = cwd.as_path();
    let mut candidates = Vec::new();
    for _ in 0..2 {
        let config_path = dir.join(CONFIG_FILENAME);
        if config_path.is_file() {
            return Ok(config_path);
        }
        candidates.push(config_path.clone());
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    Err(anyhow!(
        "Failed to find config file. Tried:\n{candidates:?}"
    ))
}
