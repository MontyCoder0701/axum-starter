use diesel::{mysql::Mysql, prelude::*};

use super::schema::user;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(Mysql))]
#[derive(serde::Serialize)]

pub struct User {
    pub id: i32,
    pub name: String,
}
