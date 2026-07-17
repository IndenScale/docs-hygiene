---
id: SPEC-003-C-013
status: baselined
---

# C-013 多粒度内容锚

规范化内容锚包含 `algorithm`、`digest`、`scope` 和可选 `locator`。支持以下组合：

| Scope | Algorithm | Locator | 校验对象 |
| --- | --- | --- | --- |
| `file` | `sha256` | 禁止 | 完整 canonical 目标的精确字节 |
| `block` | `sha256` | 必需的标题 slug | canonical 中一个 ATX 标题段的精确 UTF-8 字节 |
| `commit` | `git` | 禁止 | 当前目标字节等于完整 Git commit OID 中的对应 blob |

`file` 是兼容默认值。JSON 会省略其 `scope` 和不存在的 `locator`，因此既有文件锚边记录
保持 schema 兼容。既有 `[[ID#selector@sha256:<digest>]]` 内联语法仍然是“选择标题，
锁定整文件”，绝不会静默变成 block scope。

多个显式锚通过 Markdown frontmatter 声明：

```yaml
anchors:
  - target: TERM-1
    algorithm: sha256
    digest: <64-hex>
    scope: block
    locator: normative-behavior
  - target: TERM-2
    algorithm: sha256
    digest: <64-hex>
    scope: file
  - target: TERM-3
    algorithm: git
    digest: <完整的-40-或-64-hex-commit-oid>
    scope: commit
```

每个列表项按照 [C-012](reference-occurrence-ir.md) 产出一个
`frontmatter` / `governedAnchor` occurrence，并形成一条独立、有序的锁定边。无效或
过期项在各自列表项行产生 `DH_REFERENCE_001`。canonical 与 localized 表示保持目标、
selector、algorithm、digest、scope 和 locator 签名一致。

block 从唯一解析的 ATX 标题行开始，到下一个同级或更高级 ATX 标题之前结束；没有后继
标题时延伸到文件末尾。哈希覆盖这一精确原始字节范围，因此范围外变化不会使锚过期。
标题寻址与歧义规则遵循 [C-011](selector-resolution.md)。

commit 校验默认关闭，必须显式配置：

```yaml
governance:
  contentAnchors:
    verifyGitCommits: true
```

启用后，检查器证明完整对象 ID 可解析为 commit，从本地仓库读取
`<commit>:<governed-target-path>`，并与当前 canonical 目标逐字节比较。未 opt-in 的
commit 锚直接报错且不会调用 Git。Git 仍只是物理审计证据；稳定治理 ID 与 canonical
内容仍是语义权威。跨仓库对象、默认启用 commit 锚和自动 digest 迁移不属于本约束。
