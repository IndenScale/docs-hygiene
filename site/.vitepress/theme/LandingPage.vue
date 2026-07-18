<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useData, useRouter, withBase } from 'vitepress'

const site = (path: string) => withBase(path)
const { lang: routeLang } = useData()
const router = useRouter()

/* ------------------------------------------------------------------ */
/* i18n                                                                */
/* ------------------------------------------------------------------ */

type Lang = 'zh' | 'en'
const lang = computed<Lang>(() => (routeLang.value.toLowerCase().startsWith('en') ? 'en' : 'zh'))
const lp = (path: string) => site((lang.value === 'en' ? '/en' : '') + path)

const dict = {
  zh: {
    htmlLang: 'zh-CN',
    ariaMainNav: '主导航',
    ariaFooterNav: '页脚导航',
    ariaTheme: '切换外观',
    themeToDark: '切换到深色模式',
    themeToLight: '切换到浅色模式',
    ariaLang: '语言切换',
    homeAria: 'Docs Hygiene 首页',
    docs: '文档',
    articles: '文章',
    github: 'GitHub',
    eyebrow: '项目文档检查工具',
    h1a: '让项目文档',
    h1b: '像代码一样',
    h1c: '可以持续检查。',
    summary:
      'Docs Hygiene 在本地和 CI 中检查文档的身份、引用、依赖和新鲜度，在错误继续复制到代码之前暴露断裂。',
    ctaStart: '开始使用',
    ctaSource: '查看源码',
    note: '开源 · 本地运行 · 确定性输出 · 适合 CI',
    graphCaption: '依赖图 · 实时检查',
    graphCmd: 'docs-hygiene watch',
    graphAria: '文档依赖图动画：变化沿依赖边传播并产生 finding',
    statusIdle: '依赖关系稳定 · 上次检查通过',
    statusChange: (id: string) => `检测到上游变化：${id}`,
    statusSpread: (n: number) => `影响沿依赖传播 → ${n} 个下游文档`,
    statusFinding: '阻断性 finding：已审阅状态失效 · DH_PIN_002',
    trust: ['格式正确', '含义一致', '依赖有效', '仍然可信'],
    problemLabel: '文档为什么会失去价值',
    problemTitle: '文件还在，知识已经断了。',
    problemP1:
      '一份文档即使格式正确、链接可点，也可能已经引用了错误的概念、遗漏了上游变化，或与另一个语言版本表达不同约束。',
    problemP2:
      'AI Coding 会更快地消费这些内容，也会更快地把偏差写进代码和新文档。DH 检查的不是文字是否漂亮，而是项目知识之间的关系是否仍然成立。',
    checksLabel: 'DH 持续检查什么',
    checksTitle: '四个简单问题。',
    checks: [
      {
        name: '身份',
        question: '这份文档移动或改名后，还是同一个东西吗？',
        detail: '用稳定身份把内容含义与文件路径分开。'
      },
      {
        name: '定义',
        question: '同一个概念，到底哪一处定义说了算？',
        detail: '声明权威来源，暴露重复和冲突的定义。'
      },
      {
        name: '变化',
        question: '上游改了，哪些下游内容需要重新检查？',
        detail: '沿显式依赖传播影响，并用 Pin 识别确定性失效。'
      },
      {
        name: '证据',
        question: '需求、Issue 和实现证据还连得起来吗？',
        detail: '保留从产品意图到交付结果的可追溯关系。'
      }
    ],
    flowLabel: '放进现有工程流程',
    flowTitle: '在提交时发现，不要在事故后考古。',
    flowP:
      'DH 从项目根目录读取显式策略，对相同输入给出相同结果。开发者可以在本地解释规则，团队可以在 CI 中把关键断裂设为门禁，其他工具仍负责格式、拼写和外部链接。',
    flowLink: '查看 CI 接入方式',
    cmdNote: '结构、身份、依赖和拓扑结果可以分别检查',
    boundaryLabel: '自动化的边界',
    boundaryTitle: '机器暴露断裂，人对含义负责。',
    boundaries: [
      ['DH 检查', '身份、引用、依赖、生命周期和追溯关系'],
      ['DH 不判断', '一段自然语言在业务上是否正确'],
      ['DH 给出', '确定性 finding、影响范围和可审计更新'],
      ['人来决定', '业务含义、产品取舍和架构选择']
    ],
    readingLabel: '进一步了解',
    readingTitle: '从一个实际问题开始。',
    readingAll: '全部文章',
    readings: [
      { topic: '判断文档是否可信', title: '为什么死链检查远远不够', time: '6 分钟' },
      { topic: '稳定身份', title: '文件路径不是文档身份', time: '5 分钟' },
      { topic: '知识依赖', 'title': '“文档的 Dependabot”究竟是什么意思', time: '7 分钟' }
    ],
    finalLabel: '从一个仓库开始',
    finalTitle: '先让断裂变得可见。',
    finalBtn: '阅读快速开始',
    footerTag: '项目文档的依赖与一致性检查工具。',
    footerLicense: 'Docs Hygiene · MIT License'
  },
  en: {
    htmlLang: 'en',
    ariaMainNav: 'Primary navigation',
    ariaFooterNav: 'Footer navigation',
    ariaTheme: 'Toggle appearance',
    themeToDark: 'Switch to dark mode',
    themeToLight: 'Switch to light mode',
    ariaLang: 'Language',
    homeAria: 'Docs Hygiene home',
    docs: 'Docs',
    articles: 'Articles',
    github: 'GitHub',
    eyebrow: 'Project documentation checker',
    h1a: 'Check project docs',
    h1b: 'like code,',
    h1c: 'on every change.',
    summary:
      'Docs Hygiene checks identity, references, dependencies, and freshness — locally and in CI — exposing broken knowledge before it is copied into code.',
    ctaStart: 'Get started',
    ctaSource: 'View source',
    note: 'Open source · Runs locally · Deterministic output · CI-ready',
    graphCaption: 'Dependency graph · live',
    graphCmd: 'docs-hygiene watch',
    graphAria: 'Animated document dependency graph: a change propagates along edges and raises a finding',
    statusIdle: 'Dependencies stable · last check passed',
    statusChange: (id: string) => `Upstream change detected: ${id}`,
    statusSpread: (n: number) => `Impact propagating → ${n} downstream docs`,
    statusFinding: 'Blocking finding: reviewed state stale · DH_PIN_002',
    trust: ['Well-formatted', 'Consistent meaning', 'Valid dependencies', 'Still trustworthy'],
    problemLabel: 'Why documents lose value',
    problemTitle: 'The files remain. The knowledge broke.',
    problemP1:
      'A document can be well-formatted with working links, yet still reference the wrong concept, miss an upstream change, or express different constraints than another language version.',
    problemP2:
      'AI coding consumes this content faster — and copies the drift into code and new docs just as fast. DH does not check whether prose is pretty; it checks whether the relationships between project knowledge still hold.',
    checksLabel: 'What DH keeps checking',
    checksTitle: 'Four simple questions.',
    checks: [
      {
        name: 'Identity',
        question: 'If this document moves or is renamed, is it still the same thing?',
        detail: 'Stable identity separates meaning from file paths.'
      },
      {
        name: 'Definition',
        question: 'For the same concept, which definition actually wins?',
        detail: 'Declare authoritative sources; expose duplicate and conflicting definitions.'
      },
      {
        name: 'Change',
        question: 'When upstream changes, what downstream content must be re-checked?',
        detail: 'Propagate impact along explicit dependencies; pins mark deterministic staleness.'
      },
      {
        name: 'Evidence',
        question: 'Do requirements, issues, and implementation evidence still connect?',
        detail: 'Preserve traceability from product intent to shipped results.'
      }
    ],
    flowLabel: 'Fits your existing workflow',
    flowTitle: 'Catch breaks at commit time, not in postmortems.',
    flowP:
      'DH reads explicit policy from the project root and returns identical results for identical input. Developers can explain rules locally, teams can gate critical breaks in CI, and other tools keep owning format, spelling, and external links.',
    flowLink: 'See CI integration',
    cmdNote: 'Structure, identity, dependencies, and topology can be checked separately',
    boundaryLabel: 'The boundary of automation',
    boundaryTitle: 'Machines expose the breaks. Humans own the meaning.',
    boundaries: [
      ['DH checks', 'Identity, references, dependencies, lifecycle, and traceability'],
      ['DH does not judge', 'Whether a passage is business-correct'],
      ['DH provides', 'Deterministic findings, blast radius, auditable updates'],
      ['Humans decide', 'Business meaning, product trade-offs, architecture']
    ],
    readingLabel: 'Go deeper',
    readingTitle: 'Start with a real problem.',
    readingAll: 'All articles',
    readings: [
      { topic: 'Trusting docs', title: 'Why link checking is not enough', time: '6 min' },
      { topic: 'Stable identity', title: 'A file path is not a document identity', time: '5 min' },
      { topic: 'Knowledge deps', title: 'What “a Dependabot for docs” actually means', time: '7 min' }
    ],
    finalLabel: 'Start with one repository',
    finalTitle: 'Make the breaks visible first.',
    finalBtn: 'Read the quickstart',
    footerTag: 'Dependency and consistency checking for project documentation.',
    footerLicense: 'Docs Hygiene · MIT License'
  }
} as const

