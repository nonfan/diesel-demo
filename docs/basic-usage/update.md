---
title: Diesel 更新数据
description: 在使用 Rust 构建后端服务时，Diesel 是一个安全、强类型的 ORM 工具，它能让我们通过 Rust 的语法安全地操作数据库。本文将系统介绍 Diesel 中数据更新（UPDATE）操作的使用方法，涵盖单字段、多字段、结构体更新、批量更新等内容，并对不同数据库后端的兼容性进行说明。
---

# 更新数据

本文将系统介绍 Diesel 中数据更新（**UPDATE**）操作的使用方法，涵盖单字段、多字段、结构体更新、批量更新等内容，并对不同数据库后端的兼容性进行说明。

## 基本的更新操作

Diesel 使用 `update` 函数配合 `.set()` 来执行更新操作，整体风格与 SQL 非常接近：

更新语句通过调用 `diesel::update(target).set(changes)` 构造。然后通过调用 `execute`、`get_result` 或 `get_results`
来运行生成的语句。

我们可以通过执行以下作来编写一个发布所有帖子的查询：

<<< @/../examples/ch05_usage_update/src/bin/update.rs

想要更新整个表的情况非常罕见。因此，让我们看看如何缩小范围。您可以传递给 `update` 的第二种类型是仅调用了 `.filter`
的任何查询。我们可以将更新范围限定为刚刚发布的标题，如下所示：

<<< @/../examples/ch05_usage_update/src/bin/filter_update.rs

如果我们想只发布这篇文章，我们可以这样做：

```rust
use diesel::prelude::*;
use crate::schema::posts::dsl::*;

fn main() {
    // 查询一篇指定 id 的文章
    let post = posts
    .filter(id.eq(1))
    .first::<Post>(&mut conn)?;

    // 然后使用它进行更新
    diesel::update(&post)
    .set(draft.eq(false))
    .execute(&mut conn)?;
}
```

当你传入一个具有主键的结构体引用给 `diesel::update(&post)`，Diesel 会自动用它的主键字段（比如 id）来查找并更新对应的数据库记录。

请务必注意，我们始终传递对帖子的引用，而不是帖子本身。当我们编写 `update(post)` 时，这相当于编写 `update(posts.find(post.id))`， 或者 `update(posts.filter(id.eq(post.id)))`。

## 更新多个字段

在实际项目中，我们经常需要一次性更新多个字段。Diesel 提供了两种方式来实现这个目标。

### :one: 使用元组（Tuple）

可以直接将多个字段更新组合为一个元组传入 `.set()`：

<<< @/../examples/ch05_usage_update/src/bin/tuple_filter_update.rs

### :two: 使用结构体更新

如果你希望复用更新逻辑或者更具表达性，可以使用 `#[derive(AsChangeset)]` 的结构体：

````rust
#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct UpdateUser {
    pub title: String,
    pub body: String,
}
````

更新查询示例：

<<< @/../examples/ch05_usage_update/src/bin/struct_update.rs

## 批量更新

批量更新与单条更新语法一致，只需要改变 `.filter(...)` 的范围：

```rust
fn main() {
    diesel::update(posts.filter(published.eq(false)))
    .set(published.eq(true))
    .execute(&mut conn)?;
}
```

> [!TIP] 导航
> [前往 GitHub 查看完整示例代码](https://github.com/nonfan/diesel-demo/tree/docs/examples/ch05_usage_update)