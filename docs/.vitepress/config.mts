import {defineConfig} from 'vitepress'
import {withI18n} from 'vitepress-i18n';

export default defineConfig({
  title: "Diesel 中文文档",
  base: "/rust-demo/",
  description: "Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器",
  head: [['link', {rel: 'icon', href: '/rust-demo/favicon.png'}]],
  lang: "zn",
  locales: {
    root: {
      label: 'Chinese',
      lang: 'fr'
    },
  },
  cleanUrls: true,
  themeConfig: {
    siteTitle: false,
    logo: {
      light: "/logo.svg",
      dark: "/logo-dark.svg"
    },
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
      {icon: 'github', link: 'https://github.com/nonfan'}
    ],
    editLink: {
          pattern: 'https://github.com/nonfan/diesel-demo/edit/docs/docs/:path'
        }
  }
})
