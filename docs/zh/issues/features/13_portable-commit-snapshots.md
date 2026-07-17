---
id: FEATURE-013
status: baselined
delivery_status: delivered
---

# FEATURE-013 可移植 Commit 外发快照

## 问题

现有 commit anchor 只在本地 Git 仓库中读取同路径 blob，适合内部审计，但不能独立证明
一个外发快照、跨仓库依赖或已归档交付物的内容。commit-wise 能力因此只完成了本地档。

## 目标

- 定义外发快照对象：仓库/制品身份、commit OID、受管路径、内容 digest 和可选签名。
- 把 snapshot 作为类型化引用目标或 anchor provenance，而不是把 URL 当稳定身份。
- 默认离线校验已登记的本地 snapshot manifest；远程拉取必须显式执行并与检查解耦。
- 支持整文件和具名 block，保持与现有 file/block anchor 的规范化语义一致。
- 明确快照撤销、替换与保留策略，避免外部历史成为项目 SSOT。

## 验收标准

- 在原仓库不可访问时，已登记快照仍能通过 digest 验证。
- repository、commit、path 或 digest 任一不匹配均有确定性诊断。
- snapshot manifest 可签名、可版本化且不会泄漏凭据。
- 未启用外发快照的项目不增加网络访问或 Git 依赖。

## 依赖

基于 [FEATURE-003](03_multi-granularity-pin.md) 的 commit scope；定位为少见、显式启用的
强审计能力，不改变 Git 不是治理身份权威的现有边界。

## 交付证据

- `governance.portableSnapshots` 登记版本化 `docs-hygiene.snapshot.v1` manifest、受信
  Ed25519 公钥和签名强制策略；repository 只接受无凭据稳定身份，不接受 URL；
- manifest 绑定 snapshot 制品身份、完整 commit OID、原仓库 path、本地 payload、
  file/block scope、locator 与 digest；frontmatter anchor 通过类型化 `snapshot`
  provenance 引用这些证据，规范化边和 localized signature 均保留 provenance；
- `DH_SNAPSHOT_001`–`007` 分别报告登记/schema、repository、commit、path/payload、
  digest/shape、签名和生命周期故障；file 与 block 都用既有精确字节语义离线校验；
- `active/replaced/revoked`、`replacedBy` 和 `retainUntil` 明确撤销、替换与保留规则；只有
  active snapshot 能证明 anchor，外部历史不成为治理身份权威；
- `docs-hygiene import-snapshot` 默认输出只读 `docs-hygiene.snapshot-import.v1`，仅从用户
  显式提供的本地 Git checkout 读取精确 commit，验证完整计划后用 `--apply` 原子写入
  payload；不会 clone/fetch、修改签名 manifest 或保存 checkout 位置；
- `src/checks/tests/portable_snapshots.rs` 覆盖签名、file/block、四类 provenance 漂移和
  撤销；`tests/snapshot_import.rs` 证明导入后删除原仓库仍可离线通过；画像新增
  `dependency.portable-snapshot`。
- 完整契约见[可移植 Commit 快照](../../17_portable_snapshots.md)与
  [SPEC-003 C-017](../../definition/spec/spec-003/constraints/portable-commit-snapshot.md)。
