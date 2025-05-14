import {defineConfig} from 'vitepress'
import {withI18n} from 'vitepress-i18n';

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
        formatOptions: {
          dateStyle: 'full',
          timeStyle: 'medium'
        },
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
