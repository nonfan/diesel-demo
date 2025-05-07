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
│   ├── lib.rs                  # 项目入口，配置 Actix Web 路由 & 数据库连接池
│   ├── main.rs                 # 项目入口，配置 Actix Web 路由 & 数据库连接池
│   ├── models.rs               # 数据模型，定义表结构体（如 User），用于查询、插入、更新
│   ├── schema.rs               # Diesel 自动生成的数据库表映射宏，不能手动修改
├── tests/
├── .env                        # 环境变量文件，通常放置 DATABASE_URL，例如：sqlite://database.db
├── .gitignore                  # 忽略不需要提交到 Git 的文件，例如 target/、.env、database.db 等
├── database.db                 # SQLite 数据文件（由 Diesel 自动创建）
├── Cargo.toml                  # Rust 项目的依赖管理文件，记录依赖包、版本、构建信息等
├── README.md                   # 项目说明文档，介绍项目用途、如何运行、如何使用等
```