const t = computed(() => dict[lang.value])

function setLang(next: Lang) {
  if (next === lang.value) return
  router.go(next === 'en' ? '/en/' : '/')
}

/* ------------------------------------------------------------------ */
/* Theme toggle (compatible with VitePress appearance key)             */
/* ------------------------------------------------------------------ */

const APPEARANCE_KEY = 'vitepress-theme-appearance'
const isDark = ref(false)
let userChoseTheme = false

function toggleTheme() {
  userChoseTheme = true
  isDark.value = !isDark.value
  document.documentElement.classList.toggle('dark', isDark.value)
  try {
    localStorage.setItem(APPEARANCE_KEY, isDark.value ? 'dark' : 'light')
  } catch {
    /* private mode */
  }
}

/* ------------------------------------------------------------------ */
/* Scroll-reveal motion (Kimi: restrained entrance, ease-out curve)    */
/* ------------------------------------------------------------------ */

let revealObserver: IntersectionObserver | null = null

function setupReveal(root: HTMLElement) {
  const targets = Array.from(root.querySelectorAll<HTMLElement>('[data-reveal]'))
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
    targets.forEach((el) => el.classList.add('revealed'))
    return
  }
  // Stagger siblings inside a reveal group: 60ms steps, capped.
  root.querySelectorAll<HTMLElement>('[data-reveal-group]').forEach((group) => {
    Array.from(group.querySelectorAll<HTMLElement>('[data-reveal]')).forEach((el, i) => {
      el.style.transitionDelay = `${Math.min(i * 60, 300)}ms`
    })
  })
  revealObserver = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          entry.target.classList.add('revealed')
          revealObserver?.unobserve(entry.target)
        }
      }
    },
    { threshold: 0.12, rootMargin: '0px 0px -8% 0px' }
  )
  targets.forEach((el) => revealObserver!.observe(el))
}

