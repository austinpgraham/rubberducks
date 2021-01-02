use duplicate::duplicate;
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

use crate::dataserver::models::{
    AuroraReaderConnection,
    AuroraWriterConnection,
    AuroraConnection,
    DBConnection
};

/// Implement a simple request guard for fetching one of the list connection
/// types to the database. This gets rid of any unecessary duplication.
#[duplicate(connection_type; [AuroraReaderConnection]; [AuroraWriterConnection])]
impl<'a, 'r> FromRequest <'a, 'r> for connection_type {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let connection_resource = request.guard::<State<AuroraConnection>>()?;
        let pool = Self::get_pool_from_state(&connection_resource);
        match pool.get() {
            Ok(conn) => Outcome::Success(connection_type(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
