# 深入了解 Schema

大家好！今天我们来聊聊 Diesel 中的 Schema 是什么，特别是 `diesel print-schema` 和 `table!`
宏这两个核心部分。它们到底做了什么？生成的代码又是啥意思？如果你曾经对 `schema::posts::dsl::*`
这种东西感到困惑，那就跟着我一步步解开谜团吧！我们还会用一个简单的例子，展示 `table!` 生成的代码是什么样子的，帮你搞清楚每部分的作用。

## 数据库的“侦探”

`diesel print-schema` 是 Diesel CLI（命令行工具）提供的一个命令。它的作用简单又强大，可以看成是一个“数据库侦探”，帮你把数据库的结构翻译成
Rust 能懂的代码。

**它是怎么工作的:question:**

:one: **连接数据库**：首先，它会连上你指定的数据库（比如 PostgreSQL、MySQL 等）。

:two: **查表和列**：然后，它会去数据库里“侦查”，找出所有表的名字，以及每个表有哪些列、每列的类型是什么。

:three: **生成代码**：最后，它会把这些信息变成 Rust 代码，具体来说就是生成一堆 `table!` 宏的调用代码，保存到一个文件里（通常叫
`schema.rs`）。

示例：数据库的 books 表是如何在 Rust 代码体现

:::code-group

```sql [数据库创建的表]
CREATE TABLE books
(
    id    SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
);
```

```rust [运行 diesel print-schema 后，它可能会生成这样的代码：]
diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
    }
}
```

:::

> [!TIP] 细节
> **跳过特殊表**: 如果表名以 `__`（双下划线）开头，比如 `__diesel_migrations`（Diesel 用来记录迁移历史的表），它会自动忽略，不生成代码。
> **自动更新**：你可以在 Diesel 的配置里设置，让它每次运行数据库迁移（migrations）时自动重新跑 diesel
> print-schema，确保代码跟数据库保持一致。想知道怎么设置？可以看看 Diesel CLI 的配置文档。

## table! 宏

`table!` 宏是 Diesel 的核心魔法。它会根据你提供的表结构（比如上面的 `books` 表），生成一大堆 Rust 代码。这些代码让你可以用
Rust 安全、方便地跟数据库交互。

**它生成了什么:question:**

生成的代码其实挺复杂的，但我们可以用一个简化的版本来理解它的核心内容。还是以这个 books 表为例：

```rust
diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
    }
}
```

> [!TIP] 扩展知识
> `diesel::table!` 和直接使用 table! 本质上是一样的宏，有细微差别如：如果你写的是 `schema.rs` 文件，直接使用 `table!`
> 即可、如果你写的是可复用模块或库组件，建议用 `diesel::table!`，更明确、更安全。

运行这个宏后，Diesel 会生成一个叫 books 的 Rust 模块，里面包含了各种有用的东西，我们一步步来看看。

### 1. table 结构体

```rust
pub struct table;
```

这个 table 结构体就代表 books 表本身。当你写 SQL 查询时，会用到它，比如 `books::table.filter(...)`。

### 2. columns 模块

```rust 
pub mod columns {
    pub struct id;
    pub struct name;
}
```

每个列（id、title）都有一个对应的结构体。这些结构体用来在查询中引用列，比如 `books::title.eq("Rust")`。

**类型安全**：每个列还实现了 `Expression` trait，告诉 Diesel 这个列的 SQL 类型是什么：

```rust
impl Expression for id {
    type SqlType = Integer;
}
impl Expression for name {
    type SqlType = Text;
}
impl Expression for hair_color {
    type SqlType = Nullable<Text>;
}
```

这保证了你的查询类型安全，比如你不能把 `title`（文本类型）跟一个数字比较。

**特殊列 star**：

```rust
pub struct star;
impl Expression for star {
    type SqlType = NotSelectable;
}
```

`star` 代表 SQL 中的 `books.*`，也就是“选所有列”。但它主要用于 COUNT 查询（比如统计行数）。

Diesel 不鼓励直接用 `star` 取数据。因为 Diesel 是按列的顺序加载数据的，不是按名字，所以为了确保准确性，它更喜欢你明确列出列名（比如
SELECT books.id, books.title）。

### 3. dsl 模块

dsl 模块是 Diesel 提供的一个“快捷方式”。它把列和表重新导出，让你的查询代码更简洁。

```rust
pub mod dsl {
    pub use super::columns::{id, title};
    pub use super::table as books;
}
```

PS: 学到这里您理解了为什么在代码总可以写成 `books.insert_into` 或者 `books::table.inset_into` :question:

**也就是为什么**：只在单个函数里导入 `use schema::users::dsl::*;`，不要在模块顶部导入。因为有些宏（比如
`#[derive(Insertable)]`）会假设 users 是模块名，而不是表结构体。

**dsl 模块陷阱**

```rust
#[derive(Insertable)]
#[table_name = books] // 错误！dsl 导入下 books 是 table，不是模块
```


### 4. 其他实用工具

:one: `all_columns` 常量：

一个包含所有列的元组。如果你在查询中没指定 SELECT 的列，Diesel 默认用这个来生成 SELECT books.id, books.title, books.xxx。

```rust
pub const all_columns: (id, title) = (id, title);
```

比 `star` 好：如果你想选所有列，用 `all_columns` 比 `star` 更安全、更明确。

:two: `SqlType` 类型：

表示所有列的 SQL 类型组合。很少直接用，但它能让类型检查更严格。

```rust
pub type SqlType = (Integer, Text, Nullable<Text>);
```

:three: `BoxedQuery` 类型：

一个辅助类型，简化 boxed 查询的定义。比如 `books::BoxedQuery<'static, Pg>` 比写全 `diesel::dsl::IntoBoxed` 方便多了。

```rust
pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
```

### 5. 完整简化代码

为了方便你自己翻看，下面是 `users` 表的完整简化代码：

```rust
pub mod users {
    pub use self::columns::*;

    pub mod dsl {
        pub use super::columns::{id, name};
        pub use super::table as books;
    }

    pub const all_columns: (id, title) = (id, title);

    pub struct table;

    impl table {
        pub fn star(&self) -> star {
            star
        }
    }

    pub type SqlType = (Integer, Text, Nullable<Text>);

    pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;

    pub mod columns {
        pub struct star;
        impl Expression for star {
            type SqlType = NotSelectable;
        }

        pub struct id;
        impl Expression for id {
            type SqlType = Integer;
        }

        pub struct title;
        impl Expression for title {
            type SqlType = Text;
        }
    }
}
```

它描述了数据库中 books 表的结构，包括表的列名和数据类型等信息。通过这个模块，我们可以访问 books 表的相关定义。

