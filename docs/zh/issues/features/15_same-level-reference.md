---
id: FEATURE-015
epic: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [FEATURE-003, FEATURE-014]
---

# FEATURE-015 同层 Body → Library 引用

## 能力边界

每个受管 Body 使用语义引用消费同一 refinement level 的 Library 身份；Markdown 导航链接
不自动成为治理依赖。

## 验收

- 缺失、未知、跨层或指向 Body 的引用分别诊断；
- 一个合法引用不能满足其他 Body 的缺失义务；
- canonical/localized 使用同一目标身份集合。

## 交付证据

`src/checks/wiki_references.rs`、reference collectors/normalization 和 governance package 测试。
