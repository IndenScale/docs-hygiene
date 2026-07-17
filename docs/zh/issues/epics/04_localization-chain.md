---
id: EPIC-004
status: baselined
delivery_status: delivered
depends_on: [EPIC-001, EPIC-003]
---

# EPIC-004 本地化依赖链

## 能力链

`Canonical Authority → Path/Structure Parity → Identity/Lifecycle Parity → Reference Parity`

本地化文件是同一语义资产的表示，不产生独立治理身份或第二份权威。

## Features

1. [FEATURE-012 Canonical 权威与表示模型](../features/12_canonical-representation.md)
2. [FEATURE-013 路径与内容结构同位](../features/13_localized-structure-parity.md)
3. [FEATURE-014 身份、生命周期与引用同位](../features/14_localized-semantic-parity.md)

## Epic 验收

- canonical/localized 共享路径投影、稳定身份和 Package 结构；
- 本地化表示保持 lifecycle、successor、selector 和 anchor 签名；
- 图和 Fan-Out 不因语言数量重复计数。
