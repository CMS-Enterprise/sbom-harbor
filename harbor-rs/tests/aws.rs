use harbor::aws::secretsmanager;

#[async_std::test]
async fn can_get_secret() -> Result<(), String> {
    let okta = secretsmanager::get_secret("Okta")
        .await
        .map_err(|e| format!("{:?}", e))?;

    println!("{}", okta.unwrap());

    Ok(())
}