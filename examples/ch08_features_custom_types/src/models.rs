use diesel::prelude::*;
use serde::{Deserialize};
use crate::custom_email_type::Email;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub email: Email,
}