<script setup lang="ts">
import { computed } from 'vue'
import { useData, withBase } from 'vitepress'

const { lang } = useData()
const isEn = computed(() => lang.value.toLowerCase().startsWith('en'))
const prefix = computed(() => (isEn.value ? '/en' : ''))

const paths = [
  '/articles/why-links-are-not-enough',
  '/articles/document-identity-is-not-a-path',
  '/articles/docs-dependabot'
]

const copy = {
  zh: [
    {
      topic: '判断文档是否可信',
      title: '为什么死链检查远远不够',
      summary: '链接可以正常打开，引用的含义却已经错了。文档治理必须从路径可达走向关系有效。',
      time: '6 分钟'
    },
    {
      topic: '稳定身份',
      title: '文件路径不是文档身份',
      summary: '当文件移动、改名或拆分时，怎样让需求、概念和决策仍然保持同一个身份。',
      time: '5 分钟'
    },
    {
      topic: '知识依赖',
      title: '“文档的 Dependabot”究竟是什么意思',
      summary: '软件依赖有版本和更新反馈，知识依赖也需要可见性、失效证据与明确的复核动作。',
      time: '7 分钟'
    }
  ],
  en: [
    {
      topic: 'Trusting docs',
      title: 'Why link checking is not enough',
      summary:
        'Links can keep working while the meaning they point to has already gone wrong. Documentation governance must move from reachable paths to valid relationships.',
      time: '6 min'
    },
    {
      topic: 'Stable identity',
      title: 'A file path is not a document identity',
      summary:
        'How requirements, concepts, and decisions keep the same identity when a document moves, gets renamed, or is split apart.',
      time: '5 min'
    },
    {
      topic: 'Knowledge deps',
      title: 'What “a Dependabot for docs” actually means',
      summary:
        'Software dependencies have versions and update feedback; knowledge dependencies need visibility, staleness evidence, and an explicit re-review action.',
      time: '7 min'
    }
  ]
}

const list = computed(() => (isEn.value ? copy.en : copy.zh))
</script>

<template>
  <div class="article-index">
    <a
      v-for="(article, index) in list"
      :key="paths[index]"
      :href="withBase(prefix + paths[index])"
      class="article-index-row"
    >
      <span class="article-order">0{{ index + 1 }}</span>
      <div>
        <small>{{ article.topic }}</small>
        <h2>{{ article.title }}</h2>
        <p>{{ article.summary }}</p>
      </div>
      <span class="article-time">{{ article.time }} <b>→</b></span>
    </a>
  </div>
</template>
