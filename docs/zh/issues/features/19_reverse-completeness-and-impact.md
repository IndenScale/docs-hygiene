---
id: FEATURE-019
epic: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [FEATURE-015, FEATURE-016, FEATURE-018]
---

# FEATURE-019 反向完整性与影响传播

## 能力边界

对已建立资产验证必需反向关系，并从所有已解析语义边计算去重、cycle-safe 的反向传递
影响集合。

## 验收

- 缺失反向派生或投影可独立诊断；
- 循环不会导致无限遍历或重复影响；
- 输出顺序稳定并排除起点自身。

## 交付证据

`src/checks/derivation.rs`、`src/governance/impact.rs` 及 governance tests。
