# 关联关系

Diesel 的关联关系（`Associations`）功能用于处理表与表之间的关系，比如**一对多（One-to-Many）、joins 或多对多（Many-to-Many)**
。这使你能够通过模型之间的关系进行联表查询，避免手动写
SQL。

> [!WARNING] 学习基础
> 本指南假定您已完成 [快速开始](/intro/getting-started)。

## 准备工作

让我们为本指南创建一个新项目：

```bash
cargo new --lib diesel_relations
cd diesel_relations
```

和以前一样，让我们将 Diesel 和 dotenvy 添加到我们的依赖项中。

```bash
cargo add diesel --features "postgres"
cargo add dotenvy
```

您的 `Cargo.toml` 文件现在应该包含类似于以下的条目：

```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres"] }
dotenvy = "0.15.6"
```

以及一个 `.env` 将 Diesel 指向正确的数据库。

```bash
echo DATABASE_URL=postgres://username:password@localhost/diesel_relations > .env
```

现在我们可以通过运行以下命令来设置 Diesel：

```bash
diesel setup
```

## 一对多关系

我们需要创建两个想要连接的不同对象，对于第一个一对多示例，让我们创建书籍和页面。一本书可以有很多页，但一页只能属于一本书。

### 创建迁移文件

```bash
diesel migration generate create_books
diesel migration generate create_pages
```

接下来为迁移文件编写SQL：

:::code-group

```sql [up.sql(books)]
CREATE TABLE books
(
    id    SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
);
```

```sql [down.sql(books)]
DROP TABLE books;
```

```sql [up.sql(pages)]
CREATE TABLE pages
(
    id          SERIAL PRIMARY KEY,
    page_number INT     NOT NULL,
    content     TEXT    NOT NULL,
    book_id     INTEGER NOT NULL REFERENCES books (id)
);
```

```sql [down.sql(pages)]
DROP TABLE pages;
```

:::

我们可以应用新的迁移：

```bash
diesel migration run

# 让我们确保 down 迁移也是正确的：
diesel migration redo -n 2
```

> [!TIP] redo知识点
> `redo` 命令会先撤销（revert）指定的迁移，然后重新应用（reapply）这些迁移。`-n` 是 Diesel CLI 的参数，表示指定操作的迁移数量。
`-n 2` 意味着对最近的 2 个迁移执行 redo 操作。


迁移成功后自动创建 `schema.rs` 文件，然后手动编写 `models.rs` 模型文件，如下：

```rust
use diesel::prelude::*;
use crate::schema::{books, pages};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}
```

Diesel 中的关联始终是子级到父级的。您可以使用 `#[diesel(belongs_to)]` 声明两个记录之间的关联。首先，我们需要添加
`#[derive(Associations)]` ，这样我们就可以向 Page 添加 `#[diesel(belongs_to(Book))]` 。这表示页面属于书籍，从而反映了我们的一对多关系。

默认情况下，diesel 会假设你的结构体包含一个字段。例如，对于给定的示例 `user_id`。如果你的外键字段有不同的名称，你可以通过
Foreign_key 选项指定：`#[diesel(belongs_to(Book, Foreign_key = xxx_id))]`

### 读取数据

<<< @/../examples/ch09_features_relations/src/bin/belonging_to.rs

`Page::belonging_to`
允许查询与一个或多个父记录关联的所有子记录。在本例中，它将加载书名为“Momo”的书籍的所有页面。此函数会生成一个用于加载这些数据的查询。它不会执行查询，因此稍后可以向查询中添加其他子句。生成的查询相当于
`SELECT * FROM pages WHERE book_id IN(…)`，其中包含一个从函数输入中获取的给定书籍 ID 列表。

加载一本书的所有页面是一个简单的查询，如果我们想加载每本书的所有页面，它会变得更加复杂。这是一个典型的 ORM 问题，其中框架有时会在所谓的
N+1 查询问题中运行。该问题描述了框架首先天真地加载所有书籍，然后对每本书执行一个查询以接收相关页面的情况。这种方法对性能不利，因为它执行未绑定数量的查询。

