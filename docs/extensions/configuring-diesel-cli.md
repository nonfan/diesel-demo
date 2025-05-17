# 配置 Diesel CLI

## 什么是 Diesel CLI:question:

Diesel CLI 是 Diesel 提供的一个命令行工具，帮你管理数据库相关的任务，比如：

- 生成迁移文件（`diesel migration generate`）。
- 运行或回滚迁移（`diesel migration run`、`diesel migration revert`）。
- 生成 schema 文件（`diesel print-schema`）

你可以把它理解为 Diesel 的“数据库管家 + 代码生成器”。

**配置功能**指的是通过一个配置文件（通常是 `diesel.toml`），告诉 Diesel CLI 你的数据库连接信息、生成文件的路径，以及一些自动化行为（比如每次迁移后自动更新 schema）。配置好了之后，你就不用每次都手动指定数据库 URL 或重复跑命令，省时又省力！

## 配置的核心

Diesel CLI 的配置主要靠一个文件：`diesel.toml`，放在项目根目录下。这个文件用 TOML 格式，简单易读。我们来看看它长啥样，以及怎么用它来实现 schema 自动生成和管理。

### 创建 diesel.toml

在项目根目录下创建一个 `diesel.toml` 文件，基础结构是这样的：

```toml
[print_schema]
file = "src/schema.rs"

[database]
url = "postgres://postgres:mypassword@localhost:5432/drop_db"
```

- `[print_schema]`：控制 diesel print-schema 命令的行为，比如生成的文件路径。
- `file`：指定生成的 schema.rs 文件路径，这里是 src/schema.rs。
- `[database]`：定义数据库连接信息。
- `url`：你的数据库 URL，和 .env 文件里的 DATABASE_URL 一样。

`[database]` 配置不是一定的，原因是 Diesel CLI 有“多重查找配置”的机制，所以就算你没在 `diesel.toml` 中写 `[database]` 配置，它也能连接上数据库。

Diesel CLI 会首先查找环境变量 DATABASE_URL、其次 `.env` 文件。

多数据库支持：你甚至可以为不同的环境配置多个数据库：

```toml
[database.development]
url = "postgres://postgres:mypassword@localhost:5432/drop_db"

[database.production]
url = "postgres://user:pass@prod_host:5432/prod_db"
```

然后用 `--database` 指定环境：

```bash
diesel migration run --database production
```


### 配置自动生成 Schema

最实用的功能之一是让 Diesel 在每次运行迁移（diesel migration run）后自动更新 `schema.rs`。这需要加一个配置项：

```toml
[print_schema]
file = "src/schema.rs"
auto_run = true
```

`auto_run = true`：告诉 Diesel CLI，每次运行 `diesel migration run` 或 `diesel migration revert` 后，自动执行 `diesel print-schema`，更新 `schema.rs`。

### 更细致的配置选项

`diesel.toml` 支持更多选项，让你精细控制 schema 生成的行为。以下是一些常用的：

:one: **过滤表:**

如果你只想生成某些表的 schema（比如忽略测试表），可以用 `filter`：

```toml
[print_schema]
file = "src/schema.rs"
auto_run = true
filter = { only_tables = ["users", "posts"] }
```

`only_tables`：只生成 users 和 posts 表的 schema，其他表会被忽略。 还有其他选项，比如 `except_tables`（排除某些表）。