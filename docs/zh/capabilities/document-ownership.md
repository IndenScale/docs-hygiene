# 文档责任、复核日落与知识冗余

责任治理把已建立身份变成有明确责任人、复核期限和知识冗余的资产。该能力需要显式启用：
配置 `governance.ownership` 后，所有 `baselined` 或 `current` 的受管资产、Package 领域和
Package 叶子都必须满足策略。

## 离线 Principal 目录

```yaml
governance:
  ownership:
    enabled: true
    confirmationMaxAgeDays: 365
    reviewWarningDays: 30
    resetAuditLog: .docs-hygiene/review-resets.jsonl
    principals:
      - id: person:alice
        kind: person
        status: active
      - id: person:bob
        kind: person
        status: active
      - id: group:platform-docs
        kind: group
        status: active
        members: [person:alice, person:bob]
```

Principal ID 是稳定引用，不是复制的显示名。person ID 以 `person:` 开头且没有成员；group
ID 以 `group:` 开头，并列出唯一、可直接解析的 person 成员。人员变为 `inactive` 后会
立即失去 Owner 和理解确认资格。

group 只有在 active 且能展开出 active person 时才能作为 Owner。理解确认必须由个人
给出；group 永远不能冒充一个或两个人。

## 身份元数据

每个已建立身份在自己的 YAML Manifest 或 Markdown frontmatter 中声明：

```yaml
ownership:
  owner: person:alice
  understoodBy:
    - principal: person:alice
      confirmedAt: 2026-07-01
    - principal: person:bob
      confirmedAt: 2026-07-02
review:
  reviewBy: 2026-10-01
  lastReset:
    at: 2026-07-17
    by: person:bob
    reason: quarterly semantic review
```

`owner` 必须解析到一个 active person 或已展开 group。`understoodBy` 至少包含两个不同的
active person，且 `confirmedAt` 不得在未来，也不得超过 `confirmationMaxAgeDays`。
Owner 可以占一席，但不能自证两席。

`reviewBy` 必须是有效的当前或未来日期。进入到期前 `reviewWarningDays` 窗口会产生 warning，
超过期限则产生 error。可选 `lastReset` 必须包含非未来日期、active person 执行人、非空
理由，且 `at` 不得晚于 `reviewBy`。

`draft`、`review` 和 `proposed` 可以提前准备元数据，但不被门禁。`archived`、`abandoned`
和 `superseded` 免除持续责任义务，既有生命周期和终止目标规则仍然生效。迁移后的
`baselined` 或 `current` 后继会独立校验，不能继承旧权威的陈旧确认。

## 显式 Review Reset

Reset 是人工语义复核动作，默认只读：

```bash
docs-hygiene reset-review TERM-RETRY \
  --actor person:bob \
  --reason "quarterly semantic review" \
  --review-by 2027-01-31
```

计划只报告一个精确目标且不写文件。加入 `--apply` 后，命令只更新该身份的
`review.reviewBy` 和 `review.lastReset`，并把同一类型化记录原子追加到 `resetAuditLog`。
actor 必须是唯一 active person，新期限必须位于未来并推进已有有效期限；目标歧义或任何
校验失败都保持零写入。

普通内容提交不会隐式续期。JSON 输出使用 `docs-hygiene.review-reset.v1`。

## 诊断与画像证据

- `DH_OWNERSHIP_001`：Principal 目录无效，或 Owner 缺失/不可解析；
- `DH_REVIEW_001`：复核证据缺失、无效或过期；
- `DH_REVIEW_002`：复核期限进入 warning 窗口；
- `DH_KNOWLEDGE_001`：当前有效 active-person 理解确认少于两人。

标准报告和 `docs-hygiene.profile.v1` 公开责任、复核和知识冗余的 covered/total/percentage、
即将到期与已过期数量，以及逐身份 knowledge bus factor。画像不变量
`identity.responsibility`、`identity.review-sunset`、`identity.knowledge-redundancy` 只接受
未 suppression 的检查证据；suppression 仍为 `unverified`，不构成 Passed。

交付契约见 [FEATURE-026](../issues/features/26_ownership-and-community.md)。