/* ------------------------------------------------------------------ */
/* Dependency-graph canvas — digital art with product semantics        */
/* ------------------------------------------------------------------ */

interface GNode {
  id: string
  x: number
  y: number
}

const NODES: GNode[] = [
  { id: 'PRD-CORE', x: 0.1, y: 0.28 },
  { id: 'UL-RATE-LIMIT', x: 0.1, y: 0.7 },
  { id: 'GATEWAY', x: 0.38, y: 0.14 },
  { id: 'I18N-ZH', x: 0.38, y: 0.48 },
  { id: 'OPS-GUIDE', x: 0.38, y: 0.84 },
  { id: 'API-REF', x: 0.64, y: 0.3 },
  { id: 'SDK-NOTES', x: 0.64, y: 0.66 },
  { id: 'RELEASE', x: 0.9, y: 0.48 }
]

const EDGES: Array<[string, string]> = [
  ['PRD-CORE', 'GATEWAY'],
  ['PRD-CORE', 'I18N-ZH'],
  ['UL-RATE-LIMIT', 'I18N-ZH'],
  ['UL-RATE-LIMIT', 'OPS-GUIDE'],
  ['GATEWAY', 'API-REF'],
  ['I18N-ZH', 'API-REF'],
  ['I18N-ZH', 'SDK-NOTES'],
  ['OPS-GUIDE', 'SDK-NOTES'],
  ['API-REF', 'RELEASE'],
  ['SDK-NOTES', 'RELEASE']
]

