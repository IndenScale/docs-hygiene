# Intent Refinement Level

本目录保存 Intent Refinement Level。它的 Library 是以身份治理的 `ul/` 递归领域树，每个
Markdown 叶子只定义一个稳定术语；受治理 Body 是 `prd/` 下的递归 PRD Package。

## 权威边界

- 每个 UL 领域 Manifest 定义身份和直属成员集。
- `ul/` 下每个 Markdown 叶子只定义一个稳定产品术语。
- 每个 PRD Manifest 枚举原子角色、故事、需求和验收成员。
- PRD 使用语义 Wiki Link 引用所消费的 UL 术语。
- 已基线化 PRD 必须继续由 Definition Refinement Level Spec 形式化。
- 已交付能力仍以代码、配置和测试为准。

## 生命周期

Intent Body 使用 `draft` → `review` → `baselined` → `superseded` → `archived`。
被放弃的提案使用 `abandoned`，不成为规范基线。

## 资产

- [通用语言目录](ul/)
- [PRD-001 三维契约治理](prd/prd-001/index.md)
- [PRD-002 受治理语义链接与编辑器导航](prd/prd-002/index.md) — Wiki Link 已交付，编辑器导航待办
- [PRD-003 渐进式规则激活](prd/prd-003/index.md) — 已交付
- [PRD-004 多维文档治理](prd/prd-004/index.md) — 推进中；画像、模板、规范化边、多粒度锚、传递影响、生命周期、权威迁移和拓扑策略已交付
