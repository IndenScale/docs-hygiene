---
id: FEATURE-014
status: baselined
delivery_status: delivered
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

## 交付证据

- `governance.topology.exceptions` 以唯一 id 和精确 `(node, fanIn|fanOut)` 声明例外预算、
  reason、owner、`approvedBy`、expiry 与有序日期 degree history；schema 不提供 wildcard；
- 只有元数据有效、目标存在、当前度数超过全局预算但不超过例外预算且存在最近历史时，
  例外才是 `applied`；普通节点、相反方向、cycle 和例外预算超额继续阻断；
- `DH_TOPOLOGY_003`–`005` 分别报告无效/过期、闲置和历史问题；过期声明自动恢复底层
  `DH_TOPOLOGY_001`，删除最后一条超额边后产生清理 warning；
- 标准 text/JSON 与 profile 的 `topologyExceptions` 报告当前度数、全局/例外预算、带符号
  余量、最近 observation、趋势差、owner/approver/reason/expiry 及反向传递影响；
- 画像新增 `excepted` outcome 和 `exceptionIds`；有效例外不阻断普通 check，但不能证明
  `topology.thresholds` Passed。旧 suppression 仍为 `unverified`；
- `topology.budgets`、`topology.public-exceptions`、`topology.trends` 从 Missing 提升为
  Delivered；`src/checks/tests/topology.rs`、`src/profile/tests.rs` 和 `tests/topology_cli.rs` 覆盖方向
  隔离、过期、缺历史/元数据、闲置、标准报告与画像降级。
- 完整契约见[可审计超级节点例外](../../18_supernode_exceptions.md)、
  [SPEC-003 C-018](../../definition/spec/spec-003/constraints/supernode-exceptions.md)与更新后的
  [C-005](../../definition/spec/spec-003/constraints/exceptions.md)。
