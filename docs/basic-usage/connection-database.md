# 连接池

在 Diesel + Web 框架（如 Actix Web、Axum）的项目中，**连接池**是处理数据库连接的重要方式。它通过预先维护一组连接，让请求可以复用连接，从而提升性能并保证线程安全。

## 什么是连接池:question:

连接池（Connection Pool）是一种复用数据库连接的机制。相比每次请求都新建连接、关闭连接，连接池通过预先创建并维护多个连接，让每个请求可以快速获取并使用连接，避免重复创建开销。

### 连接池的优势

* **性能更高**：避免频繁建立和销毁连接。
* **线程安全**：连接池可在线程间共享 Diesel 的连接。
* **异步兼容**：虽然 Diesel 是同步库，但连接池可以作为异步应用中的共享资源传递。
* **资源管理**：自动管理连接数量和生命周期，防止连接泄露或资源耗尽。

### Diesel 推荐的连接池：r2d2

Diesel 官方推荐使用 [`r2d2`](https://crates.io/crates/r2d2) 作为连接池库。它是 Rust 生态中广泛使用的数据库连接池解决方案。

你可以使用如下类型别名简化使用：

```rust
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
```

### r2d2 的核心能力

* **连接复用**：避免重复创建连接，降低数据库负载。
* **资源控制**：限制最大连接数，防止数据库过载。
* **错误恢复**：自动检测连接状态并回收失效连接。

### 为什么需要连接池？

在没有连接池的情况下，每次数据库操作（如查询或更新）都需要：

1. 建立新的数据库连接（涉及网络握手、认证等）；
2. 执行数据库操作；
3. 关闭连接。

这种方式**效率低**，在 Web 应用中每个请求都新建连接，会导致**大量性能开销**。连接池通过重用已有连接，大幅提升并发性能，是 Web
应用中不可或缺的基础设施。

## r2d2 在 Diesel 中的集成

Diesel 是一个类型安全的 ORM 库，本身不直接管理连接池，但通过 `r2d2` 特性提供了无缝集成。启用 `r2d2` 后，Diesel
可以使用连接池来管理数据库连接，特别适合 Web 后端开发。

在 `Cargo.toml` 中 Diesel 依赖中添加特性 `r2d2`：

```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
dotenv = "0.15"
```

- `features = ["postgres", "r2d2"]`：启用 `PostgreSQL` 后端和 `r2d2` 连接池支持。如果使用 MySQL 或 SQLite，可以将
  `postgres` 替换为 `mysql` 或 `sqlite`。
- `dotenv`：用于加载环境变量（如 DATABASE_URL）。

### r2d2 的核心组件

`r2d2` 的工作原理基于以下几个核心组件：

:one:  `ConnectionManager`:

- 定义如何创建和验证数据库连接。
- Diesel 提供了 `diesel::r2d2::ConnectionManager<T>`，其中 T 是具体的连接类型（如 `PgConnection`、`MysqlConnection` 或
  `SqliteConnection`）。

:two: `Pool`：

- `r2d2::Pool` 是连接池的核心，管理一组连接。
- 通过 `Pool::get()` 获取可用连接，操作完成后自动归还连接。

:three: `PooledConnection`：

- 从池中获取的单个连接，包装了底层的数据库连接（如 `PgConnection`）。
- 支持 Diesel 的所有查询操作。

:four: `PoolBuilder`：

- 用于配置连接池的参数，例如最大连接数、空闲连接数、连接超时等。

## 配置和使用 r2d2

以下是如何在 Rust 项目中使用 r2d2 连接池的详细步骤，结合 Diesel 和 Web 后端场景。

### 添加数据库环境

```bash
echo DATABASE_URL=postgres://user:password@127.0.0.1:5432/database > .env
```

### 创建连接池函数

<<< @/../examples/ch02_r2d2/src/pool.rs

> [!TIP] 为什么在 create_db_pool 中使用 Result？
> `create_db_pool` 负责初始化数据库连接池，可能因以下原因失败：DATABASE_URL 环境变量未设置、数据库不可用或配置错误或连接池资源不足。使用
> Result 可以避免以下问题：防止崩溃、提供错误恢复等友好行为。

### 常用配置参数

`r2d2::Pool::builder()` 支持以下配置项，优化连接池行为：

| 参数                   | 说明                  | 默认值    | 建议值（Web 后端）   |
|----------------------|---------------------|--------|---------------|
| `max_size`           | 池中最大连接数             | 10     | 10-20（视数据库容量） |
| `min_idle`           | 最小空闲连接数（可为空闲时保持连接）  | `None` | 2-5           |
| `connection_timeout` | 获取连接的超时时间           | 30秒    | 5-10秒         |
| `max_lifetime`       | 连接的最大存活时间（避免长期连接失效） | 30分钟   | 30分钟          |
| `idle_timeout`       | 空闲连接的超时时间（回收空闲连接）   | 10分钟   | 5-10分钟        |
| `test_on_check_out`  | 借出连接时测试其有效性         | `true` | `true`（生产环境）  |

:::code-group

```rust [示例配置]
fn pool() {
    r2d2::Pool::builder()
    .max_size(15)
    .min_idle(Some(3))
    .connection_timeout(std::time::Duration::from_secs(5))
    .max_lifetime(Some(std::time::Duration::from_secs(1800)))
    .build(manager)
    .expect("Failed to create pool")
}
```

:::

## 在 Web 后端中的使用

在 Web 应用中，`r2d2` 连接池通常与 Web 框架（如 Actix-web）结合，共享给多个请求处理程序。如下几个步骤在 Actix-web
框架中使用r2d2:

:one: 在 `Cargo.toml` 添加  `actix-web` 依赖:

```toml
[dependencies]
actix-web = { version = "4.10.2" }
diesel = { version = "2.2.10", features = ["postgres", "r2d2"] }
dotenvy = "0.15.7"
```

:two: 在 `mian.rs` 文件编写服务器：

```rust
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

:three: 在 `main.rs` 创建并共享连接池：

在项目入口 `main.rs` 中初始化连接池，并通过 Web 框架（如 Actix Web）的 `App::app_data()`
方法将其作为共享状态传入各个处理函数。这样可以确保整个服务生命周期内复用同一个连接池，提高性能并保证线程安全。

```rust
use actix_web::{web, App, HttpServer};
use ch02_r2d2::create_db_pool; // [!code focus]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool()?; // [!code focus]

    HttpServer::new(move || {
        App::new()
        // 克隆连接池，以共享连接池
        .app_data(web::Data::new(pool.clone())) // [!code focus]
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
> [!TIP] 前往 GitHub 查看完整示例代码
> [本教程代码直达➡️](https://github.com/nonfan/rust-demo/tree/docs/examples/ch02_r2d2)
