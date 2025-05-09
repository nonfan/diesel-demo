<img src="./logo.svg"/>

## Diesel 对于 Rust 是一种安全、 可扩展的 ORM 和查询生成器

## 意想不到的 BUG :bug:

:one: 错用 `derive`，应用 `diesel`

```rust
// 假设能导入Page，确不能导入Book，然后 models 文件实在一摸一样，究竟错哪了？
use rust_demo::models::{Page,Book};
```
**看到了哪里不一样了吗 :question:**

```rust
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
```

```rust
#[derive(table_name = crate::schema::books)]
#[diesel(table_name = crate::schema::pages)]
```