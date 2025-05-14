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
cargo new --lib diesel_demo
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

现在，我们将编写一个小的 CLI 来让我们管理一个博客（忽略了我们只能从这个 CLI 访问数据库的事实......).
我们首先需要一个表格来存储我们的帖子。让我们为此创建一个迁移：

### 迁移文件

```bash
diesel migration generate create_posts
```

Diesel CLI 将以所需的结构为我们创建两个空文件。您将看到如下所示的输出：

```text
Creating migrations/20250514133237_create_posts/up.sql
Creating migrations/20250514133237_create_posts/down.sql
```

迁移允许我们随着时间的推移发展数据库架构。每个迁移都包含一个用于应用更改的 `up.sql` 文件和一个用于还原更改的 `down.sql` 文件。应用并立即还原迁移应保持数据库架构不变。




