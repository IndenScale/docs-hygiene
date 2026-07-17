---
id: FEATURE-002
status: baselined
delivery_status: delivered
---

# FEATURE-002 声明与语义解耦（统一引用模型）

## 背景

交付前，引用声明语法与语义是焊死的：Wiki Link 解析结果直接进入边规范化，
frontmatter 仅作为身份声明被临时消费；Markdown Link 被硬编码排除在语义边之外。
这个产品决策虽合理，却是“缺席”而非可审计策略，未来调整时没有统一挂点。

## 目标

把 Wiki Link、Markdown Link、frontmatter 收敛为纯前端语法：各自解析为
统一的引用中间表示（ReferenceOccurrence），由策略驱动的映射决定其语义，
为未来演进（新语法、新 relation、新消费方）创造条件。

- 定义中立 IR：`{ raw_target, syntax, location, payload }`，payload 预留
  selector 与 anchor 承载位。
- 每种语法一个 collector：wiki link、markdown link、frontmatter 各自产出
  同一 IR，不直接产出边。
- relation 映射策略化：由（syntax, 上下文）查表得出
  [治理边](../../../zh/intent/ul/governance/governance-edge.md)种类；
  "Markdown Link 非语义"从硬编码缺席变为显式策略条目。
- 引用边规范化器成为消费 IR 的唯一、语法无关入口。

## 范围

范围内：IR 定义、三个 collector、映射策略表、归一化泛化，且现有诊断
契约与报告格式保持不变（行为不变重构）。

范围外：新增 relation 种类、用户自定义语法插件、对 Markdown Link 语义化
的默认开启（策略仍默认其为非语义）。

## 依赖

与 [FEATURE-001](01_fine-grained-references.md) 正交，但 selector 应作为
IR payload 的一部分统一定义，建议两个 Feature 共享 IR schema 设计。
[FEATURE-003](03_multi-granularity-pin.md) 的多锚定声明依赖本 Feature
的 frontmatter collector。

## 验收标准

- 既有测试套件与 dogfood 检查全部保持绿色，证明行为不变。
- 新增一条假设语法时，仅需新增 collector 与策略条目即可接入（以测试
  演示，不要求真实第四语法落地）。
- IR 与映射表有版本化 schema 说明，JSON 报告中的边记录不变。

## 交付证据

- `src/reference.rs` 定义开放字符串标识的
  `docs-hygiene.reference-occurrence.v1` IR 与显式策略表；
- `src/checks/reference_collectors.rs` 让 Wiki Link、Markdown Link 和
  frontmatter collector 产出同一 IR；
- `src/checks/reference_normalization.rs` 是语法无关的唯一引用边规范化入口；
- `src/checks/repository_structure.rs` 的 Markdown 路径完整性检查复用同一个
  Markdown collector，不再维护第二份正则；
- `src/checks/tests/reference_ir.rs` 证明三种内建语法的策略处置，并以假设第四
  语法证明接入只需要 collector 输出与策略条目；
- [SPEC-003 C-012](../../../zh/definition/spec/spec-003/constraints/reference-occurrence-ir.md)
  固化版本、字段、扩展边界与公开边 JSON 不变约束。

## 参考

- [PRD-004 统一治理边（FR-001）](../../../zh/intent/prd/prd-004/requirements/unified-governance-edge.md)
- [ROADMAP 决策边界](../../../../ROADMAP.md)（Markdown Link 提供路径导航，
  不建立语义依赖）
