---
# https://vitepress.dev/reference/default-theme-home-page
layout: home
title: "Diesel中文文档"
description: Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器

hero:
  name: "Diesel中文文档"
  text: "Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器"
  tagline: ⚠️ 免责声明：本文内容为个人学习记录，不构成专业建议，仅供参考。如有错误欢迎指正。
  actions:
    - theme: brand
      text: Diesel 是什么?
      link: /intro/what-is-diesel
    - theme: alt
      text: 快速开始
      link: /intro/getting-started

features:
  - title: 防止运行时错误
    details: 我们不想浪费时间跟踪运行时错误。我们通过让 Diesel 消除编译时数据库交互错误的可能性来实现这一点。
  - title: 为性能而生
    details: Diesel 提供了一个高级查询构建器，让您在 Rust 而不是 SQL 中思考问题。我们专注于零成本抽象，使 Diesel 能够运行查询并加载数据，速度甚至比 C 更快。
  - title: 高效且可扩展
    details: 与 Active Record 和其他 ORM 不同，Diesel 被设计为抽象。Diesel 使您能够编写可重用的代码，并根据您的问题域而不是 SQL 进行思考。
---

