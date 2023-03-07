use std::collections::HashMap;
use aws_lambda_events::apigw::{ApiGatewayV2httpRequestContext, ApiGatewayV2httpRequestContextHttpDescription};
use lambda_http::Request;
use lambda_http::ext::RequestExt;
use lambda_http::http::Method;
use serde::Serialize;
use aws_lambda_events::query_map::QueryMap;
use lambda_http::request::RequestContext;
use harbor::api;

pub const TEST_TEAM_ID: &str = "11111111-1111-1111-1111-111111111111";

pub struct MockFactory {}
impl MockFactory {
    #[allow(dead_code)]
    pub fn team_id_path_params(id: Option<String>) -> HashMap<String, String> {
        match id {
            None => HashMap::from([("teamId".to_string(), TEST_TEAM_ID.to_string())]),
            Some(_) => HashMap::from([("teamId".to_string(), id.unwrap())]),
        }
    }

    #[allow(dead_code)]
    pub fn with_children_qs_params() -> HashMap<String, String> {
        HashMap::from([("children".to_string(), "true".to_string())])
    }

    #[allow(dead_code)]
    pub fn request<D>(method: Method,
        payload: Option<D>,
        path_params: HashMap<String, String>,
        qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {

        let method = method;
        let mut body = "".to_string();
        if payload.is_some() {
            let payload = payload.unwrap();
            let json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
            body = json;
        }

        let request = Request::new(aws_lambda_events::encodings::Body::from(body))
        .with_path_parameters(QueryMap::from(path_params))
        .with_query_string_parameters(qs_params)
        .with_request_context(RequestContext::ApiGatewayV2(ApiGatewayV2httpRequestContext{
            route_key: None,
            account_id: None,
            stage: None,
            request_id: None,
            authorizer: None,
            apiid: None,
            domain_name: None,
            domain_prefix: None,
            time: None,
            time_epoch: 0,
            http: ApiGatewayV2httpRequestContextHttpDescription{
                method,
                path: None,
                protocol: None,
                source_ip: None,
                user_agent: None,
            },
            authentication: None,
        }));

        Ok(request)
    }

    #[allow(dead_code)]
    pub fn get<D>(path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::GET, None, path_params, qs_params)
    }

    #[allow(dead_code)]
    pub fn put<D>(payload: D, path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::PUT, Some(payload), path_params, qs_params)
    }

    #[allow(dead_code)]
    pub fn post<D>(payload: D, path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::POST, Some(payload), path_params, qs_params)
    }

    #[allow(dead_code)]
    pub fn delete<D>(path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::DELETE, None, path_params, qs_params)
    }

    #[allow(dead_code)]
    pub fn get_team_request(id: Option<String>) -> Result<Request, String> {
        Self::get::<api::Team>(Self::team_id_path_params(id), Self::with_children_qs_params())
    }

    #[allow(dead_code)]
    pub fn get_teams_request() -> Result<Request, String> {
        Self::get::<api::Team>(HashMap::default(), HashMap::default())
    }

    #[allow(dead_code)]
    pub fn delete_team_request(id: Option<String>) -> Result<Request, String> {
        Self::delete::<api::Team>(Self::team_id_path_params(id), Self::with_children_qs_params())
    }
}
