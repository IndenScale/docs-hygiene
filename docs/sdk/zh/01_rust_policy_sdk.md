---
id: SDK-001
status: current
glossary: GLOSSARY-001
glossary_version: 0.1.0
crate_version: 0.1.0
---

# Rust Policy SDK

## 定义来源

SDK-001 把 GLOSSARY-001 身份投影为可复用 Rust 策略原语。Manifest 关系图验证
尚未进入当前公共表面。

## 公共表面

`docs_hygiene` Rust Library 导出 `Config`、`run_checks`、`Report`、
`print_text_report` 和 `print_json_report`。CLI 消费该表面，不再拥有独立检查实现；
仓库 Implementation Body 在 `implementation-manifest.yml` 中声明该依赖。

## 语义映射

| 定义身份 | SDK 符号 |
| --- | --- |
| 受管策略输入 | `Config` |
| 策略求值 | `run_checks` |
| 诊断结果记录 | `Report` |
| 人类可读 Evidence Adapter | `print_text_report` |
| 机器可读 Evidence Adapter | `print_json_report` |

这些映射覆盖已有结构检查和文档契约，不声称已经实现跨层 Manifest 或追溯关系图。

## 证据

`cargo test` 验证共享 Library 模块和 CLI 集成。针对本仓库运行
`cargo run -- check --fail-on-warning` 验证配置的 UL、PRD、Glossary、Spec 和 SDK
Reference 文档契约。