Diesel 的关联 API 通过为以下情况提供特殊定制的 API 来避免该问题：

<<< @/../examples/ch09_features_relations/src/bin/belonging_to_by_api.rs

这里我们使用与之前类似的结构，通过 `Page::belonging_to` 构建相关查询来加载给定书籍列表的所有页面。与之前的重要区别在于，我们现在将书籍切片作为参数传递。这将再次构建与之前
`SELECT * FROM pages WHERE book_id IN(…)` 查询等效的语句。这里的重要区别在于，我们稍后会使用 `.grouped_by`
函数将每个页面分组到正确的书籍中。此代码块中总共执行了两个查询，与查询的书籍和页面数量无关。

#### 返回序列化数据结构

加载关联数据的一个常见用例是返回序列化数据结构，例如:

```json
[
  {
    "id": 1,
    "title": "Momo",
    "pages": [
      page,
      page
    ]
  }
]
```

<<< @/../examples/ch09_features_relations/src/bin/belonging_to_by_api_for_serialize.rs

`#[serde(flatten)]` 可用于展平 Book 的子属性（如 id, title），生成简洁的 JSON 输出，适配 `{book子属性, pages}` 需求。

如果想序列化输出终端，切记不要忘记了对 `Post` 和 `Book` 实现 `Serialize` trait。

## Joins

我们目前使用 `diesel::associations` 模块提供的 API 加载了一本书的所有页面。这个 API
是专门为“父子关系”（比如：一本书有很多页面）设计的，但不适用于反过来的关系（比如：从页面查它属于哪本书）。

要处理这种“反向关系”，推荐使用 SQL 的 JOIN 操作。

Diesel 提供了两种 JOIN：

- `INNER JOIN`：要求关联的数据一定存在（比如页面必须属于某本书）。
- `LEFT JOIN`：即使没有关联的数据（比如某个页面没有对应的书），也会加载出来。

### INNER JOIN

`QueryDsl::inner_join` 允许在不同的表之间构建 INNER JOIN 语句。

<<< @/../examples/ch09_features_relations/src/bin/inner_join.rs

`QueryDsl::inner_join()` 会修改构建好的查询，在 SQL 中加入一个 INNER JOIN 子句，这个 JOIN 的连接条件（ON 子句）可以根据你在
schema.rs 文件中使用的 joinable! 宏自动推导出来。

另外，你也可以通过 `JoinDsl::on` 方法手动指定自定义的 ON 条件。

如果你没有显式指定 `select` 子句，那么 Diesel 会默认选择两边表的所有字段，并返回一个由这两边默认字段组成的元组（tuple）。这个结果可以被反序列化成
Rust 的元组类型，或者任何实现了 Queryable 的兼容类型。

可以链接多个联接以联接多个表。联接的嵌套控制确切联接的表。这意味着以下两个语句不相等:

```rust
users::table.inner_join(posts::table.inner_join(comments::table));

// Results in the following SQL
// SELECT * FROM users
// INNER JOIN posts ON users.id = posts.user_id
// INNER JOIN comments ON post.id = comments.post_id

users::table.inner_join(posts::table).inner_join(comments::table);

// Results in the following SQL
// SELECT * FROM users
// INNER JOIN posts ON users.id = posts.user_id
// INNER JOIN comments ON users.id = comments.user_id
```

### LEFT JOIN

`QueryDsl::left_join` 允许在不同的表之间构建 LEFT JOIN 语句。

<<< @/../examples/ch09_features_relations/src/bin/left_join.rs

使用 `left_join`（左连接）和使用 `inner_join`（内连接）很相似，但有一个很重要的区别： 左连接中，从被连接的表中返回的所有列都被认为是“可以为
null”的。这会带来一些具体影响：

- 查询结果的结构不同：

```rust
books::table.left_join(pages::table).load(conn)

// 返回结果是 (Book, Option<Page>)
```

每本书一定会有（所以是 Book），但它对应的页面 Page 可能没有（所以是 `Option<Page>`）。

