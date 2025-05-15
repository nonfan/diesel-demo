# 数据模型

在 Diesel 中，数据模型是用 Rust 的结构体（struct）来表示数据库中的一张表或一条记录。它是 Diesel 将 Rust 与 SQL
世界连接的核心方式之一。每一张数据库表通常会有几个不同用途的结构体对应。

这些结构体通常使用 Diesel 提供的一组派生宏（derive macro）来标记其用途，例如：

- `#[derive(Queryable)]`
- `#[derive(Insertable)]`
- `#[derive(AsChangeset)]`
- `#[derive(Selectable)]`
- `#[derive(Identifiable)]`

## Queryable 模型

这个结构体表示从数据库中查询得到的一条记录, 用途如下：

- 映射数据库表中一行完整的数据；
- 通常字段与数据库表中的字段一一对应；
- 必须包含表中所有查询字段，类型也要完全匹配
- 查询前需要指定表名，使用 #[diesel(table_name = ...)]。

**常见扩展：**

- `#[derive(Selectable)]` 允许你使用 `.select()` 查询字段并直接构造成该结构体；
- `#[derive(Serialize)]` 用于将结构体转为 JSON，常见于 REST API。

```rust
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {}
```

## Insertable 模型

这个结构体用于插入新数据行到数据库中, 用途如下：

- 通常不包含自增的主键字段（如 id）；
- 仅包含你需要写入的字段；
- 可以是完整字段，也可以是部分字段（只要满足数据库约束）；
- 插入前需要指定表名，使用 #[diesel(table_name = ...)]。

```rust
#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {}
```

## AsChangeset 模型

这个结构体用于更新已有的数据库记录, 用途如下：

- 字段可以是 `Option<T>`，代表可选更新；
- 如果某字段为 `None`，则更新时会被忽略；
- 适用于更新部分字段的场景；
- 更新前需要指定表名，使用 `#[diesel(table_name = ...)]`。

为什么你没有加 `AsChangeset` 也能更新数据 :question:

✅ **那是因为有两种更新方式：**

:one: 使用结构体（需要 `#[derive(AsChangeset)]`）

```rust
#[derive(AsChangeset)]
#[diesel(table_name = posts)]
struct UpdatePost {
    title: String,
    body: String,
}

fn update() {
    diesel::update(posts.filter(id.eq(1)))
    .set(&update_post)
    .execute(&mut conn)?;
}
```

:two: 使用字段表达式（不需要结构体和宏）

```rust
fn update() {
    diesel::update(posts.filter(id.eq(1)))
    .set(title.eq("新的标题"))
    .execute(&mut conn)?;
}
```

这种方式是 `DSL` 风格，直接使用 `.set(field.eq(value))`，不需要定义结构体或宏。

## Identifiable 模块（可选）

这个宏用于表示某个结构体拥有主键，可以被唯一识别, 用途如下：

- 用于和 `belongs_to` / `has_many` 建立关系；
- 如果结构体的主键字段不是 id，需要配合 `#[primary_key(...)]` 指定；
- 多用于关联关系的场景。

```text
#[derive(Identifiable)]
#[diesel(primary_key(user_id))]
```

## 常用宏一览

在 Diesel ORM 中，定义模型（models）时常用的宏主要包括以下这些，每个宏都有其专属用途，帮助我们更方便地和数据库进行交互。其中涉及到关联关系（高级特性）不过多展开，后续学习关联关系了解更多宏。

| 宏名                                    | 用途说明                                                                          |
|---------------------------------------|-------------------------------------------------------------------------------|
| `#[derive(Queryable)]`                | 表示该结构体可用于从数据库中查询数据（select）。                                                   |
| `#[derive(Insertable)]`               | 表示该结构体可用于向数据库中插入数据（insert）。                                                   |
| `#[derive(AsChangeset)]`              | 表示该结构体可用于更新数据库中的记录（update）。                                                   |
| `#[derive(Identifiable)]`             | 为该结构体生成主键信息，通常用于 `belongs_to` 关联等。                                            |
| `#[derive(Selectable)]`               | 搭配 `.select()` 使用，用于从表中选择部分字段映射到结构体。                                          |
| `#[diesel(table_name = your_table)]`  | 指定该结构体对应的数据库表，用于 `Insertable`、`AsChangeset`、`Selectable` 等宏中。                 |
| `#[diesel(primary_key(id))]`          | 显式指定主键字段，默认是 `id`，若你的主键不是 `id` 或是复合主键，需要加上此项。                                 |
| `#[diesel(belongs_to(ParentStruct))]` | 用于定义一对多/多对一的表间关系，配合 `joinable!` 和 `allow_tables_to_appear_in_same_query!` 使用。 |
