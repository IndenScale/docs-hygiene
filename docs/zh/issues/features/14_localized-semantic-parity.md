---
id: FEATURE-014
epic: EPIC-004
status: baselined
delivery_status: delivered
depends_on: [FEATURE-005, FEATURE-012, FEATURE-013]
---

# FEATURE-014 本地化身份、生命周期与引用同位

## 能力边界

canonical/localized 保持稳定 ID、slug authority、lifecycle、successor、目标身份、selector、
anchor 和 snapshot provenance 签名。

## 验收

- 任一语义签名漂移都定位到具体表示；
- 本地化 successor 不产生第二条权威迁移；
- 多个语言表示不放大 Fan-In/Fan-Out。

## 交付证据

reference normalization、package localization、lifecycle、slug checks 及 dogfood fixtures。
