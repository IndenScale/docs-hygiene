---
id: FEATURE-027
epic: EPIC-008
status: baselined
delivery_status: delivered
addresses:
  - PRD-001-FR-001
  - PRD-001-FR-002
  - PRD-001-FR-003
  - PRD-001-FR-004
  - PRD-001-FR-006
depends_on: [FEATURE-015, FEATURE-018]
---

# FEATURE-027 开放工程资产模型迁移

## 能力边界

以 UL、PRD、Issue、Artifact 四类角色替代 Intent、Definition、Implementation 精化轴。
只有 UL 与 PRD 固化目录结构；Issue 可以位于本地归档或外部跟踪器；Artifact 包括代码、
测试、配置、生成物、SDK 内容与提交，不要求固定目录。

## 验收

- 配置、运行时模型和检查器不再包含 `refinementLevel`、`formalizes`、`realizes`、
  `projects` 或完整纵向派生要求；旧字段必须产生明确诊断；
- `docs/engineering/` 只保留 `ul/` 与 `prd/`，Definition 与固定实现 Manifest 被删除；
- PRD 不再内嵌变更级 acceptance 与 delivery status，本 Issue 能独立列出验收与证据；
- UL/PRD 语义引用、锁定、生命周期、影响与拓扑能力继续通过回归测试；
- 中英文入口、公开文档、示例配置和链接不再把三层精化模型描述为现行设计；
- 自举检查、完整测试、Markdown lint 与 diff 检查全部通过。

## 约束迁移

- 原 SPEC-001 的身份、引用、生命周期与确定性约束迁入 PRD-001、UL 契约与本 Issue；
- 原 SPEC-002 的规则激活约束由 PRD-003、FEATURE-009–011 与激活测试承接；
- 原 SPEC-003 的能力配置、图、锁定、快照、生命周期和人员治理约束由 PRD-004、
  FEATURE-017–026、公开治理文档及测试承接；
- 旧 Spec 不再作为独立权威保留，历史演化由 Git、ADR-0001 与已归档 Issue 追溯。

## 交付证据

- 决策：`docs/decisions/ADR-0001_open-engineering-asset-model.md`；
- 产品契约：`docs/engineering/ul/`、`docs/engineering/prd/prd-001/`；
- 运行时：`src/governance.rs`、`src/checks/governance_models.rs`、
  `src/checks/wiki_references.rs`、`src/activation/`；
- 配置与迁移：`docs-hygiene.yml`、`src/main/scaffold.rs`；
- 验证：101 个库单元测试及全部 CLI/集成/自举测试通过；`docs-hygiene check .
  --fail-on-warning`、Markdown lint 与 diff 检查通过。
