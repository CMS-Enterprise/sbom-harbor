use async_std;
use std::env;

mod common;

use crate::common::{get_test_context, teardown};

use harbor::lib::Client;

#[async_std::test]
async fn can_login() -> anyhow::Result<()> {
    let cloud_front_domain = env::var("CF_DOMAIN")?;
    let username = env::var("ADMIN_USERNAME")?;
    let password = env::var("ADMIN_PASSWORD")?;

    let result = Client::new(cloud_front_domain, username, password).await;

    assert!(!result.is_err());

    Ok(())
}

#[async_std::test]
async fn can_build_and_teardown_test_context() -> std::io::Result<()> {
    let ctx = get_test_context().await;

    assert!(!ctx.is_err(), "{:?}", ctx);

    let ctx = ctx.unwrap();

    teardown(ctx).await?;
    Ok(())
}
