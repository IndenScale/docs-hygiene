---
id: SPEC-003-C-011
status: proposed
---

# C-011 标题 Selector 解析

语义 Wiki Link 可以在稳定目标身份之后追加可选标题 selector：

```text
[[ID#heading-slug]]
[[ID#heading-slug|label]]
[[ID#heading-slug@sha256:<64-hex>|label]]
```

selector 语法由一个或多个小写 ASCII 字母数字单词组成，单词之间使用单连字符。它在
canonical 受管目标的一至六级 ATX Markdown 标题中解析。标题 slug 将 ASCII 字母数字
转为小写，把中间连续的空白、标点或格式字符折叠为一个连字符，并移除首尾分隔符。
围栏代码不产生标题。被选择的 slug 必须只出现一次；重复标题 slug 属于歧义和无效输入。

规范化治理边独立保留 selector、关系类型和内容锚。因此 selector 可以与现有全文件
SHA-256 锚同时存在，但不会把该锚改成块级 hash。未提供 selector 时，文件级引用行为
完全不变。

显式 frontmatter 锚可以按照 [C-013](scoped-content-anchors.md) 使用 `scope: block`，
并复用 selector 作为 locator；内联 selector 加 hash 语法为保持兼容仍锁定整文件。

`DH_SELECTOR_001` 报告目标不可读或无法解析到 ATX 标题的 selector。诊断定位 Wiki Link
来源行，并关联 canonical 目标。canonical 与 localized Body 表示必须保持相同的目标身份、
selector 和可选内容 hash 签名有序集合。

至少一条规范化语义边携带 selector 时，治理级 `dependency.selector` 不变量才适用；
只有 `governance.identity` checker 没有 selector 诊断时才通过。disabled 或 suppression
执行仍不构成证据。行区间、自然语言片段和跨项目寻址不属于本约束；块级 hash 由
C-013 单独定义。
