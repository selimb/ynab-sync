use std::fs;

use anyhow::Context;
use serde::Deserialize;
use tracing::info;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database_file: String,
}

impl AppConfig {
    pub fn from_file() -> Result<AppConfig, anyhow::Error> {
        let config_path = std::env::current_dir()
            .context("CWD is invalid")?
            .as_path()
            .join("config.json5");

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
