use duplicate::duplicate;
use std::env;
use diesel::{
    pg::PgConnection,
    r2d2::{
        ConnectionManager,
        Pool,
        PooledConnection
    }
};

pub mod users;
pub mod schema;

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
}


impl DBConnection for AuroraReaderConnection {
    fn get_connection_pool() -> PgPool {
        let database_url = env::var("RD_READER_URL").expect("Failed to get database URL from environment.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create database connection pool.")
    }
}

impl DBConnection for AuroraWriterConnection {
    fn get_connection_pool() -> PgPool {
        let database_url = env::var("RD_WRITER_URL").expect("Failed to get database URL from environment.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create database connection pool.")
    }
}

pub struct AuroraConnection {
    pub reader: PgPool,
    pub writer: PgPool
}

#[macro_export]
macro_rules! get_pool {
    ($connection:expr, $connection_type:expr) => {
        match $connection_type {
            ConnectionType::Reader => $connection.reader.get().unwrap(),
            ConnectionType::Writer => $connection.writer.get().unwrap()
        }
    }
}
