use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{books,pages};

#[derive(Queryable,Selectable,Deserialize,Serialize, Identifiable)]
#[derive(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable,Selectable,Deserialize,Serialize, Identifiable, Associations)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = crate::schema::pages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}