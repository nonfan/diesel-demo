use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub remark: String,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub remark: String,
}
