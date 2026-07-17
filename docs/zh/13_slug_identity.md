# Slug 身份治理

文件名 pattern 约束名称形状；`docs.slugSchemas` 在此之上为选定 Document Kind 增加稳定、
可建立项目索引的 slug 身份契约。

## 配置

```yaml
docs:
  bases:
    - id: articles
      root: docs/articles
      localizedRoots:
        zh: docs/zh/articles
      patterns:
        - id: article
          regex: "^(?P<slug>[a-z0-9-]+)\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source:
        type: filename
        capture: slug
      pattern: "^[a-z][a-z0-9-]*$"
      minLength: 3
      maxLength: 64
      reserved: [admin, api]
      normalization: lowercaseKebab
      identityField: id
      aliasesField: aliases
      renamePolicy: stableIdentity
```

每个 Document Kind 只能有一个 Schema 和一个权威来源：`filename` 读取命中文件名 pattern
中的具名捕获组；`frontmatter` 读取指定字段；`stableId` 投影指定稳定 ID 字段。
`normalization` 可取 `none`、`lowercase` 或 `lowercaseKebab`；`pattern`、`minLength`、
`maxLength` 和 `reserved` 均作用于规范值。

## 身份索引

检查器为每个 Document Kind 建立确定性索引。主 slug 和 alias 共享命名空间，因此规范化
重复、大小写碰撞、保留字及 alias 冲突不能静默指向两个身份。共享稳定 ID 的 canonical
与 localized 文件必须公开同一规范 slug；以 frontmatter 或稳定 ID 为权威来源时，两者
仍可以使用不同文件名。

## 重命名生命周期

`renamePolicy: stableIdentity` 要求 `identityField`，将治理身份与路径分离。显式重命名迁移
期间，`requireAlias` 还要求 `aliasesField` 至少保留一个旧 slug；消费方迁移完成后回到
`stableIdentity`。`allowPathBreak` 表示明确放弃稳定路径身份。alias 与当前 slug 使用相同
规则校验并进入同一索引。

当 `docs.structure` 规则处于活动状态时，`DH_SLUG_001` 会阻断 CI。JSON `data` 包含
`originalValue`、`normalizedValue`、`documentKind`、可选 `conflictPath` 和可执行的
`remediation`。`docs.slugSchemas` 为空时不运行 slug 检查。
