---
id: FEATURE-014
status: proposed
delivery_status: planned
---

# FEATURE-014 超级节点与可审计拓扑例外

## 问题

全局 Fan-Out/Fan-In 阈值已能阻断，但合法公共概念也会超过预算。当前只能用通用
suppression 隐藏诊断，无法表达“这是经审议的超级节点”，也没有 owner 或到期复核。

## 目标

- 为指定稳定身份声明 supernode exception，字段至少包含方向、预算、理由、owner 和 expiry。
- 例外只放宽声明节点及方向，不允许 wildcard 绕过整个拓扑规则族。
- 匹配违规仍进入画像，标记为 excepted；例外不能证明不变量 Passed。
- 缺少理由/owner、已过期、目标不存在或未实际需要的例外均产生诊断。
- 报告例外节点的当前度数、预算余量、传递影响和最近趋势，为人工复核提供证据。

## 验收标准

- 普通节点超预算仍失败；有效超级节点例外不会阻断，但在报告中可见。
- expiry 到期后自动恢复失败，续期必须显式修改审计字段。
- 删除最后一条超额边后，闲置例外被提示清理。
- legacy suppression 保持兼容，但不能满足治理级拓扑成熟度。

## 依赖

依赖 [FEATURE-007](07_topology-analysis-and-budgets.md)。例外语义应落实
[SPEC-003 C-005](../../../zh/definition/spec/spec-003/constraints/exceptions.md)，而不是新增
第二套 suppression。
