import DefaultTheme from 'vitepress/theme-without-fonts'
import type { Theme } from 'vitepress'
import LandingPage from './LandingPage.vue'
import ArticlesIndex from './ArticlesIndex.vue'
import './custom.css'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('LandingPage', LandingPage)
    app.component('ArticlesIndex', ArticlesIndex)
  }
} satisfies Theme
