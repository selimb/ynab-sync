mod common;

#[cfg_attr(not(feature = "live_test"), ignore)]
#[tokio::test]
async fn it_works() -> Result<(), anyhow::Error> {
    let client = common::get_client()?;
    let data = client.get_budgets().await?;
    println!("data: {data:#?}");
    Ok(())
}
