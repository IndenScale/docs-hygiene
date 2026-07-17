---
id: FEATURE-012
status: proposed
delivery_status: planned
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
