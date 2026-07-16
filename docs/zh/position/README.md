# 产品立场

本目录记录用于理解 Docs Hygiene 的产品模型。Position 文档解释指导术语和需求的
整体图景，但不替代 UL 目录、PRD、Glossary 目录、Spec 或实现证据。

## 文档

- [三个层次中的 Reference 与主体](01_reference_and_subject_across_three_layers.md)

## 变更路由

- 中文共享产品含义的表示进入 `docs/zh/intent/ul/` 下的对应术语文件，并与 canonical UL 保持身份和版本一致。
- 期望产品行为的变化进入 `docs/zh/intent/prd/` 下的原子成员。
- 形式定义进入 `docs/definition/`。
- 对解释模型的调整进入本目录。
- 对已交付行为的主张必须回到根目录代码、配置、测试和运行 Evidence 核验。
