---
id: FEATURE-021
epic: EPIC-006
status: baselined
delivery_status: delivered
depends_on: [FEATURE-020]
---

# FEATURE-021 期待类型与解析类型

## 能力边界

每次语义引用解析都保留 relation、expected reference relation/Document Kind，以及实际
resolved endpoint type、位置和 lifecycle；未解析也产生显式 resolution result。

## 验收

- expectation 和 resolution 是可序列化的一等结构；
- 语义引用、claim 和 Pin 共用端点类型表达；
- unresolved、ambiguous 和 resolved 状态不由空字段隐式区分。

## 交付证据

`ReferenceExpectation`、`ReferenceEndpoint`、`ReferenceResolution` 与
`ReferenceResolutionOutcome` 是 `GovernanceEdge` 的可序列化一等字段；Reference IR、
Manifest 与 Pin 共用该端点模型。governance/reference IR 测试覆盖 resolved、
unresolved、ambiguous、incompatible 及 Document Kind。
