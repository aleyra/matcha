pub mod urls;
pub mod models;
pub mod schema;

use diesel::data_types::PgTimestamp;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url ="postgresql://username42:password42@localhost:5432/matcha42";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, name: &String, birthdate: &PgTimestamp) -> models::user::User {

    let new_post = models::user::NewUser { name, birthdate };

    diesel::insert_into(schema::user::table)
        .values(&new_post)
        .returning(models::user::User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}