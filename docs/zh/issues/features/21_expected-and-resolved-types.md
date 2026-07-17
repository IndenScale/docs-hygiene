---
id: FEATURE-021
epic: EPIC-006
status: baselined
delivery_status: partial
depends_on: [FEATURE-020]
---

# FEATURE-021 期待类型与解析类型

## 能力边界

每次语义引用解析都保留 relation、expected refinement/relation/Document Kind，以及实际
resolved endpoint type、位置和 lifecycle；未解析也产生显式 resolution result。

## 验收

- expectation 和 resolution 是可序列化的一等结构；
- 横向引用、纵向派生、claim 和 Pin 共用端点类型表达；
- unresolved、ambiguous 和 resolved 状态不由空字段隐式区分。

## 当前差距

现有代码会在具体 checker 中比较 expected refinement/relation，但 `GovernanceEdge` 没有
期待类型、解析类型或显式 resolution outcome，因此仅能证明部分交付。
