---
id: FEATURE-019
epic: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [FEATURE-015, FEATURE-018]
---

# FEATURE-019 反向影响传播

## 能力边界

从所有已解析语义边计算去重、cycle-safe 的反向传递影响集合。

## 验收

- 循环不会导致无限遍历或重复影响；
- 输出顺序稳定并排除起点自身。

## 交付证据

`src/governance.rs` 及 governance graph tests。
