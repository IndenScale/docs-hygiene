# Domain 拓扑与 Fan-out

Docs Hygiene 把 UL 与 Glossary 建模为递归 Library Domain Tree。该结构显式表达语义
边界，不要求 checker 从自然语言正文推断含义。

## Domain

Domain 是 Library 内部的稳定语义边界。Domain 目录具有 `manifest.yml`，声明
`kind: domain`、稳定 `id`、生命周期 `status` 和非空直属 `members`。UL 或 Glossary
根 Manifest 是完整 Domain Tree 的权威，即使根节点不重复声明 `kind: domain`。

## Sub Domain

Sub Domain 是另一个 Domain 声明的直属子 Domain。它是同一种受治理节点，不形成新的
精化层级或引用关系。当术语簇、所有权或不同下游投影等仓库证据支持稳定边界时才引入。

## Fan-out 预算

Fan-out 只统计一个 Domain 的直属成员。每个 Markdown 叶子和子 Domain 各计为一个，
子 Domain 的后代不计入父级。检查只运行于 canonical UL 与 Glossary Tree；本地化表示
保持相同拓扑，但不重复产生诊断。

默认阈值包含边界值：

- 0–14 个直属成员：不产生诊断；
- 15–49 个直属成员：产生 warning 级 `DH_DOMAIN_001`；
- 50 个及以上直属成员：产生 error 级 `DH_DOMAIN_001`。

每个 Domain 只产生最高适用诊断。计数是结构可审议性信号，不证明某种语义分组。
工具与 Agent 可以提出候选 Sub Domain，但不能只根据数量移动成员或创造身份。

## 配置

```yaml
governance:
  domainFanout:
    warningAt: 15
    errorAt: 50
```

`warningAt` 至少为 1，`errorAt` 必须大于 `warningAt`。项目可以覆盖阈值以表达自己的
审议预算。刻意扁平的 Library 如果采用其他治理机制，可以关闭检查：

```yaml
rules:
  governance.domain-fanout:
    mode: disabled
```

Manifest 缺失、路径不安全、身份重复或未登记子项等结构错误仍由 `DH_LIBRARY_001`
独立执行。