const SOURCES = ['PRD-CORE', 'UL-RATE-LIMIT']
const NODE_BY_ID = new Map(NODES.map((n) => [n.id, n]))
const ADJ = new Map<string, string[]>()
for (const [a, b] of EDGES) {
  if (!ADJ.has(a)) ADJ.set(a, [])
  ADJ.get(a)!.push(b)
}

interface Palette {
  edge: string
  nodeFill: string
  nodeStroke: string
  blue: string
  blueSoft: string
  red: string
  redSoft: string
}

const graphCanvas = ref<HTMLCanvasElement | null>(null)
const statusText = ref('')
const statusKind = ref<'ok' | 'change' | 'finding'>('ok')

let raf = 0
let resizeObserver: ResizeObserver | null = null
let palette: Palette | null = null
let reducedMotion = false
let cw = 0
let ch = 0

function readPalette(): Palette {
  const cs = getComputedStyle(document.documentElement)
  const v = (name: string) => cs.getPropertyValue(name).trim()
  return {
    edge: v('--k-sep'),
    nodeFill: v('--k-bg'),
    nodeStroke: v('--k-line-strong'),
    blue: v('--k-blue'),
    blueSoft: v('--k-blue-10'),
    red: v('--k-red'),
    redSoft: v('--k-red-10')
  }
}

interface EdgeEvent {
  from: string
  to: string
  start: number
  dur: number
}

interface Cycle {
  source: string
  events: EdgeEvent[]
  activated: Map<string, number> // node -> activation time
  findingNode: string
  findingAt: number
  total: number
}

function bfsOrder(source: string): Array<[string, string]> {
  const seen = new Set([source])
  const queue = [source]
  const out: Array<[string, string]> = []
  while (queue.length) {
    const cur = queue.shift()!
    for (const next of ADJ.get(cur) ?? []) {
      if (seen.has(next)) continue
      seen.add(next)
      out.push([cur, next])
      queue.push(next)
    }
  }
  return out
}

function buildCycle(): Cycle {
  const source = SOURCES[Math.floor(Math.random() * SOURCES.length)]
  const order = bfsOrder(source)
  const events: EdgeEvent[] = []
  const activated = new Map<string, number>()
  let tms = 0
  for (const [from, to] of order) {
    events.push({ from, to, start: tms, dur: 420 })
    activated.set(to, tms + 420)
    tms += 300
  }
  const findingNode = order.length ? order[order.length - 1][1] : source
  const findingAt = tms + 260
  return { source, events, activated, findingNode, findingAt, total: findingAt + 1500 }
}

const easeInOut = (p: number) => (p < 0.5 ? 2 * p * p : 1 - Math.pow(-2 * p + 2, 2) / 2)

function px(n: GNode) {
  return { x: n.x * cw, y: n.y * ch }
}

