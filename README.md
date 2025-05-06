# Diesel CRUD 指南

CRUD 代表“创建、读取、更新、删除”, Diesel 为所有 4 个部分提供支持。

## 初始化新项目

我们需要做的第一件事是生成我们的项目。

```bash
cargo new demo
cd demo
```

## 安装 Diesel CLI

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

## 为您的项目设置 Diesel

我们可以将 `DATABASE_URL` 写入 `.env` 文件，避免污染全局环境，方便本地多个项目使用各自的数据库配置。

```bash 
echo DATABASE_URL=./database.db > .env
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

```up.sql
CREATE TABLE users (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   username VARCHAR NOT NULL,
   remark TEXT NOT NULL
)
```

```down.sql
DROP TABLE users
```

编写SQL文件完成后，我们运行迁移：

```shell
diesel migration run
```

运行迁移后会创建包含以下内容的 `schema.rs` 文件：

```rs
diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        remark -> Text,
    }
}
```

## CRUD

### 创建用户

```rust
#[post("/users")]
async fn create_posts(pool: web::Data<DbPool>, body: web::Json<NewUser>) -> Result<impl Responder> {
    use crate::schema::users::dsl::*;

    let new_user = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}
```

### 获取用户

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

### 更新用户

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

### 删除用户

```rust
#[delete("/users/{id}")]
async fn delete_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let user_id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}
```