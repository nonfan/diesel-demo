import {defineConfig} from 'vitepress'

export default defineConfig({
  title: "Diesel 中文文档",
  base: "/diesel-demo/",
  description: "Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器",
  head: [['link', {rel: 'icon', href: '/diesel-demo/favicon.png'}], ['meta', {
    name: 'keywords',
    content: 'Diesel中文文档'
  }],
    ['meta', {name: 'author', content: 'MOFAN'}],
    ['link', {rel: 'canonical', href: 'https://nonfan.github.io/diesel-demo/'}]],
  lang: "zh-CN",
  cleanUrls: true,
  sitemap: {
    hostname: 'https://nonfan.github.io/diesel-demo/'
  },
  themeConfig: {
    lastUpdated: {
      text: '最后更新于',
    },
    siteTitle: false,
    logo: {
      light: "/logo.svg",
      dark: "/logo-dark.svg"
    },
    docFooter: {
      prev: '上一篇',
      next: '下一篇'
    },
    darkModeSwitchLabel: '外观',
    returnToTopLabel: '返回顶部',
    sidebarMenuLabel: '菜单',
    nav: [
      {text: '指南', link: '/intro/what-is-diesel'}
    ],
    sidebar: [
      {
        text: '介绍',
        collapsed: false,
        items: [
          {text: 'Diesel 是什么?', link: '/intro/what-is-diesel'},
          {text: '快速开始', link: '/intro/getting-started'},
        ]
      },
      {
        text: '基础用法',
        collapsed: false,
        items: [
          {text: '数据模型', link: '/basic-usage/models'},
          {text: '查询执行', link: '/basic-usage/read'},
          {text: '插入数据', link: '/basic-usage/create'},
          {text: '更新数据', link: '/basic-usage/update'},
          {text: '删除数据', link: '/basic-usage/delete'},
          {text: '原生 SQL', link: '/basic-usage/native_sql'},
        ]
      },
      {
        text: "高级特性",
        collapsed: false,
        items: [
          {text: "事务", link: '/advanced-features/transaction'},
          {text: '连接池', link: '/advanced-features/connection-database'},
          {text: "关联关系", link: '/advanced-features/relations'},
          {text: "自定义类型", link: '/advanced-features/custom-types'},
          {text: "迁移", link: '/advanced-features/migration'},
        ]
      },
      {
        text: "扩展",
        collapsed: false,
        items: [
          {text: "深入了解 Schema", link: "/extensions/schema_in_depth"},
          {text: "扩展柴油", link: "/extensions/extending-diesel"},
          {text: "配置 Diesel CLI", link: "/extensions/configuring-diesel-cli"},
        ]
      }
    ],
    search: {
      provider: 'local'
    },
    socialLinks: [
      {icon: 'github', link: 'https://github.com/nonfan/diesel-demo'}
    ],
    editLink: {
      pattern: 'https://github.com/nonfan/diesel-demo/edit/docs/docs/:path',
      text: "在 GitHub 上编辑此页面"
    },
    footer: {
      message: '基于 MIT 许可发布',
      copyright: 'Copyright © 2025-present <a href="https://github.com/nonfan/diesel-demo">MOFAN</a>'
    },
  }
})
