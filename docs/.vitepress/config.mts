import { defineConfig } from 'vitepress'
import { withI18n } from 'vitepress-i18n';

// https://vitepress.dev/reference/site-config
const vitePressOptions = defineConfig({
                                    title: "Diesel 中文文档",
                                    base: "/rust-demo/",
                                    description: "Diesel 是一个安全、可扩展的 Rust ORM 和查询构建器",
                                    head: [['link', { rel: 'icon', href: '/rust-demo/favicon.png' }]],
                                    lang:"zn",
                                    locales: {
                                        root: {
                                          label: 'Chinese',
                                          lang: 'fr'
                                        },
                                    },
                                    themeConfig: {
                                      // https://vitepress.dev/reference/default-theme-config
                                      siteTitle: false,
                                      logo: {
                                          light: "/logo.svg",
                                          dark: "/logo-dark.svg"
                                          },
                                      nav: [
                                        { text: '首页', link: '/' },
                                        { text: '示例', link: '/markdown-examples' }
                                      ],
                                      sidebar: [
                                        {
                                          text: '示例',
                                          items: [
                                            { text: 'Markdown Examples', link: '/markdown-examples' },
                                            { text: 'Runtime API Examples', link: '/api-examples' }
                                          ]
                                        }
                                      ],

                                      socialLinks: [
                                        { icon: 'github', link: 'https://github.com/nonfan' }
                                      ]
                                    }
                                  })

const vitePressI18nOptions = {
  locales: ['zhHans']
};

export default defineConfig(withI18n(vitePressOptions, vitePressI18nOptions));

