---
id: FEATURE-006
epic: EPIC-002
status: baselined
delivery_status: planned
depends_on: [FEATURE-004, FEATURE-005]
---

# FEATURE-006 Repo-wise Commit Anchor

## 能力边界

颗粒度使用 `scope: repo`；实现机制使用 `algorithm: git` 和完整 commit OID。该 OID 表示
仓库全部 tracked state，不是独立 scope，也不是单个目标文件的版本号。

## 验收

- Schema 不再接受 `scope: commit`；
- repo anchor 验证完整 commit 可解析，并比较完整 tracked repository state；
- 任一 tracked path 的新增、删除、模式或内容变化都会使 repo anchor 过期；
- untracked 文件不影响结果；
- 报告明确区分 `scope=repo`、`algorithm=git` 和 commit OID。

## 当前差距

当前 `ContentAnchorScope::Commit` 仍存在，校验只读取 `<commit>:<target-path>` blob，
不能证明 Repo-wise 语义，因此不得标记为 delivered。
