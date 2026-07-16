# Intent Layer

本目录保存 Intent Layer。它的 Reference Library 是版本化 `ul/` 递归领域树，每个
Markdown 叶子只定义一个稳定术语；受治理 Body 是 `prd/` 下的递归 PRD Package。

## 权威边界

- 每个 UL 领域 Manifest 定义身份、版本和直属成员集。
- `ul/` 下每个 Markdown 叶子只定义一个稳定产品术语。
- 每个 PRD Manifest 枚举原子角色、故事、需求和验收成员。
- PRD 固定 UL 版本，并引用所消费的术语。
- 已基线化 PRD 必须继续由 Definition Layer Spec 形式化。
- 当前能力仍以代码、配置、测试和 Evidence 为权威。

## 生命周期

Intent Body 使用 `draft` → `review` → `baselined` → `superseded` → `archived`。
被放弃的提案使用 `abandoned`，不成为规范基线。

## 资产

- [通用语言目录](ul/)
- [PRD-001 三层契约治理](prd/prd-001/index.md)
- [PRD-002 受治理语义链接与编辑器导航](prd/prd-002/index.md) — Backlog
