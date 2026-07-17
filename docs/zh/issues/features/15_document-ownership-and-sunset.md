---
id: FEATURE-015
status: baselined
delivery_status: delivered
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

## 已固化 Schema

```yaml
ownership:
  owner: person:alice
  understoodBy:
    - { principal: person:alice, confirmedAt: 2026-07-01 }
    - { principal: person:bob, confirmedAt: 2026-07-02 }
review:
  reviewBy: 2026-10-01
  lastReset:
    at: 2026-07-17
    by: person:bob
    reason: quarterly semantic review
```

Principal 目录、确认最大年龄与 Reset 审计路径由 `governance.ownership` 配置；完整已承诺
接口见下方交付证据引用的 C-019。

## 验收标准

- current/baselined 身份缺 Owner、少于两名有效理解者或 review 过期时产生确定性诊断。
- Owner 与理解者必须可解析且去重；群组不能在未展开成员时冒充两个人。
- Reset CLI 默认 dry-run，显式 apply 只更新目标身份并留下可审计证据。
- 生命周期迁移时，新权威必须重新满足 Owner、双人理解与 review deadline，不能继承陈旧确认。
- 画像分别报告责任覆盖率、到期风险和 knowledge bus factor，不把 suppression 视为通过。

## 依赖

依赖 [FEATURE-008](08_identity-lifecycle-and-authority-migration.md)。人员目录 Adapter 可以后置；
首版至少支持项目内稳定 person identity，确保离线检查确定性。

## 交付证据

- `governance.ownership` 固化离线 Principal 目录：person/group 使用稳定前缀、active/inactive
  状态，group 必须显式展开为唯一 person 成员；旧项目未配置时保持兼容；
- 每个 `baselined/current` 资产、Package 领域和叶子在自身 Manifest/frontmatter 声明
  `ownership.owner`、带 `confirmedAt` 的 `understoodBy` 和 `review.reviewBy`；Owner 必须可解析，
  理解确认只接受两个不同、active、未过期的 person；
- `DH_OWNERSHIP_001`、`DH_REVIEW_001/002`、`DH_KNOWLEDGE_001` 确定性区分责任、过期/临期
  复核和知识冗余故障。人员 inactive、确认过期或 group 确认会立即降低 bus factor；
- `reset-review` 默认 dry-run；显式 `--apply` 只更新一个精确身份的 deadline/`lastReset`，
  同时原子追加 JSONL 审计。actor、reason、未来且推进的 deadline 或目标校验失败时零写入；
- 迁移后 `baselined/current` 后继独立接受三项检查，不能继承前任证据；archived/abandoned/
  superseded 免除持续责任，但继续受既有 lifecycle 与终止目标约束；
- 标准 text/JSON 和 `docs-hygiene.profile.v1` 报告责任、当前复核、知识冗余 covered/total/
  percentage、临期/过期计数和逐身份 bus factor；三项治理级不变量均已 Delivered，suppression
  只能得到 `unverified`；
- `src/checks/tests/ownership.rs` 覆盖完整/缺失、人员离组、确认过期、group、临期/过期、
  终止豁免和迁移后继；`tests/review_reset.rs` 覆盖 dry-run、单目标 apply、审计与失败原子性；
  `src/profile/tests.rs` 覆盖覆盖率及 suppression 非证据。
- 完整接口见[文档责任、复核日落与知识冗余](../../19_document_ownership.md)及
  [SPEC-003 C-019](../../definition/spec/spec-003/constraints/ownership-review.md)。
