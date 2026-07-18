# 开放工程资产模型

状态：adopted

范围：内建软件文档 Profile

## 立场主张

Docs Hygiene 让长期产品语言与意图保持可检查，同时不把每种交付形态都变成文档层。
项目是受治理主体，显式选择的项目目录是执行边界，仓库拓扑仍由项目决定。

## 资产角色

| 角色 | 权威 | 要求长期目录 |
| --- | --- | --- |
| UL | 共享语言与长期约束 | 是 |
| PRD | 产品意图、边界与需求 | 是 |
| Issue | 变更范围、验收、协作与证据 | 否 |
| Artifact | 代码、测试、配置、SDK 内容或提交等实现与证据 | 否 |

UL 与 PRD 是开放工程文档。Issue 可以位于本地归档或外部跟踪器；Artifact 由适配器发现
或由 Issue 引用。Definition、Implementation、Glossary 与 SDK 不再是内建轴或固定目录。

## 引用关系

PRD Body 通过语义 `references` 边消费稳定 UL Library 身份。Issue 通过 `addresses`
关联 PRD 需求，可以 `dependsOn` 其他 Issue，并通过 `evidencedBy` 引用 Artifact。
关键可变依赖可以被锁定或快照化。导航用 Markdown 链接不建立这些关系。

## 语言表示

`en`、`zh`、`ja` 等语言代码标识表示形式；`canonical` 与 `localized` 是权威属性，
不是语言值。一个语义资产有一个 canonical 表示和零到多个 localized 表示，它们保留
同一身份与治理关系。

## 治理关系图

```text
UL ◀── references ── PRD ◀── addresses ── Issue ── evidencedBy ──▶ Artifact
                                      └── dependsOn ──▶ Issue
```

只有稳定身份、显式关系、生命周期与证据进入治理图。任何边都不隐含镜像目录或渐进精化层。

## 边界

当前检查验证已配置 Markdown 结构、稳定身份、生命周期、本地化同位、语义与锁定引用、
内容锚和图策略。Issue 与 Artifact 集成属于适配器边界。检查器不推断自然语言等价性，
不决定产品验收，也不规定代码、测试、配置或 SDK 内容的位置。
