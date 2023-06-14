use platform::hyper;
use platform::hyper::ContentType;
use serde::{Deserialize, Serialize};

use crate::Error;

#[allow(dead_code)]
fn org_tags_url() -> String {
    "https://api.snyk.io/hidden/org_tags?version=2022-12-09~experimental".to_string()
}

/// A purpose build Snyk HTTP Client.
#[derive(Debug)]
pub struct Client {
    token: String,
    inner: hyper::Client,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        let inner = hyper::Client::new();
        Self { token, inner }
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    #[allow(dead_code)]
    pub async fn org_tags(&self) -> Result<Vec<OrgTag>, Error> {
        let response: Option<OrgTagObjectListGetResponse> = self
            .inner
            .get(
                &org_tags_url(),
                ContentType::Json,
                &self.token(),
                None::<OrgTagObjectListGetResponse>,
            )
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        match response {
            None => Err(Error::Snyk("snyk failed to list org tags".to_string())),
            Some(r) => Ok(r.data),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgTag {
    #[serde(rename = "attributes")]
    pub attributes: Box<OrgTagAttributes>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "is_personal", skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<SelfLink>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgTagAttributes {
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgTagObjectListGetResponse {
    #[serde(rename = "data")]
    pub data: Vec<OrgTag>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<PaginatedLinks>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelfLink {
    // Allowing the following lint since the spec generated code seems to violate this suggestion.
    #[allow(clippy::box_collection)]
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<Box<String>>,
}

// The models below this are indirectly referenced by the above models.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JsonApi {
    /// Version of the JSON API specification this server supports.
    #[serde(rename = "version")]
    pub version: String,
}

// WARN: I had to change from the generated code to Option<String> despite the spec.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginatedLinks {
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::services::snyk::client::Client;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_list_org_tags() -> Result<(), Error> {
        let token = std::env::var("SNYK_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let tags = client.org_tags().await?;

        assert!(!tags.is_empty());

        for tag in tags {
            let id = tag.id.unwrap().to_string();
            let link = tag.links.unwrap().param_self.unwrap().to_string();
            assert!(link.contains(&id));
        }

        Ok(())
    }
}
