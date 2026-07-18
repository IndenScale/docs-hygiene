import { defineConfig } from 'vitepress'

const zhSidebar = [
  {
    text: '开始使用',
    items: [
      { text: '文档首页', link: '/docs/' },
      { text: '快速开始', link: '/docs/getting-started' },
      { text: 'DH 如何工作', link: '/docs/how-it-works' }
    ]
  },
  {
    text: '使用与集成',
    items: [
      { text: '项目配置', link: '/docs/configuration' },
      { text: '接入 CI', link: '/docs/ci' }
    ]
  },
  {
    text: '核心概念',
    items: [
      { text: '身份与依赖', link: '/docs/concepts/identity-and-dependency' },
      { text: '能力与边界', link: '/docs/capabilities' }
    ]
  }
]

const enSidebar = [
  {
    text: 'Getting Started',
    items: [
      { text: 'Docs Home', link: '/en/docs/' },
      { text: 'Quickstart', link: '/en/docs/getting-started' },
      { text: 'How DH Works', link: '/en/docs/how-it-works' }
    ]
  },
  {
    text: 'Usage & Integration',
    items: [
      { text: 'Project Configuration', link: '/en/docs/configuration' },
      { text: 'CI Integration', link: '/en/docs/ci' }
    ]
  },
  {
    text: 'Core Concepts',
    items: [
      { text: 'Identity & Dependencies', link: '/en/docs/concepts/identity-and-dependency' },
      { text: 'Capabilities & Boundaries', link: '/en/docs/capabilities' }
    ]
  }
]

export default defineConfig({
  base: process.env.SITE_BASE || '/',
  title: 'Docs Hygiene',
  description: '让项目文档像代码一样，可以持续检查。',
  cleanUrls: true,
  lastUpdated: true,
  head: [
    ['meta', { name: 'theme-color', media: '(prefers-color-scheme: light)', content: '#ffffff' }],
    ['meta', { name: 'theme-color', media: '(prefers-color-scheme: dark)', content: '#121212' }],
    ['meta', { name: 'color-scheme', content: 'light dark' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:site_name', content: 'Docs Hygiene' }],
    ['meta', { property: 'og:title', content: 'Docs Hygiene — 让项目文档像代码一样持续检查' }],
    ['meta', { property: 'og:description', content: '在本地和 CI 中发现项目文档之间已经断掉、冲突或过期的关系。' }]
  ],
  markdown: {
    lineNumbers: true
  },
  locales: {
    root: {
      label: '简体中文',
      lang: 'zh-CN'
    },
    en: {
      label: 'English',
      lang: 'en-US',
      link: '/en/',
      themeConfig: {
        nav: [
          { text: 'Home', link: '/en/' },
          { text: 'Docs', link: '/en/docs/' },
          { text: 'Articles', link: '/en/articles/' },
          { text: 'GitHub', link: 'https://github.com/IndenScale/docs-hygiene' }
        ],
        sidebar: {
          '/en/docs/': enSidebar
        },
        outline: { level: [2, 3], label: 'On this page' },
        returnToTopLabel: 'Back to top',
        sidebarMenuLabel: 'Menu',
        darkModeSwitchLabel: 'Appearance',
        lightModeSwitchTitle: 'Switch to light mode',
        darkModeSwitchTitle: 'Switch to dark mode',
        skipToContentLabel: 'Skip to content',
        docFooter: { prev: 'Previous', next: 'Next' },
        lastUpdated: { text: 'Last updated', formatOptions: { dateStyle: 'medium' } },
        footer: {
          message: 'Well-formatted docs are not necessarily trustworthy knowledge.',
          copyright: 'Docs Hygiene · MIT License'
        }
      }
    }
  },
  themeConfig: {
    siteTitle: 'Docs Hygiene',
    nav: [
      { text: '首页', link: '/' },
      { text: '文档', link: '/docs/' },
      { text: '文章', link: '/articles/' },
      { text: 'GitHub', link: 'https://github.com/IndenScale/docs-hygiene' }
    ],
    sidebar: {
      '/docs/': zhSidebar
    },
    search: {
      provider: 'local',
      options: {
        locales: {
          root: {
            translations: {
              button: { buttonText: '搜索文档', buttonAriaLabel: '搜索文档' },
              modal: {
                noResultsText: '没有找到相关内容',
                resetButtonTitle: '清除查询',
                footer: {
                  selectText: '选择',
                  navigateText: '切换',
                  closeText: '关闭'
                }
              }
            }
          }
        }
      }
    },
    outline: { level: [2, 3], label: '本页内容' },
    returnToTopLabel: '返回顶部',
    sidebarMenuLabel: '文档菜单',
    darkModeSwitchLabel: '切换外观',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    skipToContentLabel: '跳到正文',
    docFooter: { prev: '上一页', next: '下一页' },
    lastUpdated: { text: '最后更新于', formatOptions: { dateStyle: 'medium' } },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/IndenScale/docs-hygiene' }
    ],
    footer: {
      message: '文档格式正确，不代表项目知识仍然可信。',
      copyright: 'Docs Hygiene · MIT License'
    }
  }
})
