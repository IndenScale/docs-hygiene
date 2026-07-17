---
id: FEATURE-012
status: baselined
delivery_status: delivered
---

# FEATURE-012 关键依赖强制 Pin 与更新工作流

## 问题

现有 file/block/commit anchor 是可用机制，但项目无法声明“哪些依赖必须 Pin、必须使用何种
粒度、允许多久不更新”。因此依赖维度具备检测能力，却没有关键依赖治理闭环。

## 目标

- 按 source Kind、target Kind、edge relation、路径或身份集合声明 critical dependency。
- 策略可以要求 anchor 存在、算法、最小粒度以及是否禁止 whole-file Pin。
- Pin 漂移时同时报告直接依赖方和反向传递影响，区分内容变化与声明损坏。
- 提供只读更新计划和显式 apply；不得在检查过程中自动接受新指纹。
- 更新记录包含旧/新 digest、目标身份、selector、执行人和理由，便于审计。

## 验收标准

- 未 Pin、粒度不足、算法不允许和 hash 过期分别有稳定诊断。
- 更新工具支持 dry-run、原子写入和部分目标选择；任何无效目标阻止相关写入。
- 非关键引用保持无需 Pin，避免把所有依赖升级成高维护成本。
- 画像区分“支持锚定”和“关键依赖全部满足 Pin 策略”。

## 依赖

依赖 [FEATURE-003](03_multi-granularity-pin.md) 与
[FEATURE-002](02_reference-syntax-semantics-decoupling.md)。

## 交付证据

- `governance.criticalDependencies` 可按 source/target `body|library` Kind、规范化
  `references/formalizes/realizes/projects` relation、source/target path glob 与身份集合选择
  关键边；未匹配的普通引用不增加 Pin 成本；
- 策略声明允许算法、`minimumScope`、`forbidWholeFile` 与 `maxAgeDays`；file、commit、
  block 的最低强度顺序显式固定，block 或禁止 whole-file 时要求 SHA-256；
- `DH_PIN_001`–`006` 独立报告缺 Pin、scope 不足、算法不允许、内容变化、审计年龄过期
  及声明损坏；诊断同时列出直接依赖方、目标位置与反向传递影响；
- frontmatter anchor 可携带 `updatedAt`、`updatedBy`、`reason`，策略年龄门禁要求三项审计
  元数据完整；既有未使用新字段的非关键 anchor 保持兼容；
- `docs-hygiene update-pins` 默认只输出 `docs-hygiene.pin-update.v1`，支持重复
  `--policy` / `--target` 局部选择；只有 `--apply` 才整体写入文档与
  `governance.pinAuditLog` JSONL，批次带回滚保护；actor/reason、旧/新 digest、目标、
  selector 均被记录，既有不合规 Pin 可迁移到所需 shape；
- `src/checks/tests/critical_dependencies.rs` 覆盖稳定诊断、反向影响、年龄与非关键引用；
  `tests/pin_update.rs` 覆盖只读计划、显式 apply、审计、无效选择零写入、局部目标和
  `formalizes` 垂直边；画像新增 `dependency.critical-pins`。
- 完整契约见[关键依赖 Pin](../../16_critical_dependency_pins.md)与
  [SPEC-003 C-016](../../definition/spec/spec-003/constraints/critical-pin-policy.md)。
