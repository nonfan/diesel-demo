# 原生 SQL

在 Diesel 中，`sql_query` 是一个允许你直接**编写原生 SQL（Raw SQL）语句**的 API，用于处理那些 Diesel 的
DSL（领域特定语言）不易表达的复杂查询或数据库特性。

## 基本语法

```rust
use diesel::sql_query;
use diesel::prelude::*;

fn main() {
    let results = sql_query("SELECT * FROM posts WHERE published = true")
    .load::<Post>(&mut conn)?;
}
```