use diesel::{
    self,
    prelude::*
};
use chrono::NaiveDateTime;

use crate::{
    get_pool,
    dataserver::models::{
        AuroraConnection,
        ConnectionType,
        schema::users
    }
};


#[derive(Insertable, Queryable, Identifiable, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub email: String,
    pub signup_source: String,
    pub access_token: String,
    pub token_expire_time: NaiveDateTime,
    pub profile_picture: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub last_logged_in_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl User {
    pub fn create(new_user: &User, connection: &AuroraConnection) -> QueryResult<Self> {
        let pool = get_pool!(connection, ConnectionType::Writer);
        diesel::insert_into(users::table)
               .values(new_user)
               .get_result(&pool)
    }

    pub fn get(id: i64, connection: &AuroraConnection) -> QueryResult<Self> {
        let pool = get_pool!(connection, ConnectionType::Reader);
        users::table.find(id).first::<Self>(&pool)
    }
}
