use platform::auth::{Action, Effect};
use platform::Error;

mod common;
use crate::common::mongodb::{local_context, AuthScenario};

#[async_std::test]
async fn can_create_local_cluster() -> Result<(), Error> {
    let cx = local_context().await?;

    let uri = cx.connection_uri();
    assert!(!uri.is_empty());

    Ok(())
}

// TODO: Basic crud operations are covered by the policy tests.
// Additional CRUD tests need to be defined as more nuances edge cases emerge.
#[async_std::test]
async fn can_crud() -> Result<(), Error> {
    println!("can_crud not implemented");
    Ok(())
}

// Tests that if a user has no policy assigned for the default scenario resource they are denied.
#[async_std::test]
async fn can_assert_implicit_deny_no_policy() -> Result<(), Error> {
    let test_name = "can_assert_implicit_deny_no_policy".to_string();
    let ctx = local_context().await?;

    let scenario = AuthScenario::new(test_name.clone(), &ctx).await?;

    scenario.remove_group_policy_for_user().await?;

    let effect = scenario.assert(Action::Create).await?;

    // Always teardown even if the assertion fails so that we don't get residual data.
    scenario.teardown().await?;

    assert_eq!(effect, Effect::Deny);

    Ok(())
}

// Tests that if a user has an explicit deny, as well as an allow, the explicit deny takes precedence.
#[async_std::test]
async fn can_assert_explicit_deny() -> Result<(), Error> {
    let test_name = "can_assert_explicit_deny".to_string();
    let ctx = local_context().await?;

    let mut scenario = AuthScenario::new(test_name.clone(), &ctx).await?;

    scenario.with_policy(Action::Create, Effect::Deny).await?;

    let effect = scenario.assert(Action::Create).await?;

    // Always teardown even if the assertion fails so that we don't get residual data.
    scenario.teardown().await?;

    assert_eq!(effect, Effect::Deny);

    Ok(())
}

// Tests that if a user has a policy for the resource, but not for the action requested, they are denied.
#[async_std::test]
async fn can_assert_implicit_deny_no_action() -> Result<(), Error> {
    let test_name = "can_assert_implicit_deny".to_string();
    let ctx = local_context().await?;

    let mut scenario = AuthScenario::new(test_name.clone(), &ctx).await?;

    scenario.with_policy(Action::Read, Effect::Allow).await?;

    let effect = scenario.assert(Action::Update).await?;

    // Always teardown even if the assertion fails so that we don't get residual data.
    scenario.teardown().await?;

    assert_eq!(effect, Effect::Deny);

    Ok(())
}

// Tests that if a user has a policy for the resource for the action requested, they are allowed.
#[async_std::test]
async fn can_assert_explicit_allow_for_action() -> Result<(), Error> {
    let test_name = "can_assert_explicit_allow".to_string();
    let ctx = local_context().await?;
    let mut scenario = AuthScenario::new(test_name.clone(), &ctx).await?;

    scenario.with_policy(Action::Update, Effect::Allow).await?;

    let effect = scenario.assert(Action::Update).await?;

    // Always teardown even if the assertion fails so that we don't get residual data.
    scenario.teardown().await?;

    assert_eq!(effect, Effect::Allow);

    Ok(())
}

// Tests that if a user has a read-only policy for the resource they cannot edit.
#[async_std::test]
async fn can_assert_deny_for_readonly_policy() -> Result<(), Error> {
    println!("can_assert_deny_for_readonly_policy not implemented");
    Ok(())
}

// Tests that if a user has been disabled they cannot access resources.
#[async_std::test]
async fn can_assert_deny_for_disabled_user() -> Result<(), Error> {
    println!("can_assert_deny_for_disabled_user not implemented");
    Ok(())
}

// Tests that if a user has a policy for the resource, but not for the action requested, they are denied.
#[async_std::test]
async fn can_assert_allow_any_for_admin() -> Result<(), Error> {
    println!("can_assert_allow_any_for_admin not implemented");
    Ok(())
}

// Tests that if a user has a policy for the resource, but not for the action requested, they are denied.
#[async_std::test]
async fn can_assert_deny_any_for_disabled_admin() -> Result<(), Error> {
    println!("can_assert_deny_any_for_disabled_admin not implemented");
    Ok(())
}
