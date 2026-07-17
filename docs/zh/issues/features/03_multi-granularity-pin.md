---
id: FEATURE-003
status: baselined
delivery_status: delivered
---

# FEATURE-003 多颗粒度 Pin

## 背景

交付前，pin 只有 filewise 一档：`[[ID@sha256:<hash>]]` 对整个目标文件字节做
SHA-256，`ContentAnchor` 没有粒度字段。一个引用一次只能锚定一个目标文件的整体
内容，既会因目标文件无关变化产生噪声，也无法表达复数块、复数文档或特定 commit
等多个上游条件。

## 目标

把锚定从"语法内联的单档 hash"升级为与 relation 正交的、多颗粒度的
锚定声明，允许一个文件同时锚定多个上游条件。

- `ContentAnchor` 扩展为 `{algorithm, digest, scope, locator?}`，`scope` 取
  `file`（现状）、`block`、`commit` 三档。
- block 档：锚定目标文件的特定块，locator 复用
  [FEATURE-001](01_fine-grained-references.md) 的 selector；仅该块内容
  变化时触发过期信号。
- commit 档：锚定特定 commit hash，面向法律文档等强审计场景；作为
  opt-in 的 git 后端，默认关闭，需在[决策边界](../../../../ROADMAP.md)中
  明确其与"Git 仅为物理容器"立场的关系。
- 多锚定声明：允许单个文件声明复数锚定（多个块、多个文档），声明
  承载于 frontmatter，经 [FEATURE-002](02_reference-syntax-semantics-decoupling.md)
  的统一引用模型进入边记录。

## 范围

范围内：anchor schema 扩展、file/block 两档校验、frontmatter 多锚定声明、
commit 档的策略与 schema 定义及 opt-in 验证。

范围外：commit 档默认启用、跨仓库锚定、hash 自动迁移工具。

## 依赖

- [FEATURE-001](01_fine-grained-references.md)：block 档的 locator 即
  selector。
- [FEATURE-002](02_reference-syntax-semantics-decoupling.md)：多锚定声明
  需要 frontmatter collector 与统一 IR。

## 验收标准

- block 锚定在目标块变化时告警、在目标文件其他部分变化时不告警，均有
  fixture 覆盖。
- 单文件多锚定声明可解析、可校验，诊断逐锚定定位。
- commit 锚定在 opt-in 项目中可验证，未 opt-in 时保持现有行为。
- 输出保持确定性：锚定记录按稳定顺序进入 JSON 报告。

## 交付证据

- `src/governance.rs` 为 `ContentAnchor` 增加 `file`、`block`、`commit` scope
  与可选 locator；file scope 继续省略新增 JSON 字段，保持既有边兼容；
- `src/checks/reference_collectors.rs` 从 frontmatter `anchors` 列表逐项产出
  `frontmatter / governedAnchor` occurrence，并逐项定位无效声明；
- `src/checks/anchors.rs` 对整文件、精确 ATX 标题段和 opt-in Git commit blob
  分别校验；既有内联 selector 加 hash 仍锁定整文件；
- `governance.contentAnchors.verifyGitCommits` 默认关闭，未 opt-in 的 commit
  声明不调用 Git 并产生确定性错误；
- `dependency.scoped-anchor` 已进入画像不变量注册表；
- `src/checks/tests/anchors.rs` 证明块外变化不告警、块内变化告警、多锚稳定排序、
  逐项诊断、file JSON 兼容和 commit 开关/漂移行为；
- 完整约束见
  [SPEC-003 C-013](../../../zh/definition/spec/spec-003/constraints/scoped-content-anchors.md)。

## 参考

- [PRD-004 统一治理边（FR-001）](../../../zh/intent/prd/prd-004/requirements/unified-governance-edge.md)
- [ROADMAP 决策边界](../../../../ROADMAP.md)（可选内容 hash 锚定
  review-sensitive 依赖）
