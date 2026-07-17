---
id: EPIC-006
status: baselined
delivery_status: partial
depends_on: [EPIC-005]
---

# EPIC-006 类型化引用解析与校验

## 能力链

`ReferenceOccurrence → ReferenceExpectation → Resolution → ResolvedType → Compatibility`

relation 有枚举类型不等于端点已经类型化。本 Epic 要把期待类型、解析类型和兼容性结果
提升为一等模型，替代散落在具体 checker 中的比较。

## Features

1. [FEATURE-020 语法中立 Reference Occurrence](../features/20_reference-occurrence-ir.md)
2. [FEATURE-021 期待类型与解析类型](../features/21_expected-and-resolved-types.md)
3. [FEATURE-022 统一兼容性校验](../features/22_reference-compatibility-validation.md)

## Epic 验收

- collector 不决定治理语义；
- 每次解析保留 expected 与 resolved endpoint type；
- 缺失、歧义、错层、错关系、错 Kind 和终止目标使用统一结果诊断。

当前 occurrence IR 已交付，端点类型仍由具体 checker 临时比较，因此 Epic 为部分交付。
