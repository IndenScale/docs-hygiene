---
id: SPEC-003-C-009
status: proposed
---

# C-009 文档模板注册表

`documentContracts.templates` 是可复用契约片段的有序注册表。每个模板和 Profile 都有
稳定身份，只允许 ASCII 字母、数字、`.`、`_` 和 `-`。Profile 通过 `template` 最多绑定
一个模板。

解析过程必须确定。模板中的章节、字段和占位符表达式排列在 Profile 局部增量之前；
Profile 局部标量覆盖模板标量，未提供时依次采用模板值和兼容默认值。解析后的章节或
字段身份重复属于无效配置，不能解释为隐式覆盖。

`DH_TEMPLATE_001` 报告无效身份、重复声明、未知绑定、无效表达式和解析成员冲突；
`DH_TEMPLATE_002` 报告没有 Profile 绑定的模板。旧内联 Profile 仍然有效并继续执行，
但不能证明模板复用覆盖率。

只有注册表有效、至少存在一个模板和一个 Profile、每个 Profile 都已绑定且每个模板都
被使用时，`structure.reusable-templates` 才达到受控等级。带版本的画像报告公开数量、
绑定、未模板化 Profile、未使用模板和注册表有效性；suppression 仍不构成通过证据。

稳定绑定不等于模板生命周期治理。revision 窗口、Profile pin 和原子迁移属于
[C-010](template-lifecycle.md)定义的独立治理级 `structure.template-migration` 不变量。
