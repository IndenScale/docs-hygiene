# Library Claim 治理

`governance.coreClaims` 把选定的可复用含义显式提升为 Library SSOT。维护者声明受管 claim
及其唯一 canonical Library 身份；Docs Hygiene 不从自然语言推断权威或阻断性重复。

## Claim 注册表

```yaml
governance:
  manifests:
    - docs/intent/ul/manifest.yml
    - docs/intent/prd/prd-001/manifest.yml
  coreClaims:
    - id: retry-policy
      authority:
        id: RETRY-POLICY
        selector: canonical-contract
      candidatePaths:
        - docs/intent/prd/**/*.md
      similarityThreshold: 0.72
      occurrences:
        - path: docs/intent/prd/prd-001/requirements/retries.md
          selector: copied-contract
          policy: forbidden
        - path: docs/intent/prd/prd-002/migration.md
          selector: legacy-contract
          policy: migrate
          migrateBy: 2026-09-30
        - path: docs/intent/prd/prd-003/summary.md
          selector: approved-excerpt
          policy: controlledExcerpt
```

claim ID 必须唯一；`authority.id` 必须解析到一个受管 Library 身份，可选
`authority.selector` 必须唯一解析。权威进入 superseded 状态时，诊断复用其
`supersededBy` 给出替换目标。已确认 occurrence 必须是受管 Body member，且 heading
selector 必须唯一解析。

## 已确认重复策略

- `forbidden` 始终阻断，并要求 Body 改为引用 canonical 身份；
- `migrate` 要求 `migrateBy: YYYY-MM-DD`，该 UTC 日期内为 warning，之后为 error；
- `controlledExcerpt` 只允许显式评审且锁定权威 block 的摘录，不得声明 `migrateBy`。

门禁只消费配置中已确认的 occurrence。其他相似正文不会成为诊断，除非维护者将其登记
进注册表。

## 受控摘录

受控摘录复用 scoped anchor 契约：

```yaml
---
id: PRD-RETRY-SUMMARY
status: proposed
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <canonical heading block 的 64-hex SHA-256>
    scope: block
    locator: canonical-contract
---
```

anchor target 和 locator 必须与 claim 权威一致。缺 Pin 或 Pin 过期会在 Body occurrence
或 anchor 声明处产生 `DH_CLAIM_001`。同一声明同时形成 `pinnedReference` 治理边，因此
既有 anchor 校验、生命周期、Fan 分析与反向传递影响继续生效。

## 候选扫描

```bash
docs-hygiene scan-library-claims . --format json
```

只读命令把权威 block 与该 claim 的显式 `candidatePaths` 命中的 heading block 比较，
使用确定性的规范化词集合 Jaccard 分数及 `0..=1` 的 `similarityThreshold`。版本化报告
`docs-hygiene.library-claim-scan.v1` 包含权威与未确认候选的位置、selector、行号、分数
和短证据片段；已登记 occurrence 不再重复报告。

候选只是建议数据：发现候选不会让命令失败，也不会改变 `docs-hygiene check`。维护者先
审阅证据，只有确认文本确属重复定义或受控摘录后，才添加 occurrence。

## 诊断与迁移

`DH_CLAIM_001` 覆盖注册表歧义、无法解析或终止状态权威、非法 occurrence 位置、禁止
重复、迁移逾期，以及受控摘录缺 Pin / Pin 过期。`DH_REFERENCE_001` 继续治理底层语义
边或锁定边。两者都属于 `governance.identity`；配置至少一个 core claim 后，可选画像
不变量 `identity.library-claims` 生效。

权威替换复用既有 Library 生命周期：保留旧身份并设置 `status: superseded`，用
`supersededBy` 指向 baselined 或 current 的 Library 身份，再把 claim 权威和 Body 依赖
迁移到诊断给出的后继。
