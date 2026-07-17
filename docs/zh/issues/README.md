# Docs Hygiene 治理 Feature 索引

本目录记录 Docs Hygiene 的产品 Feature。`delivery_status: delivered` 表示根据当前代码、
配置和测试逆向补录的已交付能力；`delivery_status: planned` 表示尚未交付，不能由设计文档
或局部机制推断为可执行保证。

## 治理理念落实矩阵

| # | 治理理念 | 当前判断 | 已交付票据 | 待交付票据 |
| --- | --- | --- | --- | --- |
| 1 | 文件命名 Schema 与 slug | 已落实 | [FEATURE-004](features/04_naming-and-kind-inference.md)、[FEATURE-009](features/09_slug-identity-governance.md) | — |
| 2 | Kind 驱动脚手架、frontmatter 与结构不变式 | 已落实 | [FEATURE-005](features/05_document-contracts-and-template-lifecycle.md)、[FEATURE-010](features/10_kind-aware-scaffolding-and-frontmatter-schema.md) | — |
| 3 | Library 作为核心概念 SSOT | 已落实 | [FEATURE-006](features/06_library-ssot-and-package-trees.md)、[FEATURE-011](features/11_library-ssot-extraction-and-duplication.md) | — |
| 4 | 关键依赖指纹与细粒度内容哈希 | 已落实 | [FEATURE-003](features/03_multi-granularity-pin.md)、[FEATURE-012](features/12_critical-dependency-pin-policy.md) | — |
| 5 | block/file/commit 细粒度引用 | 已落实 | [FEATURE-001](features/01_fine-grained-references.md)、[FEATURE-003](features/03_multi-granularity-pin.md)、[FEATURE-013](features/13_portable-commit-snapshots.md) | — |
| 6 | Typed Reference 引用对象 | 基本落实 | [FEATURE-002](features/02_reference-syntax-semantics-decoupling.md) | — |
| 7 | Graph Analysis 与 Fan-Out 预算 | 已落实 | [FEATURE-007](features/07_topology-analysis-and-budgets.md)、[FEATURE-014](features/14_supernode-governance-exceptions.md) | — |
| 8 | Owner、日落、Reset 与双人理解 | 初步落实 | [FEATURE-008](features/08_identity-lifecycle-and-authority-migration.md) | [FEATURE-015](features/15_document-ownership-and-sunset.md) |

## 结论

- 已落实：命名与 slug 身份闭环、Kind/Frontmatter、Library SSOT、关键依赖 Pin、可移植
  commit 快照与离线校验。
- 基本落实：Typed Reference。
- 初步落实：只有身份生命周期与权威迁移，尚未形成文档责任和知识冗余闭环。
- 当前最关键的缺口不是更多解析器，而是谁对文档负责、何时必须复核，以及如何证明至少
  两人理解。

## 证据口径

落实判断只接受当前 `main` 工作树中的实现、配置和自动化测试。PRD、SPEC、ROADMAP 中的
计划文字不单独构成交付证据。2026-07-17 复核时，`cargo test` 的 114 个测试全部通过，
其中包括 83 个单元测试、28 个 CLI/迁移/快照测试、2 个仓库 dogfood 测试和 1 个模板迁移测试。
