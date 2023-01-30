use anyhow::{anyhow, Context, Error};
use aquia::config::Config;
use aquia::dynamo::{Context as DynamoContext, Error as DynamoError, Entity as DynamoEntity, Store};
use aquia::lambdahttp::{Contex, Error as LambdaError, param_as_str, DynamoRestContext, DynamoRestService, to_entity};
use async_trait::async_trait;
use aws_lambda_events::query_map::QueryMap;
use lambda_http::{Request, RequestExt};
use lambda_http::http::Method;
use serde::{Deserialize, Serialize};

use crate::{api, entities};
use crate::entities::{Entity};


pub struct TeamService {}
impl DynamoRestService<'_, api::Team, entities::Team, TeamContext> for TeamService {}

pub struct TeamContext {
    method: Method,
    pub id: String,
    pub children: bool,
    pub team: Option<api::Team>,
}

impl DynamoRestContext<'_, api::Team, entities::Team> for TeamContext {
    fn as_dto(&self) -> api::Team {
        self.team.clone().unwrap()
    }
}

impl aquia::lambdahttp::Context for TeamContext {
    fn from_request(request: &Request) -> Result<Self, LambdaError> {
        Ok(Self {
            method: request.method().clone(),
            id: with_team_id(&request.path_parameters())?,
            team: request.payload()?,
            children: with_children(&request.query_string_parameters())?,
        })
    }
}

impl DynamoContext<'_, entities::Team> for TeamContext {
    fn as_dynamo_entity(&self) -> Result<entities::Team, DynamoError> {
        let method = self.method.clone();
        match method {
            Method::GET | Method::DELETE => {
                let mut query = entities::Team::new("".to_string());
                query.partition_key = self.id.clone();
                Ok(query)
            },
            Method::POST | Method::PUT => {
                let dto = match &self.team {
                    None => {
                        return Err(aquia::dynamo::Error::EntityError("team is required".to_string()));
                    }
                    Some(dto) => dto,
                };
                let entity: entities::Team = to_entity(dto)
                    .map_err(|e| {
                        return DynamoError::EntityError(e.to_string());
                    })?;
                Ok(entity)
            },
            _ => {
                Err(DynamoError::ConfigError(format!("method invalid {}", self.method.to_string())))
            }
        }
    }

    fn is_aggregate_root(&self) -> bool {
        self.children
    }
}

fn with_team_id(params: &QueryMap) -> Result<String, LambdaError> {
    let id = param_as_str("teamId", params)?;
    Ok(id.unwrap())
}

fn with_project_id(params: &QueryMap) -> Result<String, LambdaError> {
    let id = param_as_str("projectId", params)?;
    Ok(id.unwrap())
}

fn with_codebase_id(params: &QueryMap) -> Result<String, LambdaError> {
    let id = param_as_str("codebaseId", params)?;
    Ok(id.unwrap())
}

fn with_children(params: &QueryMap) -> Result<bool, LambdaError> {
    match params.first("children") {
        None => Ok(false),
        Some(c) => {
            let c = c.to_lowercase();
            let c = c.as_str();
            match c {
                 "" | "true" => Ok(true),
                _ => Ok(false),
            }
        }
    }
}
