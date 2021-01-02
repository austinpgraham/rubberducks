use juniper::graphql_object;
use crate::dataserver::models::AuroraConnection;

#[derive(Debug)]
pub struct Mutation {}

graphql_object!(Mutation: AuroraConnection |&self| {});
