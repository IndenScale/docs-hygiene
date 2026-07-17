---
id: FEATURE-013
status: proposed
delivery_status: planned
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
