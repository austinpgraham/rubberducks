use juniper::graphql_object;
use crate::dataserver::models::AuroraConnection;

#[derive(Debug)]
pub struct Query {}

graphql_object!(Query: AuroraConnection |&self| {});
