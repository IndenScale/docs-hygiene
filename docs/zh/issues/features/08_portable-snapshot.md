---
id: FEATURE-008
epic: EPIC-002
status: baselined
delivery_status: delivered
depends_on: [FEATURE-004, FEATURE-005]
---

# FEATURE-008 Portable Snapshot

## 能力边界

版本化、可签名 snapshot manifest 保存 repository identity、commit provenance、file/block
payload 和 digest，使外发证据可以离线校验。commit 是来源状态，不是 entry scope。

## 验收

- 原仓库不可访问时仍能验证已登记 payload；
- repository、commit、path、digest、签名和生命周期错误分别诊断；
- 导入默认只读，不 clone/fetch，显式 apply 原子写入。

## 交付证据

`src/portable_snapshot.rs`、`src/checks/portable_snapshots.rs`、
`src/snapshot_import.rs` 及 snapshot 测试。
