use serde::{de::DeserializeOwned};
use crate::http::{ContentType, get as httpGet, post as httpPost};
use anyhow::{Result as AnyhowResult};
use std::env::var;

pub async fn snyk_http_helper<T: DeserializeOwned>(url: String, http_type: String) -> AnyhowResult<Option<T>> {
    let token: String = get_snyk_access_token();
    println!("{} from: {}", http_type, url);

    //REVIEW: There is probably a better way to do this!
    if http_type == "POST" {
        return httpPost(
            &url,
            ContentType::Json,
            token.as_str(),
            None::<String>,
        ).await;
    }
    else {
        return httpGet(
            &url,
            ContentType::Json,
            token.as_str(),
            None::<String>,
        ).await;
    }

}

fn get_snyk_access_token() -> String {
    let snyk_token = "SnykToken";
    
    match var(snyk_token) {
        Ok(token) => return token,
        Err(_) => panic!("Environment variable {} is not set", snyk_token),
    }
}