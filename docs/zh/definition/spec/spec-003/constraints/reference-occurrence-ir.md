---
id: SPEC-003-C-012
status: baselined
---

# C-012 引用出现项 IR

引用声明语法与治理语义通过带版本的
`docs-hygiene.reference-occurrence.v1` 中间表示解耦：

```json
{
  "schemaVersion": "docs-hygiene.reference-occurrence.v1",
  "rawTarget": "TERM-1",
  "syntax": "wikiLink",
  "context": "governedContent",
  "location": { "path": "docs/body.md", "line": 12 },
  "payload": {
    "selector": "term",
    "anchor": { "algorithm": "sha256", "digest": "..." }
  }
}
```

`syntax` 与 `context` 是开放字符串标识，不是封闭枚举。collector 只识别一种
声明表面并产出 occurrence，不决定边关系；策略再把 `(syntax, context)` 映射为
处置结果：

| 语法 | 上下文 | 处置 |
| --- | --- | --- |
| `wikiLink` | `governedContent` | `semanticDependency` |
| `markdownLink` | `projectNavigation` | `navigationOnly` |
| `frontmatter` | `identityDeclaration` | `identityDeclaration` |
| `frontmatter` | `governedAnchor` | `semanticDependency` |

Wiki Link、Markdown Link 和 Markdown frontmatter collector 共享这一 IR。
frontmatter collector 把所声明的稳定 `id` 产出为身份声明，并把每个显式锚产出为受管
依赖。“仅导航”和“身份声明”均是显式策略结果，不进入治理图。多粒度锚 payload 遵循
[C-013](scoped-content-anchors.md)。

唯一的引用边规范化入口消费 occurrence 与策略。新增语法因此只需增加 collector
和策略条目，不必在规范化器中增加语法分支。未知 `(syntax, context)` 不产生语义边。
内建锚策略按照 C-013 支持 file/block scope 的 SHA-256，以及显式启用的 commit scope
Git 对象 ID。

该 IR 是内部扩展契约；字段含义或兼容性变化时必须升级 schema version。既有 file 锚
JSON 保持不变，多粒度锚按照 C-013 增加显式字段。规范化边继续遵循
[C-003](edge-normalization.md)；selector 解析遵循 [C-011](selector-resolution.md)。