function drawNode(ctx: CanvasRenderingContext2D, id: string, p: Palette, mode: 'idle' | 'active' | 'finding', pulse = 0) {
  const n = NODE_BY_ID.get(id)!
  const { x, y } = px(n)
  const size = 13
  const r = 4
  if (mode !== 'idle') {
    const color = mode === 'finding' ? p.red : p.blue
    const soft = mode === 'finding' ? p.redSoft : p.blueSoft
    ctx.beginPath()
    ctx.arc(x, y, size / 2 + 7 + pulse * 3, 0, Math.PI * 2)
    ctx.fillStyle = soft
    ctx.fill()
    ctx.strokeStyle = color
    ctx.lineWidth = 1.5
  } else {
    ctx.strokeStyle = p.nodeStroke
    ctx.lineWidth = 1
  }
  ctx.beginPath()
  ctx.roundRect(x - size / 2, y - size / 2, size, size, r)
  ctx.fillStyle = p.nodeFill
  ctx.fill()
  ctx.stroke()
}

function drawBase(ctx: CanvasRenderingContext2D, p: Palette) {
  ctx.clearRect(0, 0, cw, ch)
  ctx.strokeStyle = p.edge
  ctx.lineWidth = 1
  for (const [a, b] of EDGES) {
    const na = px(NODE_BY_ID.get(a)!)
    const nb = px(NODE_BY_ID.get(b)!)
    ctx.beginPath()
    ctx.moveTo(na.x, na.y)
    ctx.lineTo(nb.x, nb.y)
    ctx.stroke()
  }
  for (const n of NODES) drawNode(ctx, n.id, p, 'idle')
}

let cycle: Cycle | null = null
let cycleStart = 0
let lastPhase = ''

function frame(now: number) {
  const canvas = graphCanvas.value
  const ctx = canvas?.getContext('2d')
  if (!canvas || !ctx || !palette) return
  if (!cycle) {
    cycle = buildCycle()
    cycleStart = now
    lastPhase = ''
  }
  const elapsed = now - cycleStart
  if (elapsed > cycle.total + 1400) {
    cycle = null
    statusKind.value = 'ok'
    statusText.value = t.value.statusIdle
    drawBase(ctx, palette)
    raf = requestAnimationFrame(frame)
    return
  }

  // Phase → status line (idempotent)
  const phase = elapsed < 200 ? 'idle' : elapsed < cycle.findingAt ? 'spread' : 'finding'
  if (phase !== lastPhase) {
    lastPhase = phase
    if (phase === 'idle') {
      statusKind.value = 'change'
      statusText.value = t.value.statusChange(cycle.source)
    } else if (phase === 'spread') {
      statusKind.value = 'change'
      statusText.value = t.value.statusSpread(cycle.events.length)
    } else {
      statusKind.value = 'finding'
      statusText.value = t.value.statusFinding
    }
  }

  drawBase(ctx, palette)

  // Activated nodes (blue) and finding node (red)
  for (const [id, at] of cycle.activated) {
    if (elapsed >= at) drawNode(ctx, id, palette, 'active')
  }
  drawNode(ctx, cycle.source, palette, 'active')
  if (elapsed >= cycle.findingAt) {
    const pulse = 0.5 + 0.5 * Math.sin((elapsed - cycle.findingAt) / 130)
    drawNode(ctx, cycle.findingNode, palette, 'finding', pulse)
  }

  // Traveling pulses along edges
  for (const ev of cycle.events) {
    const local = (elapsed - ev.start) / ev.dur
    if (local < 0 || local > 1) continue
    const a = px(NODE_BY_ID.get(ev.from)!)
    const b = px(NODE_BY_ID.get(ev.to)!)
    const k = easeInOut(local)
    const x = a.x + (b.x - a.x) * k
    const y = a.y + (b.y - a.y) * k
    ctx.beginPath()
    ctx.arc(x, y, 6, 0, Math.PI * 2)
    ctx.fillStyle = palette.blueSoft
    ctx.fill()
    ctx.beginPath()
    ctx.arc(x, y, 2.6, 0, Math.PI * 2)
    ctx.fillStyle = palette.blue
    ctx.fill()
  }

  raf = requestAnimationFrame(frame)
}

