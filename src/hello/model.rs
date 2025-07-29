use diesel::{mysql::Mysql, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::hello)]
#[diesel(check_for_backend(Mysql))]
pub struct Hello {
    pub id: i32,
    pub title: String,
}
