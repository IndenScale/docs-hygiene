# Definition Layer

本目录保存 Definition Layer。它的 Reference Library 是版本化 `glossary/` 递归领域
树，每个 Markdown 叶子只定义一个精确身份；受治理 Body 是 `spec/` 下的递归 Spec
Package 与 Test Definition。

## 权威边界

- 每个 Glossary 领域 Manifest 声明直属成员集。
- `glossary/` 下每个 Markdown 叶子定义一个精确身份及其 UL 来源。
- 每个 Spec Manifest 枚举原子约束和验证成员。
- Spec 固定 Glossary 版本，并使用这些身份形式化来源 PRD。
- Test Case、模型、Oracle 和 Verifier 是 Definition Body。
- Test Result 和运行观察属于独立 Evidence 平面。

## 资产

- [三层治理 Glossary 目录](glossary/)
- [三层治理 Spec](spec/spec-001/index.md)
