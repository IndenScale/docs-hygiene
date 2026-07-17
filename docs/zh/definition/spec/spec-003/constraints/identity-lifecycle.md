---
id: SPEC-003-C-015
status: baselined
---

# C-015 身份生命周期与权威迁移

每个受管资产、Package 领域和 Package 叶子都有一个稳定身份和一个生命周期状态。状态在
当前快照中承担以下义务：

| 类别 | 状态 | 权威含义 |
| --- | --- | --- |
| 演进中 | `draft`、`review`、`proposed` | 尚不能作为替代权威 |
| 已建立 | `baselined`、`current` | 可以作为权威和迁移后继 |
| 终止 | `superseded`、`archived`、`abandoned` | 不能再成为当前治理边的目标 |

`superseded` 身份必须声明且只能声明一个 `supersededBy` 稳定 ID；其他状态不得声明该
字段。后继必须存在、不同于旧身份、保持相同 `refinementLevel` 与 `referenceRelation`，
并处于 `baselined` 或 `current`。这些规则禁止自替代、悬空后继、跨维度替代，以及下一
权威尚未建立的替代链。

```yaml
id: TERM-OLD
status: superseded
supersededBy: TERM-NEW
```

canonical 与 localized Package 表示必须像保持 ID 和 status 一样保持 `supersededBy`。
关系图报告用有序 `authorityMigrations` 映射公开声明的替代关系。任何仍指向
`superseded`、`archived` 或 `abandoned` 身份的规范化边都会在消费方产生
`DH_GOVERNANCE_001`，并在存在后继时给出迁移目标。

这是当前状态一致性契约。历史转移发生时间仍由 Git 留证；检查器不会自动改写消费方、
删除历史身份，也不会从相似正文推断后继。只有这些快照义务和所有激活的身份检查均通过
且没有 suppression 时，治理级 lifecycle 与 authority-migration 不变量才通过。
已建立后继还必须独立满足 [C-019](ownership-review.md) 的责任、复核日落与知识冗余契约，
不得继承前任证据。
