# 快速开始

在本指南中，我们将通过一些简单示例来演示 CRUD 的每个部分，CRUD
代表“创建（Create）、读取（Read）、更新（Update）、删除（Delete）”。本指南的每个步骤都会在前一步的基础上进行构建，建议你按顺序跟随操作。

在我们开始之前，请确保您已安装并运行 `PostgreSQL`、`SQLite` 或 `MySQL` 之一。在项目存储库中，您可能会找到各种示例
对于每个受支持的数据库。


> [!TIP] 关于 Rust 版本的注释
> Diesel 需要 Rust 1.78 或更高版本。如果您正在按照本指南操作，请运行 `rustup update stable` 命令，确保您使用的 Rust
> 至少为该版本。。

## 初始化新项目

我们需要做的第一件事是生成我们的项目。

```bash
cargo new --src diesel_demo
cd diesel_demo
```

首先，让我们将 Diesel 添加到我们的依赖项中。我们还将使用一个名为 `.env` 的工具来管理环境变量。我们也将它添加到我们的依赖项中。

::: code-group

```toml [Cargo.toml (PostgreSQL)]
[dependencies]
diesel = { version = "2.2.0", features = ["postgres"] }
dotenvy = "0.15"
```

```toml [Cargo.toml (SQLite)]
[dependencies]
diesel = { version = "2.2.0", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }
dotenvy = "0.15"
```

```toml [Cargo.toml (MySQL)]
[dependencies]
diesel = { version = "2.2.0", features = ["mysql"] }
dotenvy = "0.15"
```

:::

## 安装 Diesel CLI

Diesel 提供了一个单独的 CLI 工具来帮助管理您的项目。由于它是一个独立的二进制文件，不会直接影响您项目的代码，因此我们不会将其添加到
`Cargo.toml` 中，而是直接将其安装在我们的系统上。

Diesel 官方为 diesel cli 提供了预构建的二进制文件。您可以通过以下方式安装命令行工具：

::: code-group

```bash [Linux/MacOS]
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

```bash [Windows]
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex
```

:::

你也可以使用 `cargo-binstall` 或者 `cargo install` 来安装 Diesel CLI，执行以下命令即可快速安装。

::: code-group

```bash [cargo-binstall]
cargo binstall diesel_cli
```

```bash [cargo install]
cargo install diesel_cli
```

```text [区别]
✅ cargo install
工作方式：从源码编译安装。

优点：
- 更灵活，能保证安装的是源码编译后的最新版本。
- 安装过程中可以应用自定义编译参数。

缺点：
- 编译时间较长，尤其是依赖多、项目大的时候。
- 占用更多本地资源（CPU、内存）。

⚡️ cargo-binstall
工作方式：下载预编译的二进制文件进行安装（如果有提供）。

优点：
- 安装速度极快，不需要本地编译。
- 适合想快速使用工具的用户，尤其是 CLI 工具。

