---
id: FEATURE-015
status: proposed
delivery_status: planned
---

# FEATURE-015 文档 Owner、日落 Reset 与双人知识冗余

## 问题

文档已有 identity lifecycle，但没有“谁负责、何时失效、谁真正理解”的可执行保证。
`status: current` 可以无限期保持，单点知识风险也不可见。

## 目标

- 每个治理身份声明主 Owner 和至少一名 Knowledge Reviewer；引用组织身份，不复制显示名。
- 声明 `reviewBy` 或基于策略计算的 sunset deadline；到期后从 warning 递进为 error。
- Reset 必须是手动审议动作，记录时间、执行人、依据和新的 deadline；普通内容提交不能隐式续期。
- 至少两名不同的有效人员确认理解文档含义，Owner 可以占一席，但不能自证两席。
- 人员离组、身份失效或确认过期时，知识冗余立即降级并触发 remediation。
- archived/abandoned 文档免除持续 Owner 义务，但必须满足保留与替代权威规则。

## 建议 Schema

```yaml
ownership:
  owner: person:alice
  understoodBy: [person:alice, person:bob]
review:
  reviewBy: 2026-10-01
  lastReset:
    at: 2026-07-17
    by: person:bob
    reason: quarterly semantic review
```

具体字段名应在实现前通过 Definition 固化；上例只表达不变量，不是已承诺接口。

## 验收标准

- current/baselined 身份缺 Owner、少于两名有效理解者或 review 过期时产生确定性诊断。
- Owner 与理解者必须可解析且去重；群组不能在未展开成员时冒充两个人。
- Reset CLI 默认 dry-run，显式 apply 只更新目标身份并留下可审计证据。
- 生命周期迁移时，新权威必须重新满足 Owner、双人理解与 review deadline，不能继承陈旧确认。
- 画像分别报告责任覆盖率、到期风险和 knowledge bus factor，不把 suppression 视为通过。

## 依赖

依赖 [FEATURE-008](08_identity-lifecycle-and-authority-migration.md)。人员目录 Adapter 可以后置；
首版至少支持项目内稳定 person identity，确保离线检查确定性。
