/**
 * Primary entry point for the dataserver for
 * Rubber Ducks
 */
use rocket::config::{
    Config,
    Environment
};

pub mod models;
pub mod graphql;
pub mod guards;

#[rocket::get("/health")]
pub fn health_check() -> &'static str {
    "Server is alive."
}

pub fn start_dataserver(host: &str, port: u16, workers: u16) {
    let secret_key = std::env::var("RD_SECRET_KEY").expect("No secret key was set. Set RD_SECRET_KEY to a secret string fix this.");

    let config = Config::build(Environment::Production)
                        .address(host)
                        .port(port)
                        .workers(workers)
                        .secret_key(secret_key)
                        .finalize()
                        .expect("Failed to establish configuration for app.");
    let app = rocket::custom(config);
    app.manage(models::get_connection())
       .manage(graphql::schema::create_schema())
       .mount("/", rocket::routes![
           health_check,
           graphql::get_graphql_handler,
           graphql::post_graphql_handler,
           graphql::get_graphql_source
        ])
       .launch();
}
