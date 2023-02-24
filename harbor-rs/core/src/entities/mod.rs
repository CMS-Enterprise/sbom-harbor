use aqum::mongo_doc;
use aqum::mongodb::MongoDocument;

mod codebase;
mod member;
mod project;
mod team;
mod token;

pub use codebase::*;
pub use member::*;
pub use project::*;
pub use team::*;
pub use token::*;

mongo_doc!(Codebase);
mongo_doc!(Member);
mongo_doc!(Project);
mongo_doc!(Team);
mongo_doc!(Token);

// pub trait Entity: Any {
//     fn discriminator(&self) -> Discriminator;
//     fn type_name(&self) -> String;
//     fn is_child_of(&self, parent_id: String) -> bool;
//     fn as_any(&self) -> &dyn Any;
//     // fn to_dynamo_entity(&self, partition_key: String, id: String) -> dyn DynamoEntity;
// }
//
// macro_rules! entity {
//     ($entity:ty, $api:ty) => {
//         impl Entity for $entity {
//             fn as_any(&self) -> &dyn Any {
//                 self
//             }
//
//             fn discriminator(&self) -> Discriminator {
//                 let type_name = self.type_name();
//                 let type_name = type_name.split(':').next_back();
//
//                 Discriminator::from_str(type_name.unwrap()).unwrap_or(Discriminator::Unknown)
//             }
//
//             fn type_name(&self) -> String {
//                 format!("{}", std::any::type_name::<$entity>())
//             }
//
//             fn is_child_of(&self, parent_id: String) -> bool {
//                 self.parent_id == parent_id
//             }
//         }
//
//         impl DynamoEntity for $entity {
//             fn schema(&self) -> Schema {
//                 Schema{
//                     table: "e1118-HarborTeams-use1".to_string(),
//                     partition_attribute: Some("TeamId".to_string()),
//                     sort_attribute: Some("EntityKey".to_string()),
//                 }
//             }
//
//             fn key_condition_expression(&self) -> Option<String> {
//                 default_key_condition_expression(&self.schema())
//             }
//
//             fn partition_key(&self) -> Option<String> {
//                 Some(self.partition_key.clone())
//             }
//
//             fn sort_key(&self) -> Option<String> {
//                 self.discriminator().to_sort_key(&self.id)
//             }
//
//             fn load(&mut self) -> Result<(), aqum::Error> {
//                 // Load for writing.
//                 if self.partition_key.is_empty() {
//                     if self.id.is_empty() {
//                         return Err(aqum::Error::Write("id is required".to_string()));
//                     }
//                     match self.discriminator() {
//                         Discriminator::Team => {
//                             self.partition_key = self.id.clone();
//                             self.parent_id = self.id.clone();
//                         },
//                         _ => {
//                             if self.parent_id.is_empty() {
//                                 return Err(aqum::Error::Write("parent_id is required".to_string()));
//                             }
//                             self.partition_key = self.parent_id.clone();
//                         },
//                     }
//
//                     self.sort_key = self.discriminator().to_sort_key(&self.id).unwrap();
//                 } else {
//                     // Load for read.
//                     self.id = Discriminator::parse_id(&self.partition_key, &self.sort_key)?;
//                 }
//
//                 Ok(())
//             }
//
//             fn as_any(&self) -> &dyn Any {
//                 self
//             }
//         }
//     }
// }
//
// pub(crate) use entity;
//
// entity!(Team, crate::api::Team);
// entity!(Member, crate::api::Member);
// entity!(Project, crate::api::Project);
// entity!(Codebase, crate::api::Codebase);
// entity!(Token, crate::api::Token);
//
// #[derive(Clone, Debug, PartialEq)]
// pub enum Discriminator {
//     Team,
//     Member,
//     Project,
//     Codebase,
//     Token,
//     Unknown,
// }
//
// impl Discriminator {
//     pub fn to_sort_key(&self, id: &String) -> Option<String> {
//         match *self {
//             Discriminator::Team => Some(self.to_string()),
//             _ => Some(format!("{}#{}", self.to_string(), id.clone()))
//         }
//     }
//
//     pub fn parse_id(partition_key: &String, sort_key: &String) -> Result<String, aqum::Error> {
//         // If the sort key is for the partition key, the id is the partition key.
//         if !sort_key.contains("#") {
//             return Ok(partition_key.clone());
//         }
//
//         let parts: Vec<&str> = sort_key.split("#").collect();
//         if parts.len() != 2 {
//             return Err(aqum::Error::Query(format!("schema corruption in EntityKey: {}", sort_key)));
//         }
//
//         // Validate the type is supported.
//         Discriminator::from_str(parts[0])?;
//
//         let id = parts[1];
//
//         Ok(id.to_string())
//     }
// }
//
// impl FromStr for Discriminator {
//     type Err = aqum::Error;
//
//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         match input.to_lowercase().as_str() {
//             "team"  => Ok(Discriminator::Team),
//             "member"  => Ok(Discriminator::Member),
//             "project"  => Ok(Discriminator::Project),
//             "codebase" => Ok(Discriminator::Codebase),
//             "token" => Ok(Discriminator::Token),
//             _      => Ok(Discriminator::Unknown),
//         }
//     }
// }
//
// impl Display for Discriminator {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             Discriminator::Team => write!(f, "team"),
//             Discriminator::Project => write!(f, "project"),
//             Discriminator::Codebase => write!(f, "codebase"),
//             Discriminator::Member => write!(f, "member"),
//             Discriminator::Token => write!(f, "token"),
//             Discriminator::Unknown => write!(f, "unknown"),
//         }
//     }
// }