function resizeCanvas() {
  const canvas = graphCanvas.value
  if (!canvas) return
  const rect = canvas.getBoundingClientRect()
  const dpr = Math.min(window.devicePixelRatio || 1, 2)
  canvas.width = Math.max(1, Math.round(rect.width * dpr))
  canvas.height = Math.max(1, Math.round(rect.height * dpr))
  const ctx = canvas.getContext('2d')
  ctx?.setTransform(dpr, 0, 0, dpr, 0, 0)
  cw = rect.width
  ch = rect.height
  if (reducedMotion && ctx && palette) drawBase(ctx, palette)
}

function onVisibility() {
  if (document.hidden) {
    cancelAnimationFrame(raf)
    cycle = null
  } else if (!reducedMotion) {
    cycle = null
    raf = requestAnimationFrame(frame)
  }
}

watch(isDark, () => {
  palette = readPalette()
  const ctx = graphCanvas.value?.getContext('2d')
  if (ctx && palette && reducedMotion) drawBase(ctx, palette)
})

// Keep status line language in sync without restarting the loop
watch(lang, () => {
  statusKind.value = 'ok'
  statusText.value = t.value.statusIdle
  cycle = null
})

/* ------------------------------------------------------------------ */

const shell = ref<HTMLElement | null>(null)
let mediaQuery: MediaQueryList | null = null
let onMediaChange: ((e: MediaQueryListEvent) => void) | null = null

onMounted(() => {
  // Theme: VitePress has already resolved the initial class; adopt it.
  isDark.value = document.documentElement.classList.contains('dark')
  try {
    userChoseTheme = localStorage.getItem(APPEARANCE_KEY) === 'dark' || localStorage.getItem(APPEARANCE_KEY) === 'light'
  } catch {
    userChoseTheme = false
  }
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  onMediaChange = (e) => {
    if (userChoseTheme) return
    isDark.value = e.matches
  }
  mediaQuery.addEventListener('change', onMediaChange)

  statusText.value = t.value.statusIdle

  if (shell.value) setupReveal(shell.value)

  // Canvas
  reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches
  palette = readPalette()
  const canvas = graphCanvas.value
  if (canvas) {
    resizeObserver = new ResizeObserver(resizeCanvas)
    resizeObserver.observe(canvas)
    resizeCanvas()
    if (reducedMotion) {
      const ctx = canvas.getContext('2d')
      if (ctx && palette) drawBase(ctx, palette)
    } else {
      raf = requestAnimationFrame(frame)
      document.addEventListener('visibilitychange', onVisibility)
    }
  }
})

onBeforeUnmount(() => {
  cancelAnimationFrame(raf)
  resizeObserver?.disconnect()
  revealObserver?.disconnect()
  document.removeEventListener('visibilitychange', onVisibility)
  if (mediaQuery && onMediaChange) mediaQuery.removeEventListener('change', onMediaChange)
})
</script>

