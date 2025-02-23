use app_config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // init_tracing()?;
    let conf = AppConfig::from_file()?;

    // let args = CliArgs::parse();
    // let cli = Cli { args, conf };
    // cli.run().await
    Ok(())
}
