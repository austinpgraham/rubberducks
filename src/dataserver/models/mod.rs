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
pub fn get_connection() -> AuroraConnection {
    AuroraConnection {
        reader: AuroraReaderConnection::get_connection_pool(),
        writer: AuroraWriterConnection::get_connection_pool()
    }
}

/// An alias struct for a connection to Aurora in both the
/// reader and writer instances.
#[duplicate(connection_type; [AuroraReaderConnection]; [AuroraWriterConnection])]
pub struct connection_type(pub PooledConnection<ConnectionManager<PgConnection>>);

pub trait DBConnection {
    fn get_connection_pool() -> PgPool;
    fn get_pool_from_state(connection: &AuroraConnection) -> &PgPool;
}


impl DBConnection for AuroraReaderConnection {
    fn get_connection_pool() -> PgPool {
        let database_url = env::var("RD_READER_URL").expect("Failed to get database URL from environment.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create database connection pool.")
    }

    fn get_pool_from_state(connection: &AuroraConnection) -> &PgPool {
        &connection.reader
    }
}

impl DBConnection for AuroraWriterConnection {
    fn get_connection_pool() -> PgPool {
        let database_url = env::var("RD_WRITER_URL").expect("Failed to get database URL from environment.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create database connection pool.")
    }

    fn get_pool_from_state(connection: &AuroraConnection) -> &PgPool {
        &connection.writer
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

pub struct AuroraConnection {
    pub reader: PgPool,
    pub writer: PgPool
}
