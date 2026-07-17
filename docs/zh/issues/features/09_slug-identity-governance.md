---
id: FEATURE-009
status: baselined
delivery_status: delivered
---

# FEATURE-009 文件 slug 身份治理闭环

## 问题

现有文件名 pattern 只能证明名称符合正则；标题 selector slug 只用于块寻址。系统尚不能
证明文件 slug、稳定 ID、标题与路径之间的一致关系，也不能治理重命名。

## 目标

- 为需要 slug 的 Document Kind 声明 slug Schema：字符集、长度、保留字和规范化算法。
- 明确 slug 来源，可选择 frontmatter 字段、文件名捕获组或稳定 ID 投影，但同一 Kind
  只能有一个权威来源。
- 建立项目范围 slug 索引，检测规范化后冲突、大小写碰撞和本地化表示漂移。
- 重命名时要求显式 alias/redirect 或身份迁移，避免路径链接静默断裂。
- JSON 诊断输出原值、规范值、Kind、冲突对象和可执行 remediation。

## 验收标准

- 同一 Kind 下非法、重复、大小写碰撞和保留 slug 均有确定性 fixture。
- canonical/localized 表示共享同一治理身份，不强制译文文件名采用翻译 slug。
- 未启用 slug Schema 的现有项目保持兼容；启用后可进入 CI 门禁。
- 文件重命名不会改变稳定治理 ID，旧路径如何处理由显式策略决定。

## 依赖

依赖 [FEATURE-004](04_naming-and-kind-inference.md) 的 pattern 与 Kind 推导；与
[FEATURE-010](10_kind-aware-scaffolding-and-frontmatter-schema.md) 共享 Kind Schema。

## 交付证据

- `src/config/slug.rs` 定义按 `documentKind` 唯一绑定的 slug Schema、三类权威来源、
  三种规范化算法和显式重命名策略；未配置 Schema 时不改变既有行为；
- `src/checks/slug_identities.rs` 建立项目级 slug/alias 索引，检查字符集、长度、保留字、
  规范化重复、大小写碰撞、alias 冲突和共享稳定 ID 的语言表示漂移；
- `renamePolicy: stableIdentity` 要求稳定身份，`requireAlias` 支持重命名迁移门禁，
  `allowPathBreak` 作为明确的兼容性逃生口；
- `DH_SLUG_001` 的版本化 JSON 诊断 `data` 包含原值、规范值、Document Kind、冲突路径
  和 remediation；
- `identity.slug-schema` 进入多维画像不变量注册表，只有配置 slug Schema 时才适用；
- `src/checks/tests/slug_identities.rs` 与 `tests/cli.rs` 覆盖非法、保留、重复、大小写、
  alias、本地化漂移、三类来源、兼容性和 JSON 输出。
