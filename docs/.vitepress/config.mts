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
    ['meta', {name: 'google-site-verification', content: 'w89dsfqi2nI2Xof32iFHlx0pF9DiEqrDpJEc7ON6ykQ'}],
    ['link', {rel: 'canonical', href: 'https://nonfan.github.io/diesel-demo/'}]],
  lang: "zh-CN",
  cleanUrls: true,
  sitemap: {
    hostname: 'https://nonfan.github.io/diesel-demo/'
  },
  themeConfig: {
    outline: [2, 3],
    lastUpdated: {
      text: '最后更新于',
    },
    outlineTitle: "页面导航",
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
      {text: '指南', link: '/intro/what-is-diesel'},
      {
        text: 'API 参考',
        items: [
          {
            items: [
              { text: 'latest', link: 'https://docs.diesel.rs/master/diesel/index.html' },
              { text: '2.2.x release', link: 'https://docs.diesel.rs/2.2.x/diesel/index.html' },
              { text: '2.1.x release', link: 'https://docs.diesel.rs/2.1.x/diesel/index.html' },
              { text: '2.0.x release', link: 'https://docs.diesel.rs/2.0.x/diesel/index.html' },
              { text: '1.4.x release', link: 'https://docs.diesel.rs/1.4.x/diesel/index.html' },
            ]
          }
        ]
      }
    ],
    sidebar: [
      {
        text: '入门',
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
        ]
      },
      {
        text: "扩展",
        collapsed: false,
        items: [
          {text: "深入了解 Schema", link: "/extensions/schema_in_depth"},
          {text: "扩展 Diesel", link: "/extensions/extending-diesel"},
          {text: "配置 Diesel CLI", link: "/extensions/configuring-diesel-cli"},
        ]
      },
      {
        text: "示例",
        collapsed: false,
        items: [
          {text: "Web CRUD 实践", items: [
              {text: "Postgres", link: "https://github.com/nonfan/diesel-demo/tree/db/postgres-crud"},
              {text: "MySQL", link: "https://github.com/nonfan/diesel-demo/tree/db/mysql-crud"},
              {text: "SQLite", link: "https://github.com/nonfan/diesel-demo/tree/db/sqlite-crud"},
              {text: "Test 测试用例", link: "https://github.com/nonfan/diesel-demo/tree/test/sqlite-crud"},
            ]}
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
