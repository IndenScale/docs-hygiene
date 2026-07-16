# 产品需求

本目录保存可以建立基线的产品意图记录。需求说明 Docs Hygiene 应治理什么，以及
如何观察用户收益，但不充当实现任务清单。

## 权威边界

- 产品语言来自 [UL Registry](../../references/zh/01_ubiquitous_language.md)。
- 当前能力来自代码、配置、测试和规则说明。
- `proposed` PRD 只记录已经进入讨论范围的意图，不代表行为已经交付。

## 生命周期

`draft` → `review` → `baselined` → `superseded` → `archived`

被放弃的提案使用 `abandoned`，不强制成为规范语义基线。已基线化 PRD 必须固定
UL Registry 版本，并解决或显式延期所有局部概念和语义变更提案。

## 记录

- [PRD-001 意图契约治理](01_intent_contract_governance.md)
