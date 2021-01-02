use juniper::RootNode;

use crate::dataserver::graphql::{
    mutations::Mutation,
    queries::Query
};

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
