use app_config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // init_tracing()?;
    let _conf = AppConfig::from_file()?;

    Ok(())
}
