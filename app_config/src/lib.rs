use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context as _};
use serde::{Deserialize, Deserializer};
use tracing::info;

const CONFIG_FILENAME: &str = "config.json5";

fn deser_url<'de, D>(deserializer: D) -> Result<reqwest::Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    reqwest::Url::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Clone, Deserialize)]
pub struct YnabConfig {
    #[serde(deserialize_with = "deser_url")]
    pub base_url: reqwest::Url,
    pub token: String,
    pub budget_id: String,
}

impl std::fmt::Debug for YnabConfig {
    /// Same as default behavior, but censors `password`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YnabConfig")
            .field("base_url", &self.base_url)
            .field("token", &"***")
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database_file: String,
    pub ynab: YnabConfig,
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
