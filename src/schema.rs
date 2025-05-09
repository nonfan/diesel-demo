// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        page_number -> Int4,
        content -> Text,
        book_id -> Int4,
    }
}

diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    pages,
);
