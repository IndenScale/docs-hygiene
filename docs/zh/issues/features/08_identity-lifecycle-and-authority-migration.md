---
id: FEATURE-008
status: baselined
delivery_status: delivered
---

# FEATURE-008 身份生命周期与权威迁移

## 背景

Docs Hygiene 已有文档身份的生命周期状态和显式权威迁移，能够阻止消费方继续引用终止
身份。这是 Owner 与日落治理的前置能力，但本身不等于责任制或定期复核。

## 已交付范围

- 资产、Package domain 和叶子身份具有受控 lifecycle status。
- `superseded` 身份必须声明 `supersededBy`；其他状态不得错误声明后继。
- 后继必须保持 refinement level/reference relation，并处于 `baselined` 或 `current`。
- `superseded`、`archived`、`abandoned` 身份不能继续作为治理边目标。
- 画像以 `authorityMigrations` 有序公开权威迁移证据。

## 交付证据

- `src/checks/lifecycle.rs` 收集各级身份并校验状态与后继；
- `src/checks/tests/lifecycle.rs` 覆盖缺失后继、非法后继、叶子迁移和陈旧消费方；
- `src/governance.rs` 与画像输出保留 lifecycle provenance 和迁移映射；
- 本仓库 Manifest 使用 `draft`、`proposed`、`review`、`baselined`、`current` 等状态。

## 后续闭环

本票据只交付 lifecycle 与权威迁移。Owner、review deadline、手动 Reset 审计和双人知识
冗余已经由已交付的 [FEATURE-015](15_document-ownership-and-sunset.md) 在独立策略层闭环。