- 手动指定查询字段时要特别处理：

如果你手动写 `.select(...)` 指定要查询的字段，来自被连接表的字段（也就是可能为 null 的字段）必须用 `.nullable()` 包裹起来，否则
Diesel 不知道该字段是不是可能为 null，就会报错。

你可以对如下情况都使用 `.nullable()` 来标记 ：

:::code-group

```rust [单个字段]
use diesel::dsl::NullableExpressionMethods; // [!code focus]

fn main() {
    let results = books::table
    .left_join(pages::table)
    .select((books::title, pages::number.nullable())) // [!code focus]
    .load::<(String, Option<i32>)>(conn)?;
}
```

```rust [表达式]
use diesel::dsl::{NullableExpressionMethods, Add}; // [!code focus]

fn main() {
    let results = books::table
    .left_join(pages::table)
    .select((books::title, (pages::number + 1).nullable())) // [!code focus]
    .load::<(String, Option<i32>)>(conn)?;
}
```

```rust [元组]
use diesel::dsl::NullableExpressionMethods; // [!code focus]

fn main() {
    let results = books::table
    .left_join(pages::table)
    .select((books::title, (pages::id, pages::number).nullable())) // [!code focus]
    .load::<(String, Option<(i32, i32)>)>(conn)?;
}

```

:::

## 多对多

我们目前有 books（书籍）表，其中一本书有多页（pages），但书籍也有作者（author）。更准确地说，一本书可以有多个作者，一个作者也可以有多本书。这就是一个多对多（many-to-many）关系。

Diesel 并没有 `has_many` 的概念，所以我们要通过创建一个中间表（join table）`books_authors` 来实现这种关系，并且在这个中间表里分别
`belongs_to` 到 books 和 authors。

### 创建迁移文件

```bash
diesel migration generate create_authors
diesel migration generate create_books_authors
```

编写迁移文件SQL内容：

:::code-group

```sql [up.sql(authors)]
CREATE TABLE authors
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);
```

```sql [down.sql(authors)]
DROP TABLE authors;
```

```sql [up.sql(books_authors)]
CREATE TABLE books_authors
(
    book_id   INTEGER REFERENCES books (id),
    author_id INTEGER REFERENCES authors (id),
    PRIMARY KEY (book_id, author_id)
);
```

```sql [down.sql(books_authors)]
DROP TABLE books_authors;
```

:::

### Model 模型

迁移成功后自动创建 `schema.rs` 文件，然后手动继续编写 `models.rs` 模型文件，如下：

```rust
use diesel::prelude::*;

use crate::schema::{books, pages, authors, books_authors};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Book))] // [!code highlight]
#[diesel(belongs_to(Author))] // [!code highlight]
#[diesel(table_name = books_authors)] // [!code highlight]
#[diesel(primary_key(book_id, author_id))] // [!code highlight]
pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}
```

重要的部分是给 BooksAuthor 两个 `belongs_to` 指向 book 和 author。

### 读取数据

<<< @/../examples/ch09_features_relations/src/bin/many_to_many.rs

如前所述，`BookAuthor::belonging_to` 会构建一个查询，我们可以在此基础上继续链式调用其他条件。在这个例子中，我们通过连接（join）books 表，来包含相关的书籍，并且只选择对填充 Book 类型有用的列。这样就实现了加载某个作者的所有书籍。

同样的方法也可以反过来应用，加载给定书籍的所有作者（示例省略）。

和以前一样，我们可以使用这种方法来加载所有作者及其相关书籍的列表：

<<< @/../examples/ch09_features_relations/src/bin/many_to_many_all.rs

这个示例展示了如何在 Diesel 中处理多对多关系：一个作者（Author）可以写多本书（Book），一本书也可以由多个作者共同创作。为此，我们通过中间表 `books_authors` 建立关联。

> [!TIP] 导航
> [前往 GitHub 查看完整示例代码](https://github.com/nonfan/diesel-demo/tree/docs/examples/ch09_features_relations)
