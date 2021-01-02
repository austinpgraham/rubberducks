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

use crate::dataserver::models::AuroraConnection;

/// Implement a simple request guard for fetching one of the list connection
/// types to the database. This gets rid of any unecessary duplication.
impl<'a, 'r> FromRequest <'a, 'r> for &'r AuroraConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<State<AuroraConnection>>() {
            Outcome::Success(conn) => Outcome::Success(conn.inner()),
            Outcome::Failure(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            _ => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
