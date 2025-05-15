# 插入数据

在 Rust 的 Diesel ORM 中，插入数据是将 Rust 结构体的数据保存到数据库表的过程。Diesel 提供了类型安全的插入
API，支持单条插入、批量插入和事务处理。本文以 diesel-demo 项目的 posts 表为例，讲解如何插入数据。

在 Diesel 中插入数据通常使用 `.insert_into()` 和 `.values()` 方法。

**准备工作**: 你需要一个带有 `#[derive(Insertable)]` 的结构体，并指定表名：

```rust
#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
```

## 插入单条数据

以下是插入单条帖子的示例，运行在命令行程序中。

<<< @/../examples/ch04_usage_insert/src/bin/single_insert.rs

`get_result(conn)`：执行插入，返回插入的记录（Post），包含数据库生成的 id。如果您对插入后不需要返回记录，可以使用
`.execute(conn)` 操作，执行插入，返回 `Result<usize>`（影响行数）。

```rust
fn main() {
    let row = diesel::insert_into(posts)
    .values(&new_post)
    .execute(conn)
    .unwrap();
}
```

## 批量插入

如果我们想一次插入多行，我们可以通过传递 `&Vec` 或上面使用的任何形式的切片来实现。请记住，您始终在此处传递引用。

<<< @/../examples/ch04_usage_insert/src/bin/batch_insert.rs

批量插入后返回插入的数据，使用 `.get_results()` 获取集合。

Diesel 支持在插入时对某些字段使用数据库中的默认值（DEFAULT 关键字）。这在某些字段有默认值（如自动递增 ID、默认时间戳、布尔值等）时非常实用。

如果你不为某个字段赋值，Diesel 会尝试使用数据库中定义的默认值。实现方法是使用 `Option<T>` 类型，并在插入时设置为 `None`。

```rust
let new_post = NewPost {
title: "使用默认值".into(),
body: "未指定 published，使用默认值".into(),
published: None, // 表示使用数据库的 DEFAULT 值
};
```

## RETURNING 子句

在上面的插入、批量插入我们已经学习的 `RETURNING` 子句，它就是这么简单！不过该小结将全部讲解。

在 SQL 中，RETURNING 子句可以让你在执行 `INSERT`、`UPDATE` 或 `DELETE` 后立即获取受影响的行的数据，而不需要再次查询数据库。

### 使用场景示例

假设你插入一条记录，并想获取这条记录的 id（通常是自增主键）：

:::code-group

```rust [插入语句 + 获取结果]
fn main() {
    let inserted_post: Post = diesel::insert_into(posts)
    .values(&new_post)
    .get_result(conn)?; // 隐含 RETURNING *
}

```

```sql [➡️ 对应的SQL]
INSERT INTO posts (title, body)
VALUES ('标题', '内容') RETURNING *;
```

:::

如果你只关心 `id` 字段，也可以在 Diesel 中显式指定：

```rust
fn main() {
    let inserted_id: i32 = diesel::insert_into(posts)
    .values(&new_post)
    .returning(posts::id)
    .get_result(conn)?;
}
```

> [!TIP] 强调
> 在支持 RETURNING 子句的后端（例如 PostgreSQL 和 SQLite）上，我们也可以从插入操作中获取数据。在 SQLite 后端，可以使用功能标志
`returning_clauses_for_sqlite_3_35` 启用对 RETURNING 子句的支持。MySQL 不支持 RETURNING 子句。要获取所有插入的行，我们可以调用
`.get_results` 而不是 `.execute`。


### 什么是 .returning(...)

Diesel 支持这一特性，通过 `.returning(...)` 方法让我们自定义希望返回的字段。

如果你不显式调用 `.returning(...)`，Diesel 默认会生成 `RETURNING *`，也就是返回整行数据，并尝试填充完整的模型结构体。

如果表结构比较复杂，比如 `users` 表有十几个字段，甚至包含敏感字段（如密码），就不适合直接返回整个模型。此时可以单独定义一个只包含需要字段的结构体作为返回值，让代码更安全、性能更好、结构更清晰。

```rust
#[derive(Selectable)]
#[diesel(table_name = users)]
pub struct ReturnedUser {
    pub id: i32,
    pub name: String,
}
```

- `Selectable` 宏用于让 Diesel 支持 `.returning()`、`.select()` 等操作。

- `#[diesel(table_name = ...)]` 是必须的，告诉 Diesel 这个结构体绑定哪张表。

> [!WARNING] ⚠️ 注意
> 只有 PostgreSQL 支持 `.returning()`，MySQL 和 SQLite 会忽略它。

### 总结

| 方法               | 行为                            |
|------------------|-------------------------------|
| `.execute()`     | 只执行语句，返回受影响的行数                |
| `.get_result()`  | 返回单条记录，对应 SQL 的 `RETURNING *` |
| `.get_results()` | 返回多条记录                        |
| `.returning(x)`  | 返回指定字段                        |

> [!TIP] 导航
> [前往 GitHub 查看完整示例代码](https://github.com/nonfan/diesel-demo/tree/docs/examples/ch04_usage_insert)