<template>
  <div ref="shell" class="landing-shell">
    <!-- ================= Header ================= -->
    <header class="landing-header">
      <div class="header-inner">
        <a class="wordmark" :href="lp('/')" :aria-label="t.homeAria">
          <span class="wordmark-mark" aria-hidden="true">DH</span>
          <span class="wordmark-title">Docs Hygiene</span>
        </a>

        <nav :aria-label="t.ariaMainNav" class="header-nav">
          <a :href="lp('/docs/')">{{ t.docs }}</a>
          <a :href="lp('/articles/')">{{ t.articles }}</a>
          <a href="https://github.com/IndenScale/docs-hygiene" target="_blank" rel="noreferrer">{{ t.github }}</a>
        </nav>

        <div class="header-controls">
          <button
            type="button"
            class="icon-button"
            :aria-label="t.ariaTheme"
            :title="isDark ? t.themeToLight : t.themeToDark"
            :aria-pressed="isDark"
            @click="toggleTheme"
          >
            <svg v-if="!isDark" class="control-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true">
              <circle cx="12" cy="12" r="4.2" />
              <path d="M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M4.9 4.9l1.6 1.6M17.5 17.5l1.6 1.6M19.1 4.9l-1.6 1.6M6.5 17.5l-1.6 1.6" />
            </svg>
            <svg v-else class="control-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <path d="M20.4 14.2A8.5 8.5 0 1 1 9.8 3.6a7 7 0 1 0 10.6 10.6Z" />
            </svg>
          </button>

          <div class="segmented" role="radiogroup" :aria-label="t.ariaLang">
            <button
              type="button"
              role="radio"
              :aria-checked="lang === 'zh'"
              class="segment"
              :class="{ selected: lang === 'zh' }"
              @click="setLang('zh')"
            >中</button>
            <button
              type="button"
              role="radio"
              :aria-checked="lang === 'en'"
              class="segment"
              :class="{ selected: lang === 'en' }"
              @click="setLang('en')"
            >EN</button>
          </div>
        </div>
      </div>
    </header>

    <!-- ================= Body ================= -->
    <main>
      <section class="hero">
        <div class="hero-copy" data-reveal-group>
          <p class="eyebrow" data-reveal><span class="live-dot" aria-hidden="true" />{{ t.eyebrow }}</p>
          <h1 data-reveal>
            {{ t.h1a }}<br><em>{{ t.h1b }}</em><br>{{ t.h1c }}
          </h1>
          <p class="hero-summary" data-reveal>{{ t.summary }}</p>
          <div class="hero-actions" data-reveal>
            <a class="button button-primary" :href="lp('/docs/getting-started')">
              {{ t.ctaStart }}
              <svg class="button-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M13 6l6 6-6 6" /></svg>
            </a>
            <a class="button button-secondary" href="https://github.com/IndenScale/docs-hygiene" target="_blank" rel="noreferrer">{{ t.ctaSource }}</a>
          </div>
          <p class="hero-note" data-reveal>{{ t.note }}</p>
        </div>

        <div class="graph-panel" data-reveal>
          <div class="graph-head">
            <span class="graph-caption"><span class="live-dot" aria-hidden="true" />{{ t.graphCaption }}</span>
            <code>{{ t.graphCmd }}</code>
          </div>
          <canvas ref="graphCanvas" class="graph-canvas" :aria-label="t.graphAria" role="img"></canvas>
          <div class="graph-status">
            <span class="status-dot" :class="statusKind" aria-hidden="true" />
            <span class="graph-status-text">{{ statusText }}</span>
          </div>
        </div>
      </section>

      <section class="trust-strip" data-reveal aria-label="Docs Hygiene">
        <template v-for="(word, i) in t.trust" :key="word">
          <p>{{ word }}</p>
          <span v-if="i < t.trust.length - 1" aria-hidden="true">≠</span>
        </template>
      </section>

      <section class="section problem-section">
        <div class="section-heading" data-reveal-group>
          <p class="section-label" data-reveal>{{ t.problemLabel }}</p>
          <h2 data-reveal>{{ t.problemTitle }}</h2>
        </div>
        <div class="problem-copy" data-reveal-group>
          <p data-reveal>{{ t.problemP1 }}</p>
          <p data-reveal>{{ t.problemP2 }}</p>
        </div>
      </section>

      <section class="section checks-section">
        <div class="section-heading" data-reveal-group>
          <p class="section-label" data-reveal>{{ t.checksLabel }}</p>
          <h2 data-reveal>{{ t.checksTitle }}</h2>
        </div>
        <div class="check-ledger" data-reveal-group>
          <article v-for="(check, index) in t.checks" :key="check.name" class="check-row" data-reveal>
            <span class="check-number">0{{ index + 1 }}</span>
            <h3>{{ check.name }}</h3>
            <div>
              <p>{{ check.question }}</p>
              <small>{{ check.detail }}</small>
            </div>
          </article>
        </div>
      </section>

      <section class="section workflow-section">
        <div class="workflow-copy" data-reveal-group>
          <p class="section-label" data-reveal>{{ t.flowLabel }}</p>
          <h2 data-reveal>{{ t.flowTitle }}</h2>
          <p data-reveal>{{ t.flowP }}</p>
          <a class="text-link" :href="lp('/docs/ci')" data-reveal>
            {{ t.flowLink }}
            <svg class="button-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M13 6l6 6-6 6" /></svg>
          </a>
        </div>
        <div class="command-stack" data-reveal aria-label="Docs Hygiene CLI">
          <div><span aria-hidden="true">$</span><code>docs-hygiene check --fail-on-warning</code></div>
          <div><span aria-hidden="true">$</span><code>docs-hygiene explain-rules</code></div>
          <div><span aria-hidden="true">$</span><code>docs-hygiene profile</code></div>
          <p><span class="pass-dot" aria-hidden="true" />{{ t.cmdNote }}</p>
        </div>
      </section>

      <section class="section boundary-section">
        <div class="section-heading" data-reveal-group>
          <p class="section-label" data-reveal>{{ t.boundaryLabel }}</p>
          <h2 data-reveal>{{ t.boundaryTitle }}</h2>
        </div>
        <dl class="boundary-table" data-reveal-group>
          <template v-for="item in t.boundaries" :key="item[0]">
            <dt data-reveal>{{ item[0] }}</dt>
            <dd data-reveal>{{ item[1] }}</dd>
          </template>
        </dl>
      </section>

      <section class="section reading-section">
        <div class="reading-head" data-reveal-group>
          <div>
            <p class="section-label" data-reveal>{{ t.readingLabel }}</p>
            <h2 data-reveal>{{ t.readingTitle }}</h2>
          </div>
          <a class="text-link" :href="lp('/articles/')" data-reveal>
            {{ t.readingAll }}
            <svg class="button-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M13 6l6 6-6 6" /></svg>
          </a>
        </div>
        <div class="reading-list" data-reveal-group>
          <a :href="lp('/articles/why-links-are-not-enough')" data-reveal>
            <span>{{ t.readings[0].topic }}</span>
            <strong>{{ t.readings[0].title }}</strong>
            <em>{{ t.readings[0].time }}</em>
          </a>
          <a :href="lp('/articles/document-identity-is-not-a-path')" data-reveal>
            <span>{{ t.readings[1].topic }}</span>
            <strong>{{ t.readings[1].title }}</strong>
            <em>{{ t.readings[1].time }}</em>
          </a>
          <a :href="lp('/articles/docs-dependabot')" data-reveal>
            <span>{{ t.readings[2].topic }}</span>
            <strong>{{ t.readings[2].title }}</strong>
            <em>{{ t.readings[2].time }}</em>
          </a>
        </div>
      </section>

      <section class="final-cta" data-reveal>
        <p class="final-label">{{ t.finalLabel }}</p>
        <h2>{{ t.finalTitle }}</h2>
        <div class="final-actions">
          <a class="button button-primary" :href="lp('/docs/getting-started')">
            {{ t.finalBtn }}
            <svg class="button-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M13 6l6 6-6 6" /></svg>
          </a>
          <code class="final-code">docs-hygiene check .</code>
        </div>
      </section>
    </main>

    <!-- ================= Footer ================= -->
    <footer class="landing-footer">
      <a class="wordmark" :href="lp('/')">
        <span class="wordmark-mark" aria-hidden="true">DH</span>
        <span class="wordmark-title">Docs Hygiene</span>
      </a>
      <p class="footer-tag">{{ t.footerTag }}</p>
      <nav :aria-label="t.ariaFooterNav" class="footer-nav">
        <a :href="lp('/docs/')">{{ t.docs }}</a>
        <a :href="lp('/articles/')">{{ t.articles }}</a>
        <a href="https://github.com/IndenScale/docs-hygiene" target="_blank" rel="noreferrer">{{ t.github }}</a>
      </nav>
      <p class="footer-license">{{ t.footerLicense }}</p>
    </footer>
  </div>
</template>
