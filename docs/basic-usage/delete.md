---
title: Diesel 删除数据
description: 学习如何在 Diesel 中使用 diesel::delete() 执行删除操作，涵盖单条、批量删除及事务场景。
keywords: Diesel, Rust, ORM, 删除数据, diesel::delete
head:
  - - meta
    - property: og:title
      content: Diesel 删除数据教程
  - - meta
    - property: og:description
      content: 掌握 Diesel 中删除操作的多种方法，包括单条记录、批量删除。
---

# 删除数据

在 Diesel 中，删除操作（DELETE）与更新操作类似，都是通过 `diesel::delete()` 来执行的。下面将详细介绍如何在 Diesel 中执行删除操作，涵盖不同的场景和方法。


## 基本的删除操作

Diesel 的删除操作由 `diesel::delete` 函数触发。基本用法如下：

<<< @/../examples/ch06_usage_delete/src/bin/delete.rs

上面示例会删除表内所有数据，这很危险！通常也不会选择清空表，我们可以对删除添加一些条件删除：

<<< @/../examples/ch06_usage_delete/src/bin/filter_delete.rs

如果你已经通过查询获取到某条记录（例如，获取了一个 Post 结构体实例），你也可以直接传递记录的引用进行删除，Diesel 会自动根据该记录的主键生成删除条件。

首先定义结构体，并确保其实现了 `Identifiable`：

```text
#[derive(Queryable, Identifiable)]
```

然后在业务逻辑中删除该记录：

<<< @/../examples/ch06_usage_delete/src/bin/struct_delete.rs

该方法依赖于 `Identifiable` trait，通过结构体中的主键自动构造删除条件，其底层原理等同于 `delete(posts.filter(id.eq(post.id)))`。

> [!TIP] 导航
> [前往 GitHub 查看完整示例代码](https://github.com/nonfan/diesel-demo/tree/docs/examples/ch06_usage_delete)