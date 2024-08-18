use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use crate::schema::user;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub birthdate: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub name: &'a String,
    pub birthdate: &'a PgTimestamp,
}