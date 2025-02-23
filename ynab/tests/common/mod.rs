use app_config::AppConfig;

pub fn get_client() -> Result<ynab::Client, anyhow::Error> {
    let conf = AppConfig::from_file()?;
    let client = ynab::Client::new(conf.ynab);
    Ok(client)
}
