---
id: FEATURE-011
status: proposed
delivery_status: planned
---

# FEATURE-011 核心概念抽取与重复定义治理

## 问题

现有检查能证明 Body 引用了 Library，却不能证明核心概念只在 Library 定义。多个 Body
仍可能复制同一段定义并随时间漂移，从而绕过 SSOT 的真实目标。

## 目标

- 由维护者显式声明“核心概念集合”或受治理 claim，不从自然语言自动猜测权威概念。
- 每个核心概念指定唯一 Library 身份及可选 block selector，Body 只能引用或投影该权威。
- 为迁移提供候选重复扫描：输出证据和相似片段供人工确认，但不把相似度直接当错误。
- 对已确认的重复定义建立确定性规则，可选择禁止、限期迁移或标记为受控摘录。
- 受控摘录必须 Pin 到权威 block，并纳入传递影响。

## 验收标准

- 一个核心概念不能同时拥有两个 canonical Library 权威。
- Body 中已确认的重复定义、未 Pin 摘录和过期摘录均产生可定位诊断。
- 扫描建议与阻断规则分离，避免自然语言误报让 CI 不稳定。
- Library 权威迁移复用 `supersededBy`，所有消费方获得确定性 remediation。

## 依赖

依赖 [FEATURE-006](06_library-ssot-and-package-trees.md)、
[FEATURE-001](01_fine-grained-references.md) 和
[FEATURE-003](03_multi-granularity-pin.md)。
