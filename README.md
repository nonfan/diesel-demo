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

:two: `books::table` 这种查询方式，老是报错像这样：

> No function or associated item `table` found in the current scope for struct `table` [E0599]

```rust
fn func() {
    let momo = books::table
    .filter(books::title.eq("Momo"))
    .select(Book::as_select())
    .get_result(&mut conn)?;
}
```

那是因为books变量被污染：

```rust
// 该引用也同样有books，但这不是你需要的哪个books
use crate::schema::books::dsl::*;

// 这才是你需要的books
use crate::schema::books;
```

避免全局变量污染，但你需要引用 `use crate::schema::books::dsl::*` 请在函数内引用！

:three: 使用 `Table::as_returninng()` 的关键

```rust
// 必须引入它
use diesel::prelude::*;
```