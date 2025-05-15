import {defineConfig} from 'vitepress'
export default defineConfig({
  title: "Diesel 中文文档",
  base: "/rust-demo/",
  description: "Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器",
  head: [['link', {rel: 'icon', href: '/rust-demo/favicon.png'}]],
  lang: "zh-CN",
  locales: {
    root: {
      label: 'Chinese',
      lang: 'fr'
    },
  },
  cleanUrls: true,
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
      {text: '指南', link: '/markdown-examples'}
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
          {text: '连接池', link: '/basic-usage/connection-database'},
          {text: '定义模型', link: '/basic-usage/defining-models'},
          {text: '查询执行', link: '/basic-usage/read'},
          {text: '插入数据', link: '/basic-usage/create'},
          {text: '更新数据', link: '/basic-usage/update'},
          {text: '删除数据', link: '/basic-usage/delete'},
        ]
      }
    ],
    search: {
      provider: 'local'
    },
    socialLinks: [
      {icon: 'github', link: 'https://github.com/nonfan/rust-demo'}
    ],
    editLink: {
      pattern: 'https://github.com/nonfan/diesel-demo/edit/docs/docs/:path',
      text: "在 GitHub 上编辑此页面"
    },
    footer: {
      message: '基于 MIT 许可发布',
      copyright: 'Copyright © 2025-present <a href="https://github.com/nonfan">MOFAN</a>'
    },
  }
})
