use duplicate::duplicate;
use std::{
    env,
    ops::Deref
};
use diesel::{
    pg::PgConnection,
    r2d2::{
        ConnectionManager,
        Pool,
        PooledConnection
    }
};
use rocket::{
    request::{
        self,
        FromRequest
    },
    http::Status,
    Outcome,
    Request,
    State
};

pub mod users;

/// We want mutations to grab a connection to the writer,
/// while everything else gets a connection from the reader.
/// This will help with overall connection pooling.
pub enum ConnectionType {
    Reader,
    Writer
}

/// Defines a simple type outlining a pool of Postgres connections.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Get the Postgres connection pool.
pub fn get_connection(c_type: ConnectionType) -> PgPool {
    let url_string = match c_type {
        ConnectionType::Reader => "RD_READER_URL",
        ConnectionType::Writer => "RD_WRITER_URL"
    };
    let database_url = env::var(url_string).expect("Failed to get database URL from environment.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create database connection pool.")
}

/// An alias struct for a connection to Aurora in both the
/// reader and writer instances.
pub struct AuroraReaderConnection(pub PooledConnection<ConnectionManager<PgConnection>>);
pub struct AuroraWriterConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

#[duplicate(connection_type; [AuroraReaderConnection]; [AuroraWriterConnection])]
impl<'a, 'r> FromRequest <'a, 'r> for connection_type {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(connection_type(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

/// Defines a simple dereference implementation to grab out the
/// base connection object from diesel.
#[duplicate(connection_type; [AuroraReaderConnection]; [AuroraWriterConnection])]
impl Deref for connection_type {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
