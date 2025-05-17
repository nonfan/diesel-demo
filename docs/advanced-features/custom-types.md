# 自定义类型

在使用 Diesel 操作数据库时，字段类型通常使用内置的 Rust 类型（如 `String`, `i32`, `bool`
等）。但在一些业务中，我们可能希望对字段进行更强的封装和类型安全 —— 这时候就需要用到 **自定义类型（Custom Types）**。

## 自定义类型的概念

Diesel 的自定义类型是将数据库中的特定数据类型映射到 Rust 类型的机制。默认情况下，Diesel 支持标准类型（如
Integer、Varchar、Timestamp），但对于非标准类型（如 PostgreSQL 的 JSONB 或自定义格式的字符串），需要开发者手动定义映射。

### 为什么需要自定义类型:question:

- **类型安全**：确保数据库操作与 Rust 类型的严格匹配，减少运行时错误。
- **数据验证**：在 Rust 层验证数据格式（如邮箱地址），提高数据质量。
- **复杂类型支持**：处理数据库特有的类型（如 JSONB、数组、枚举）。
- **代码可读性**：使用语义化的 Rust 类型（如 Email 而非 String），提高代码表达力。


## 实现自定义类型的基本步骤

要创建一个 Diesel 自定义类型，需要实现以下核心 trait：

- `ToSql`：将 Rust 类型序列化为数据库类型。
- `FromSql`：将数据库类型反序列化为 Rust 类型。
- `AsExpression`：允许类型在 Diesel 的查询 DSL 中作为表达式使用。
- 可选 trait：QueryId（优化查询性能）、Expression（扩展查询功能）。