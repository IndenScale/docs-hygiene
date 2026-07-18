# 开放工程

本目录只包含开放工程的两类长期文档结构：通用语言（`ul/`）与产品需求（`prd/`）。

## 权威边界

- 每个 UL 领域 Manifest 定义身份和直属成员集。
- `ul/` 下每个 Markdown 叶子只定义一个稳定产品术语。
- 每个 PRD Manifest 枚举稳定的产品意图成员。
- PRD 使用语义 Wiki Link 引用所消费的 UL 术语。
- Issue 关联 PRD，并承载变更范围内的验收、协作与交付证据。
- 代码、测试、配置、SDK 内容、提交与生成物都是位置无关的 Artifact。

## 生命周期

PRD Body 使用 `draft` → `review` → `baselined` → `superseded` → `archived`。
被放弃的提案使用 `abandoned`，不成为规范基线。

## 资产

- [通用语言目录](ul/)
- [PRD-001 开放工程资产治理](prd/prd-001/index.md)
- [PRD-002 受治理语义链接与编辑器导航](prd/prd-002/index.md) — Wiki Link 已交付，编辑器导航待办
- [PRD-003 渐进式规则激活](prd/prd-003/index.md) — 已交付
- [PRD-004 多维文档治理](prd/prd-004/index.md) — 已交付并基线化
