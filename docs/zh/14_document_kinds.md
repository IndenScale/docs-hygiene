# Document Kind 注册表

`documentKinds` 注册表让文档创建与校验消费同一份契约。每个 Kind 把已有 docs base、
文件名 pattern 与 Document Profile 绑定起来；Profile 继续拥有适用路径、语义 section ID、
开放正文、模板身份和模板 revision，Kind 在此基础上增加类型化 frontmatter 与确定性脚手架。

## 注册表

```yaml
documentKinds:
  - id: article
    base: articles
    pattern: article
    profile: article-profile
    scaffold:
      filename: "{slug}.md"
      title: "{identity}"
      sectionHeadings:
        context: { en: Context, zh: 上下文 }
        decision: { en: Decision, zh: 决策 }
    frontmatter:
      revision: 2
      compatibleFrom: 1
      revisionField: schemaRevision
      allowUnknownFields: false
      fields:
        - id: id
          type: string
          required: true
          source: identity
          format: "^ARTICLE-[0-9]+$"
        - id: slug
          type: string
          required: true
          source: slug
        - id: locale
          type: string
          required: true
          source: locale
          values: [en, zh]
        - id: status
          type: string
          required: true
          values: [draft, current, superseded]
          default: draft
        - id: priority
          type: integer
          default: 1
        - id: tags
          type: stringList
          default: [docs]
        - id: supersededBy
          type: string
      conditions:
        - when: { field: status, equals: superseded }
          required: [supersededBy]
```

绑定的文件名 pattern 必须声明相同 `documentKind`；绑定的 Profile 必须是每个受管文件
按配置顺序命中的第一个所有者。Profile 的 `requiredSections` 仍是开放契约，允许额外正文
章节；frontmatter 是否开放则由 `allowUnknownFields` 独立声明。

若 `docs.slugSchemas` 声明了相同 `documentKind`，注册表也会绑定该 slug Schema。其来源
捕获组或字段、稳定身份字段必须存在于 Kind 契约中；脚手架输入必须在写入前通过规范化
pattern、长度、保留名与重命名策略约束。

## 类型化 Frontmatter

字段类型包括 `string`、`integer`、`number`、`boolean` 和 `stringList`。`values` 声明
枚举，`format` 是字符串正则，`required` 控制必填，`default` 提供脚手架默认值。
`source` 可取 `input`、`identity`、`slug` 或 `locale`。条件可以在某字段等于指定值时
要求或禁止其他字段；`invariants` 用 `equals` 或 `notEquals` 比较两个字段。

`DH_KIND_001` 报告注册表绑定错误，`DH_KIND_002` 报告 Schema revision 缺失、兼容但
过期或不兼容，`DH_FRONTMATTER_001` 报告字段及跨字段违规。只有配置至少一个 Kind 时，
画像不变量 `structure.kind-schema` 才适用。

## Kind-aware Scaffold

```bash
docs-hygiene scaffold . \
  --kind article \
  --identity ARTICLE-42 \
  --slug cache-policy \
  --locale zh \
  --field priority=2
```

生成器在写入前解析 Kind 的 base、locale 根、文件名 pattern、Profile、Template、
frontmatter Schema 和 section ID。`--target` 只能覆盖为项目内相对目录，且最终路径仍须
命中 Profile。`--dry-run` 只打印路径与内容；除非显式传入 `--force`，已有文件一律拒绝
覆盖；非法输入不会产生部分文件。

## 原子迁移

Schema revision 窗口与 Template revision 窗口可以联合检查和迁移：

```bash
docs-hygiene migrate-kinds . --check --format json
docs-hygiene migrate-kinds .
```

版本化报告为 `docs-hygiene.kind-migration.v1`。兼容的文档 Schema revision 与 Profile
Template pin 一起推进；文档格式错误、字段非法、未来 revision 或低于 `compatibleFrom`
都会阻止全部文档和策略写入。未使用 Kind Schema 的项目仍可单独使用
`migrate-templates`。
