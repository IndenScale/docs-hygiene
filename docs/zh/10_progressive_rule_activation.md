# 渐进式规则激活

Docs Hygiene 从一份确定性的项目事实快照派生规则族决策，使治理要求随已证明的需求
出现，同时保持显式项目策略的最终控制权。

## 决策管线

```text
项目文件与策略
  → 项目事实
  → 激活决策
  → checker 诊断
```

事实包括 Markdown 文档数与行数、代码行数、本地化文档、概念文档、Manifest、
frontmatter、语义 Wiki Link、已配置文档 Profile、受治理精化层级和已启用 Adapter。
所有计数都先应用 ignore 策略。

## 稳定规则族

| 规则 ID | 治理范围 |
| --- | --- |
| `project.entry-docs` | 项目必需入口文档 |
| `docs.structure` | 命名、编号、链接、长度与结构策略 |
| `documents.contracts` | 基于路径推导的语义文档契约 |
| `localization.parity` | 语言阈值与表示同位 |
| `concepts.references` | 概念外键与孤立概念 |
| `governance.identity` | 身份、Package 与语义引用 |
| `governance.domain-fanout` | canonical Library Domain 直属成员预算 |
| `governance.traceability` | 相邻层级派生与投影 |
| `adapters.external` | 外部 Adapter 执行 |

每项决策包含 `mode`、状态、有序证据、价值理由和修复方向。状态依次是 `inactive`、
`advisory`、`warning` 和 `error`。

## 自动激活

`auto` 是默认模式。显式功能配置或结构存在性可以激活对应规则族。纯规模信号当前从
20 份 Markdown 文档或 20,000 行代码开始，最高只产生 `advisory`，不会意外形成 CI
阻断。本地化文档或跨多个精化层级的 Manifest 等结构信号，即使尚未配置完整策略，
也可以产生 `warning`。

同一事实模型中的证据保持单调：新增一个已满足信号只能维持或增强决策。Docs Hygiene
不持久化隐藏成熟度，也不修改策略。需要规则永久保持阻断的团队把它固定为
`required`。

## 显式权威

逐规则策略覆盖自动推导：

```yaml
rules:
  governance.traceability:
    mode: auto
  localization.parity:
    mode: required
  adapters.external:
    mode: disabled
```

- `auto` 从项目事实派生状态；
- `required` 不受推导事实影响，选择 `error`；
- `disabled` 不受推导事实影响，选择 `inactive`，并阻止 checker 或外部进程产生诊断。

未知规则 ID 和未知模式值都是配置错误。

## 可解释性

无需运行检查即可查看当前决策：

```bash
docs-hygiene explain-rules
docs-hygiene explain-rules --format json
```

文本输出面向人类；JSON 使用 `docs-hygiene.rule-activation.v1` Schema，包含完整事实
快照和有序决策，可供 CI、编辑器或治理工具消费。

未配置的结构或规模信号激活规则族时，`check` 会产生 `DH_ACTIVATION_001`，说明证据
和覆盖路径。advisory 决策把派生诊断限制为 Info，warning 限制为 Warning，error
保留 checker 已配置的严重程度语义。

## 边界

首版不自动发现 monorepo 项目，不从正文推断业务风险，不持久化迟滞状态，也不虚构
缺失的策略参数。每个显式指定的项目根目录分别获得自己的事实快照和规则决策。
