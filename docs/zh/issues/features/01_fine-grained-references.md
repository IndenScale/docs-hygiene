---
id: FEATURE-001
status: baselined
delivery_status: delivered
---

# FEATURE-001 细粒度引用（块级）

## 背景

此前引用治理以文件为最小单位：Wiki Link `[[ID]]` 指向整个目标文件，
边记录中的 `selector` 字段虽已预留但当时恒为空。文件级粒度使引用方被迫依赖整个文档，
目标文件任何无关内容的变动都会产生同样强度的治理信号。

## 目标

在现有文件级引用之上增强块级引用，使边能够精确指向目标文档
内部的具名块，同时保持确定性与本地化对等。

- 定义 selector 寻址方案，第一档采用 heading slug（标题锚点），语法进入
  Wiki Link（如 `[[ID#section-slug]]`），不改变现有 `[[ID]]` 行为。
- `GovernanceEdge.selector` 落值并参与校验：目标文件中不存在对应 heading
  时产生确定性诊断。
- 本地化表示必须保持 canonical 的 selector 签名（与现有 Wiki Link 签名
  对等检查一致）。
- 文件级语义保持兼容：无 selector 的引用行为完全不变。

## 范围

范围内：heading slug 寻址、selector 解析与存在性校验、本地化对等、
诊断与 JSON 报告中的 selector 证据。

范围外：块级内容 hash 锚定（属于 [FEATURE-003](03_multi-granularity-pin.md)）、
行区间或自然语言片段匹配、跨仓库寻址。

## 依赖

无前置 Feature。本 Feature 是
[FEATURE-003](03_multi-granularity-pin.md) 的前置（block 级 pin 复用
selector 作为 locator）。统一引用模型的中间表示应预留 selector 承载位，
见 [FEATURE-002](02_reference-syntax-semantics-decoupling.md)。

## 验收标准

- 无效 selector（目标无对应 heading）产生确定性诊断，退出行为与现有
  引用错误一致。
- 本地化文件 selector 签名不一致产生对等性诊断。
- 既有测试套件全部保持绿色，新增 fixture 覆盖 selector 正、反例。

## 交付证据

- `src/checks/reference_collectors.rs` 解析 selector，
  `src/checks/reference_normalization.rs` 将其写入边并纳入本地化语义签名；
- `src/checks/selectors.rs` 实现确定性 ATX slug 寻址和 `DH_SELECTOR_001`；
- `src/checks/tests/selectors.rs` 覆盖正常解析、目标缺失和本地化漂移；
- PRD-004 的 canonical 与 localized index 共同 dogfood
  `#documentation-hygiene-profile`，画像证据为 Passed。

完整约束见 [SPEC-003 C-011](../../../zh/definition/spec/spec-003/constraints/selector-resolution.md)。

## 参考

- [PRD-004 统一治理边（FR-001）](../../../zh/intent/prd/prd-004/requirements/unified-governance-edge.md)
- [治理边（UL 术语）](../../../zh/intent/ul/governance/governance-edge.md)
- [治理关系图](../../../zh/07_governance_graph.md)
