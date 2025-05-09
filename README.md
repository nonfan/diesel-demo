<img src="./logo.svg"/>

## 写测试不是浪费时间，而是节省你未来无数排查 bug 的时间，提高开发效率和代码质量。

### Diesel Test 指南

**项目结构预览**
```bash
diesel_demo/
├── migrations/                 # Diesel 数据库迁移文件夹（由 diesel CLI 创建）
│   └── ...                     # 每次执行 `diesel migration generate` 会在这里生成 up/down.sql
│
├── src/
│   ├── lib.rs                  # lib.rs 暴露的 crate 模块
│   ├── main.rs                 # 项目入口，配置 Actix Web 路由 & 数据库连接池
│   ├── models.rs               # 数据模型，定义表结构体（如 User），用于查询、插入、更新
│   ├── schema.rs               # Diesel 自动生成的数据库表映射宏，不能手动修改
│   ├── handlers.rs             # 用于处理路由
├── tests/
├── .env                        # 环境变量文件，通常放置 DATABASE_URL，例如：sqlite://database.db
├── .gitignore                  # 忽略不需要提交到 Git 的文件，例如 target/、.env、database.db 等
├── database.db                 # SQLite 数据文件（由 Diesel 自动创建）
├── Cargo.toml                  # Rust 项目的依赖管理文件，记录依赖包、版本、构建信息等
├── README.md                   # 项目说明文档，介绍项目用途、如何运行、如何使用等
```


### 测试前提

:one: 项目是否为 library 类型（是否有 lib.rs）？

Rust 的 `tests/` 目录只能测试 `lib.rs` 暴露的 crate 模块，不能直接访问 `main.rs`。如下示例：

**lib.rs**

```rust
pub mod handlers;
```

:two: Cargo.toml 中 `[package] name = "crate_name"` 正确设置了吗？

检查 Cargo.toml 是否如下：

```toml
[package]
name = "rust_demo"
version = "0.1.0"
edition = "2024"
```

### 遇到的问题 :question:

:one: 无法对连接池进行提炼

`actix_http::Request` 类型无法引入，是 actix_web 的底层 http 模块，尚没有找到解决办法。

```rust
type TestApp = impl Service<
    actix_http::Request,
    Response = actix_web::dev::Response,
    Error = actix_web::Error,
>;
async fn init_app() -> impl actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    Response = actix_web::dev::Response,
    Error = actix_web::Error,
> {

    let pool = setup_test_db();
    test::init_service(
        App::new()
        .app_data(web::Data::new(pool))
        .service(list_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user),
    )
    .await
}
```

### 验证测试

:one: 运行测试

```bash
cargo test
```

:two: 检查覆盖率, 使用 `cargo tarpaulin` 检查：

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

:there: 调试错误： 如果测试失败，运行

```bash
cargo build --tests
cargo test -- --show-output
```