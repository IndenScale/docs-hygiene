# Docs Hygiene Issue 归档

本目录是项目唯一的产品 Issue 存档。Issue 分为两种：Epic 定义一条端到端能力链，
Feature 定义可独立验收的原子能力。Git 保存实现历史，但产品依赖、交付状态和验收证据
必须保留在 Issue 中。

## 状态口径

- `delivered`：当前代码、配置和自动化测试已经证明全部验收条件；
- `partial`：已有实现，但缺少 Issue 声明的一等模型或完整验收；
- `planned`：能力边界已经确定，当前实现尚不能证明交付。

## Epic 主序

| Epic | 能力链 | 状态 |
| --- | --- | --- |
| [EPIC-001](epics/01_document-foundation.md) | 命名 → 内容 → 基础引用 | 已交付 |
| [EPIC-002](epics/02_anchor-granularity.md) | File → Block → Repo 内容锚 | 已交付 |
| [EPIC-003](epics/03_maturity-chain.md) | 不变量 → 成熟度 → 执行与迁移 | 已交付 |
| [EPIC-004](epics/04_localization-chain.md) | canonical → 结构同位 → 语义同位 | 已交付 |
| [EPIC-005](epics/05_reference-dependency-chain.md) | 同层引用 → 纵向依赖 → 影响传播 | 已交付 |
| [EPIC-006](epics/06_typed-reference-resolution.md) | occurrence → 期待类型 → 解析类型 → 校验 | 已交付 |
| [EPIC-007](epics/07_graph-community-governance.md) | 图 → 社区 → Fan-Out 与预算 | 已交付 |
| [EPIC-008](epics/08_open-engineering-assets.md) | UL/PRD → Issue → Artifact | 已交付 |

主序表达产品的理解和采用顺序。具体实现依赖以每个 Feature 的 `depends_on` 为准；不存在
为了压缩票据而跨 Epic 合并验收边界的“大 Feature”。

## Feature 总览

| Epic | Feature 范围 | Delivered / Partial / Planned |
| --- | --- | --- |
| EPIC-001 | FEATURE-001–003 | 3 / 0 / 0 |
| EPIC-002 | FEATURE-004–008 | 5 / 0 / 0 |
| EPIC-003 | FEATURE-009–011 | 3 / 0 / 0 |
| EPIC-004 | FEATURE-012–014 | 3 / 0 / 0 |
| EPIC-005 | FEATURE-015–019 | 5 / 0 / 0 |
| EPIC-006 | FEATURE-020–022 | 3 / 0 / 0 |
| EPIC-007 | FEATURE-023–026 | 4 / 0 / 0 |
| EPIC-008 | FEATURE-027 | 1 / 0 / 0 |

完整 Feature 清单及其依赖和证据由各 Epic 页面维护，避免在多个索引复制第二份状态。

当前共有 8 个 Epic、27 个 Feature；状态判断以每个 Feature 的验收与交付证据为准。
