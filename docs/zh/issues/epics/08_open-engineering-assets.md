---
id: EPIC-008
status: baselined
delivery_status: delivered
depends_on: [EPIC-005]
---

# EPIC-008 开放工程资产

## 能力链

`UL/PRD → Issue → Artifact`

UL 与 PRD 是长期开放工程文档；Issue 承载变更范围、验收与交付证据；Artifact 的位置和
载体由项目选择，不再映射到 Definition、Implementation 或 SDK 固定目录。

## Features

1. [FEATURE-027 开放工程资产模型迁移](../features/27_open-engineering-asset-model.md)

## Epic 验收

- 仓库只固化 UL 与 PRD 文档结构；
- Issue 可以独立承接原 Spec 的变更约束、验收和证据；
- 代码、测试、配置与 SDK 内容作为位置无关 Artifact 被引用；
- 旧精化字段被明确拒绝，避免静默形成双模型。

本 Epic 已由 FEATURE-027 完成交付。
