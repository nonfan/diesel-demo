---
title: Diesel 事务
description: 深入了解 Rust Diesel 的事务机制，学习如何使用 transaction、嵌套事务、错误处理及最佳实践，确保数据库操作的原子性和一致性。
keywords: Rust, Diesel, 事务, ORM, 数据库, 嵌套事务
head:
  - - meta
    - property: og:title
      content: Rust Diesel 事务教程
  - - meta
    - property: og:description
      content: 掌握 Diesel 中的事务用法，包括基本事务、嵌套事务、错误处理和并发优化。
---

# 事务

在 Rust 的 Diesel ORM 框架中，**事务（Transaction）** 是确保数据库操作**原子性、一致性、隔离性和持久性（ACID）**
的核心机制。无论是执行复杂的多表操作，还是确保数据一致性，事务都是不可或缺的工具。本文将详细介绍 Diesel
事务的作用、基本使用方法、嵌套事务、错误处理以及最佳实践，帮助开发者深入掌握 Diesel 的事务机制。

## 事务的作用

事务是一组数据库操作的集合，这些操作要么**全部成功执行**，要么**全部回滚**（撤销），从而保证数据库的一致性。在 Diesel
中，事务的主要作用包括：

* **原子性**：确保一组操作（如插入、更新、删除）作为一个整体执行，中间失败时回滚所有更改。
* **一致性**：防止因部分操作失败导致数据不一致，例如转账时扣款成功但入账失败。
* **隔离性**：在并发场景下，事务隔离操作，防止其他事务干扰当前操作。
* **持久性**：事务提交后，数据永久保存，即使系统崩溃也不会丢失。

**使用场景**：

- 银行转账：从一个账户扣款，同时向另一个账户入账。
- 批量操作：同时更新多张表的数据（如订单和库存）。
- 数据迁移：确保一系列操作要么全部完成，要么不生效。

## 基本事务用法

Diesel 提供了 `transaction` 方法来执行事务操作，通常通过 `Connection` 对象的 `transaction` 方法实现。基本语法如下：

<<< @/../examples/ch07_features_transaction/src/bin/transaction.rs

`conn.transaction::<T, E, F>(closure)` 内容解释：

- `T` 是返回值类型
- `E` 是错误类型（通常是 diesel::result::Error）
- `F` 是闭包类型
    - 如果闭包返回 `Ok(...)`，事务会提交；
    - 如果闭包返回 `Err(...)`，事务会自动回滚

## 事务中的回滚

在事务中你可以主动触发回滚，不需要 `panic`，只需返回 `Err(...)` 即可。

```rust
fn main() {
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {
        let user_id = get_user_id(&conn)?;

        if !is_valid(user_id) {
            // 主动回滚
            return Err(diesel::result::Error::RollbackTransaction);
        }

        do_something(user_id)?;

        Ok(())
    });
}
```

## 事务返回值

你可以在事务中处理多个操作后返回某个值（如插入后的结构体）：

```rust
fn main() {
    let inserted_post: Post = conn.transaction::<Post, diesel::result::Error, _>(|| {
        diesel::insert_into(posts)
        .values((title.eq("Hello"), body.eq("World")))
        .execute(&mut conn)?;

        let post = posts.order(id.desc()).first::<Post>(&mut conn)?;
        Ok(post)
    })?;
}
```

## 嵌套事务

Diesel 支持**嵌套事务**，但需要注意其实现方式。Diesel 使用**保存点 Savepoints**来模拟嵌套事务，而不是真正的独立事务。保存点允许在事务内部设置一个“检查点”，如果子事务失败，可以回滚到保存点而无需回滚整个事务。

<<< @/../examples/ch07_features_transaction/src/bin/nesting_transaction.rs

**嵌套事务的行为：**
- 成功：内外层事务都提交，数据保存。
- 内层失败：回滚到保存点，外层事务可以继续（取决于代码逻辑）。
- 外层失败：整个事务（包括内层）回滚。

> [!WARNING] 注意事项
> 嵌套事务在 SQLite 中可能受限，因为 SQLite 对保存点的支持不如 PostgreSQL 完善。过度使用嵌套事务可能增加代码复杂性，建议仅在必要时使用（如需要部分回滚的场景）。

## 错误处理

事务中的错误处理是 Diesel 开发中的关键部分。Diesel 的事务方法会自动回滚失败的操作，但开发者需要妥善处理错误以提供清晰的反馈。

在事务闭包中，任何返回 `Err` 的操作都会触发回滚。开发者可以通过 `match` 或 `?` 运算符处理错误：

```rust
fn main() {
  let result = connection.transaction::<_, diesel::result::Error, _>(|conn| {
    // 在事务中执行数据库操作
    let new_post = diesel::insert_into(posts)
            .values((title.eq("Rust"), body.eq("Rust 内容")))
            .get_result::<Post>(conn)?; // [!code focus]

    diesel::update(&new_post)
            .set(published.eq(true))
            .execute(conn)?; // [!code focus]

    Ok(())
  });
}
```


### 自定义错误类型

对于复杂应用，建议定义自定义错误类型以提高代码可读性和维护性：

<<< @/../examples/ch07_features_transaction/src/bin/error_transaction.rs

> [!TIP] 导航
> [前往 GitHub 查看完整示例代码](https://github.com/nonfan/diesel-demo/tree/docs/examples/ch07_features_transaction)