缺点：
- 依赖于作者是否提供对应平台的二进制文件。
- 可定制性低，不能轻松更改编译配置。
```

:::

## 为您的项目设置 Diesel

我们需要告诉 Diesel 数据库的地址。我们通过设置 `DATABASE_URL` 环境变量来实现。在我们的开发机器上，我们可能会运行多个项目，并且我们不想污染环境。我们可以将
URL 放在 `.env` 文件中。

::: code-group

```bash [PostgreSQL]
echo DATABASE_URL=postgres://user:password@127.0.0.1:5432/database > .env
```

```bash [SQLite]
# 创建SQLite数据库文件的路径
echo DATABASE_URL=./database.db > .env
```

```bash [MySQL]
echo DATABASE_URL=mysql://user:password@127.0.0.1:3306/database > .env
```

:::

现在 Diesel CLI 可以为我们设置好一切。

```bash
diesel setup
```

这将创建我们的数据库（如果它尚不存在）并设置初始迁移目录，该目录将包含用于建立 Diesel
设置的已生成的迁移文件。请注意，迁移目录不会为空，因为初始设置迁移是自动生成的。

## 博客 Demo

在官方文档中，Diesel 使用了一个基于命令行的博客 CLI 应用作为示例项目。我将基于官方Demo进行改编，加上我的理解。

### 迁移文件

```bash
diesel migration generate create_posts
```

Diesel CLI 将以所需的结构为我们创建两个空文件。您将看到如下所示的输出：

```text
Creating migrations/2025-05-14-111454_create_posts/up.sql
Creating migrations/2025-05-14-111454_create_posts/down.sql
```

迁移允许我们随着时间的推移发展数据库架构。每个迁移都包含一个用于应用更改的 `up.sql` 文件和一个用于还原更改的 `down.sql`
文件。应用并立即还原迁移应保持数据库架构不变。

接下来，我们将编写用于迁移的 SQL：

::: code-group

```postgresql [up.sql (PostgreSQL)]
CREATE TABLE posts
(
    id        SERIAL PRIMARY KEY,
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);
```

```sqlite [up.sql (SQLite)]
CREATE TABLE posts
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0
);
```

```mysql [up.sql (MySQL)]
CREATE TABLE posts
(
    id        INTEGER AUTO_INCREMENT PRIMARY KEY,
    title     VARCHAR(255) NOT NULL,
    body      TEXT         NOT NULL,
    published BOOLEAN      NOT NULL DEFAULT FALSE
);
```

:::

```sql
DROP TABLE posts;
```

> [!WARNING] 有关迁移中原始 SQL 的说明：
> 由于迁移是用原始 SQL 编写的，因此它们可以包含您所使用的数据库系统的特定功能。例如，上面的 `CREATE TABLE` 语句使用了
> PostgreSQL 的 `SERIAL` 类型。如果您想使用 SQLite，则需要改用 `INTEGER` 类型。请务必在代码块选项卡选择您正在使用的后端


我们可以应用新的迁移：

```bash
diesel migration run
```

建议你确保 `down.sql` 的内容是正确的。你可以通过“重做（redo）”这个迁移来快速验证：先执行一次 `down.sql` 回滚，再重新执行
`up.sql` 应用。如果两者都能顺利执行，就说明 `down.sql` 能正确地撤销迁移操作。

```bash
diesel migration redo
```

完成迁移操作后，Diesel 会自动根据迁移文件生成 schema 文件 `/src/scheme.rs` ，包含内容如表结构定义：表名、列名、列类型、主键：

<<< @/../examples/ch01_blog_demo_cli/src/schema.rs

### 编写 Rust 代码

既然博客Demo示例基于 `actix-web` Web框架，因此我们需要添加 `Cargo.toml` 依赖，如下完整内容：

<<< @/../examples/ch01_blog_demo_cli/Cargo.toml

#### 连接数据库

我们需要做的第一件事是建立数据库连接。

```rust
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    // 让我们可以获取环境变量 .env 内容
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

#### 创建结构体

我们还需要创建一个 `Post` 结构体，我们可以在其中读取数据，并让 diesel 生成我们将用于在查询中引用表和列的名称。通常结构体在
`./src/models.rs` 中声明：

:::code-group

```rust [PostgreSQL]
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
```

```rust [SQLite]
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::SQLite))] // [!code focus]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
```

```rust [MySQL]
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))] // [!code focus]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
```

:::

- `#[derive(Queryable)]` 将生成从 SQL 查询加载 Post 结构体所需的所有代码。

- `#[derive(Selectable)]` 将生成代码，根据通过 `#[diesel(table_name = crate::schema::posts)]` 定义的表，基于模型类型构造匹配的
  select 子句。

- `#[diesel(check_for_backend(diesel::pg::Pg))`（或 `sqlite::SQLite` 或 `mysql::MySQL`）添加了**额外的编译时检查**
  ，以验证结构体中的所有字段类型是否与其对应的 SQL 表达式兼容。这部分是~~可选的~~，但它极大地改进了生成的编译器错误消息。

- `#[derive(Serialize)]` 是 Rust 中 serde 库提供的宏，它的作用是让结构体或枚举可以被**序列化**，也就是可以被转换成 `JSON`、`YAML`、`TOML` 等格式的字符串，常用于 Web API 返回数据时自动转换成 `JSON`。这部分是~~可选的~~, 但通常我们要输出该结构体的数据便需要它。

> [!TIP] 关于字段顺序的说明
> 使用 `#[derive(Queryable)]` 假定 Post 结构体中字段的顺序与 posts 表中的列匹配，因此请确保按照 schema.rs 文件中的**顺序定义**它们。
> 将 `#[derive(Selectable)]` 与 `SelectableHelper::as_select` 结合使用可确保字段顺序始终匹配。
> `#[diesel(check_for_backend(diesel::pg::Pg))]` 属性进一步检查所有字段类型是否与查询返回的类型匹配。此属性可以显著改善编译器生成的错误消息。

#### 读取数据

请提前安排一些 posts 表数据，以供查询。让我们编写查询数据的文件 `src/bin/list_posts.rs`：

