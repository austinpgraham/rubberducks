use juniper_rocket::{
    GraphQLRequest,
    GraphQLResponse
};
use rocket::State;
use crate::dataserver::models::AuroraConnection;

pub mod context;
pub mod schema;
pub mod queries;
pub mod mutations;

#[get("/?<request>")]
pub fn get_graphql_handler(
    request: GraphQLRequest,
    gql_schema: State<schema::Schema>,
    context: &AuroraConnection,
) -> GraphQLResponse {
    request.execute(gql_schema.inner(), context)
}

#[post("/", data = "<request>")]
pub fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    gql_schema: State<schema::Schema>,
    context: &AuroraConnection,
) -> GraphQLResponse {
    request.execute(gql_schema.inner(), context)
}
