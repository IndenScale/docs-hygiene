---
id: FEATURE-001
epic: EPIC-001
status: baselined
delivery_status: delivered
depends_on: []
---

# FEATURE-001 命名、Document Kind 与稳定身份

## 能力边界

文件名 pattern 与路径共同推导 Document Kind；slug、稳定 ID、alias、编号和重命名策略
形成一个确定性身份索引。本地化细节由 EPIC-004 验收。

## 验收

- 非法文件名、编号断裂、slug 冲突和 alias 冲突分别诊断；
- 重命名不会静默改变稳定身份；
- 检查器与脚手架消费同一规范化策略。

## 交付证据

`src/config.rs`、`src/config/slug.rs`、`src/checks/repository_structure.rs`、
`src/checks/slug_identities.rs` 及对应测试。
