---
id: EPIC-002
status: baselined
delivery_status: delivered
depends_on: [EPIC-001]
---

# EPIC-002 内容锚颗粒度：File、Block、Repo

## 能力链

`File → Block → Repo`

颗粒度只有 `file | block | repo`。Git commit OID 是 Repo-wise Anchor 的实现机制，
不是第四种 scope。portable snapshot 中的 commit 仍是仓库状态 provenance。

## Features

1. [FEATURE-004 File-wise Anchor](../features/04_file-wise-anchor.md)
2. [FEATURE-005 Block-wise Anchor](../features/05_block-wise-anchor.md)
3. [FEATURE-006 Repo-wise Commit Anchor](../features/06_repo-wise-commit-anchor.md)
4. [FEATURE-007 Critical Pin Policy](../features/07_critical-pin-policy.md)
5. [FEATURE-008 Portable Snapshot](../features/08_portable-snapshot.md)

## Epic 验收

- scope 只表达内容范围，algorithm/provenance 表达实现和证据来源；
- File、Block、Repo 的覆盖关系和策略比较无歧义；
- Repo scope 校验完整 tracked repository state，而不是单个目标 blob。

File、Block、Repo、策略与快照均已交付；Repo scope 校验完整 tracked repository state。
