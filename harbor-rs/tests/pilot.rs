use crate::common::{generate_pilot_apigw_request, get_test_context, teardown};
use harbor::pilot::{function_handler, handle_request, PilotRequest};

mod common;

#[async_std::test]
async fn can_run_lambda_handler() -> std::io::Result<()> {
    let result = generate_pilot_apigw_request().await;

    assert!(!result.is_err());
    let tuple = result.unwrap();

    let ctx = tuple.0;
    let apigw_req = tuple.1;
    let input = serde_json::to_string(&apigw_req)?;

    let request = lambda_http::request::from_str(&input)?;
    let response = function_handler(request).await;

    match response {
        Ok(_) => {}
        Err(error) => {
            let msg = format!("Oh noes: {}", error);
            println!("{}", msg);
        }
    }

    teardown(ctx).await
}

#[async_std::test]
async fn can_handle_request_e2e() -> std::io::Result<()> {
    let ctx = get_test_context().await;

    assert!(!ctx.is_err());

    let ctx = ctx.unwrap();

    let resp = handle_request(ctx.request.clone()).await;

    // Always teardown regardless of response to ensure future tests run.
    teardown(ctx.clone()).await?;

    assert!(!resp.is_err(), "req = {:?}, err = {:?}", ctx.request, resp);

    let resp = resp.unwrap();

    assert!(resp.valid, "{:?}", resp);

    Ok(())
}

#[test]
fn can_validate_request() {
    let event = PilotRequest {
        team_id: String::from("foo"),
        project_id: String::from("foo"),
        codebase_id: String::from("foo"),
        cloud_front_domain: String::from("foo"),
        api_gateway_url: None,
        token: String::from("foo"),
        github_url: String::from("foo"),
    };

    match event.validate() {
        Ok(()) => {}
        Err(error) => {
            panic!("Error validating event: {:?}", error)
        }
    }

    let event = PilotRequest {
        team_id: String::from(""),
        project_id: String::from(""),
        codebase_id: String::from(""),
        cloud_front_domain: String::from(""),
        api_gateway_url: None,
        token: String::from(""),
        github_url: String::from(""),
    };

    match event.validate() {
        Ok(()) => {
            panic!("Error: event should be invalid")
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}
