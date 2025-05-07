<img src="./logo.svg"/>

## 柴油是一种安全、 可扩展的 ORM 和查询生成器

### Diesel CRUD 指南

CRUD 代表“创建、读取、更新、删除”, Diesel 为所有 4 个部分提供支持。

**项目结构预览**
```bash
diesel_demo/
├── migrations/                 # Diesel 数据库迁移文件夹（由 diesel CLI 创建）
│   └── ...                     # 每次执行 `diesel migration generate` 会在这里生成 up/down.sql
│
├── src/
│   ├── main.rs                 # 项目入口，配置 Actix Web 路由 & 数据库连接池
│   ├── models.rs               # 数据模型，定义表结构体（如 User），用于查询、插入、更新
│   ├── schema.rs               # Diesel 自动生成的数据库表映射宏，不能手动修改
├── .env                        # 环境变量文件，通常放置 DATABASE_URL，例如：sqlite://database.db
├── .gitignore                  # 忽略不需要提交到 Git 的文件，例如 target/、.env、database.db 等
├── Cargo.toml                  # Rust 项目的依赖管理文件，记录依赖包、版本、构建信息等
├── README.md                   # 项目说明文档，介绍项目用途、如何运行、如何使用等
```

### 初始化新项目

我们需要做的第一件事是生成我们的项目。

```bash
cargo new demo
cd demo
```

首先，让我们将 `Diesel` 和 `actix-web` 添加到我们的依赖项中。我们还将使用一个名为 `.env` 的工具来管理我们的环境变量。我们也会将其添加到我们的依赖项中。也包含一些日志记录依赖。

**Cargo.toml**
```toml
[dependencies]
actix-web = { version = "4.10.2" }
diesel = { version = "2.2", features = ["postgres", "r2d2",] }
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
env_logger = "0.11"
serde_json = "1.0"
```

#### features 解释

**r2d2**

在 Diesel ORM 中，r2d2 是一个用于管理数据库连接池的库，而 r2d2 特性（feature） 的作用是让 Diesel 集成 r2d2 连接池功能。

默认情况下，Diesel 的数据库连接（如 PgConnection、SqliteConnection）是单次使用的。启用 r2d2 特性后，Diesel 会提供 r2d2 连接池支持，允许你在多线程环境中高效复用数据库连接。

### 安装 Diesel CLI

```bash
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

# Windows (powershell)
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex
```

或者，您可以使用 `cargo install`：

```shell
cargo install diesel_cli
```
### Docker PostgreSQL 容器

使用 Docker 创建一个 PostgreSQL 容器并运行: 

```bash
docker run --name 容器名称 -e POSTGRES_PASSWORD=123456 -e POSTGRES_DB=databse -p 5432:5432 -d postgres
```

**`-e POSTGRES_DB` 可选**，默认创建数据库 `postgres`，用户名 `postgres`。



安装 Postgres 客户端：

```bash
brew install postgresql

# 卸载命令
brew uninstall postgresql
```


### 为您的项目设置 Diesel

我们可以将 `DATABASE_URL` 写入 `.env` 文件，避免污染全局环境，方便本地多个项目使用各自的数据库配置。


```bash 
echo DATABASE_URL=postgres://user:password@127.0.0.1:5432/database > .env
```

现在通过 Diesel CLI 初始化我们项目的基本内容：

```bash
diesel setup
```

这将创建我们的数据库（如果尚不存在）并设置初始迁移目录，该目录将包含生成的迁移文件，用于建立 Diesel 设置。请注意，`migrations`
目录不会为空，因为初始设置迁移是自动生成的。

如果我们编写一个管理用户的应用，我们首先需要创建迁移文件：

```shell
diesel migration generate create_users
```

迁移允许我们随着时间的推移发展数据库架构。每个迁移都包含一个用于应用更改的 `up.sql` 文件和一个用于还原更改的 `down.sql`
文件。应用并立即还原迁移应保持数据库架构不变。

```shell
Creating migrations/2025-05-06-062724_create_users/up.sql
Creating migrations/2025-05-06-062724_create_users/down.sql
```

接下来，我们将编写用于迁移的 SQL：

**up.sql**
```up.sql
CREATE TABLE users (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   username VARCHAR NOT NULL,
   remark TEXT NOT NULL
)
```
**down.sql**
```
DROP TABLE users
```

编写SQL文件完成后，我们运行迁移：

```shell
diesel migration run
```

运行迁移后会创建包含以下内容的 `schema.rs` 文件：

```rust
diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        remark -> Text,
    }
}
```

创建 `models.rs` 文件，用于定义数据模型结构体，这些结构体代表数据库中的表结构。

```rust
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Selectable,     Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub remark: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub remark: String,
}
```

### PostgreSQL 数据库连接

```rust
// 定义一个用于异步共享的数据库连接池类型
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 获取数据库文件地址
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // 初始化数据库连接池
    let pool = r2d2::Pool::builder()
    .max_size(15)
    .build(manager)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to build pool: {}", e)))?;

    HttpServer::new(move || {
        App::new()
        // 克隆连接池，以共享连接池
        .app_data(web::Data::new(pool.clone()))
        .service(list_users)
        .service(get_user)
        .service(create_posts)
        .service(update_user)
        .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

在 PostgreSQL 中，RETURNING 子句用于在执行 INSERT、UPDATE 或 DELETE 等 DML（数据操作语言）语句时，返回受影响的行或指定的列值。它非常有用，可以减少额外的查询，提高效率，尤其在需要获取操作结果（如新插入的 ID）时。

并非所有数据库都支持 RETURNING 子句。在支持 RETURNING 子句的后端（例如 PostgreSQL 和 SQLite）上，我们也可以从 insert 中取回数据。在 SQLite 后端，从 3.35.0 版本开始支持 RETURNING。 要启用 RETURNING 子句，请添加功能标志， returning_clauses_for_sqlite_3_35 在 Cargo.toml 中。

### CRUD 示例

#### 创建用户

```rust
#[post("/users")]
async fn create_posts(pool: web::Data<DbPool>, body: web::Json<NewUser>) -> Result<impl Responder> {
    use crate::schema::users::dsl::*;

    let new_user = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}
```

#### 获取用户

```rust
#[get("/users")]
async fn list_users(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let results = web::block(move || users.load::<User>(&mut conn))
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(results))
}
```

```rust

#[get("/users/{id}")]
async fn get_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let user_id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || users.filter(id.eq(user_id)).first::<User>(&mut conn))
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}
```

#### 更新用户

```rust
#[put("/users/{id}")]
async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<NewUser>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let new_user = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::update(users)
        .filter(id.eq(user_id))
        .set((username.eq(new_user.username), remark.eq(new_user.remark)))
        .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}
```

#### 删除用户

```rust
#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;

    let deleted_user = web::block(move || {
        diesel::delete(users.filter(id.eq(user_id)))
        .returning(User::as_returning())
        .get_result(&mut conn) // 返回被删除的行
    })
    .await?;

    match deleted_user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(diesel::NotFound) => Ok(HttpResponse::NotFound().json(json!({ "error": "用户不存在" }))),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}
```
