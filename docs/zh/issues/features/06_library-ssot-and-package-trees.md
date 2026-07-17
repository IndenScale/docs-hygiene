---
id: FEATURE-006
status: baselined
delivery_status: delivered
---

# FEATURE-006 Library SSOT 与递归 Package Tree

## 背景

Docs Hygiene 已把共享含义从 Body 主张中抽离为同精化层级的 Library，并使用稳定身份、
显式成员树和语义引用建立 SSOT。本票据逆向记录现有实现。

## 已交付范围

- Intent/Definition/Implementation 分别具有 UL、Glossary、SDK Library。
- Library manifest 声明稳定 ID、精化层级、引用关系、生命周期和直属成员。
- Library 支持任意深度的 domain tree；每个叶子是带 frontmatter 身份的 Markdown 文件。
- Body 必须语义引用同层 Library；下游 Library 必须投影相邻上游 Library。
- 身份重复、路径穿越、遗漏成员、孤立概念、非法跨层引用与投影缺失均可诊断。
- canonical 与 localized 表示保持路径、身份和引用签名同位。

## 交付证据

- `src/checks/package_structure.rs` 与 `src/checks/package_localization.rs` 校验递归树和表示同位；
- `src/checks/wiki_references.rs` 校验 Body 到 Library 的横向引用；
- `src/checks/derivation.rs` 校验 Library projection 与反向完整性；
- `src/checks/tests/governance_packages.rs` 和 `tests/dogfood.rs` 覆盖正反例；
- `docs/intent/ul/`、`docs/definition/glossary/` 与 `sdk-manifest.yml` 在本仓库 dogfood。

## 后续闭环

本票据只证明“引用了 Library”。显式核心 claim 权威、重复策略、候选扫描与抽取迁移输入
已经由已交付的 [FEATURE-011](11_library-ssot-extraction-and-duplication.md) 闭环；系统仍不从
自然语言相似度直接生成阻断结论。