<<< @/../examples/ch01_blog_demo_cli/src/bin/list_posts.rs

`use schema::posts::dsl::*` 这行导入了一堆别名，这样我们就可以用 `posts` 代替 `posts::table`，用 `published` 代替 `posts::published`。当我们只处理单个表时，这很有用，但这并不总是我们想要的。始终将对 `schema::table::dsl::*` 的导入保留在当前函数内部，以防止污染模块命名空间。

> [!DANGER] 防止污染模块命名空间
> “命名空间污染”指的是引入太多名称（变量、函数、结构等）到作用域中，导致命名冲突或可读性变差。在 Rust 中，如果你把 `use schema::posts::dsl::*` 写在模块顶部，就相当于把 posts、published 等名称“扔进了全局作用域”

我们可以使用 `cargo run --bin list_posts` 运行我们的脚本。查看到我们之前插入的数据。

#### 添加数据

接下来，让我们编写一些代码来创建一个新文章。我们需要一个用于插入新记录的结构体。在 `src/models.rs` 添加 `NewPost` 结构体。

```rust
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
  pub title: String,
  pub body: String,
}
```

- `#[derive(Insertable)]`：允许你使用这个结构体向数据库插入数据（.insert_into()）。


现在让我们创建一个 `./src/bin/create_post.rs` 文件来添加文章。

<<< @/../examples/ch01_blog_demo_cli/src/bin/create_post.rs

我们可以使用 `cargo run --bin create_post` 运行我们的脚本，查看到我们插入的数据。

不幸的是，运行 list_posts 仍然不会显示我们的新帖子，因为我们将其保存为草稿。如果我们回顾一下 list_posts 中的代码，我们添加了 `.filter(published.eq(true))`， 并在迁移中将 default 发布为 `false`。我们需要发布它！但为了做到这一点，我们需要研究如何更新现有记录。

> [!TIP] 事务操作
> 使用 `transaction` 方法开启一个数据库事务。事务确保其中的所有操作要么全部成功，要么全部失败，具有原子性。这里只做简单了解，后续详细学习事务内容。

#### 更新数据

现在我们已经完成了 `create` 和 `read` 操作，`update` 实际上相对简单。让我们直接创建 `./src/bin/update_post` 脚本：

<<< @/../examples/ch01_blog_demo_cli/src/bin/update_post.rs

就是这样！让我们试试 `cargo run --bin publish_post ID`, 将ID替换成您创建帖子的ID。运行结果如下：

```text
Updating post 2
Published post Rust 快速开始
```

现在，我们终于可以看到我们的帖子 `cargo run --bin list_posts`。

```text
展示 2 篇文章
************
Title: Rust入门
Body 关于Rust如何入门到内容
-----------

Title: Rust 快速开始
Body 关于Rust如何快速开始
-----------
```

#### 查询单条数据

另外，让我们实现获取单篇帖子的功能。我们将显示帖子 ID 及其标题。注意 `.optional()` 的调用。它返回 `Option<Post>` 而不是抛出错误，我们可以在匹配模式中使用它。有关修改构造的 `select` 语句的其他方法，[请参阅 QueryDsl 的文档](https://docs.diesel.rs/2.2.x/diesel/query_dsl/trait.QueryDsl.html)。

现在让我们创建一个 `./src/bin/get_post.rs` 文件来查询单篇帖子。

<<< @/../examples/ch01_blog_demo_cli/src/bin/get_post.rs

我们可以看到我们的帖子 `cargo run --bin get_post 1`。

```text
文章ID: 2 
标题: Rust 快速开始
```

#### 删除数据

让我们展示如何删除内容。有时我们写了一些我们非常讨厌的东西，我们没有时间查找 ID。因此，让我们根据标题删除，甚至只是标题中的一些单词。

现在让我们创建一个 `./src/bin/delete_post.rs` 文件来根据标题模糊删除帖子。

<<< @/../examples/ch01_blog_demo_cli/src/bin/delete_post.rs

我们可以使用 `cargo run --bin delete_post Rust` 来删除标题包含 Rust 内容的帖子:

```text
删除 2 篇帖子
```

当我们再次尝试运行 `cargo run --bin list_posts` 时，我们可以看到该帖子确实已被删除。这仅仅触及了 Diesel 功能的冰山一角，但希望本教程能为您提供良好的基础。

> [!TIP] 前往 GitHub 查看完整示例代码
> [本教程博客Demo代码直达➡️](https://github.com/nonfan/rust-demo/tree/docs/examples/ch01_blog_demo_cli)