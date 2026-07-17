---
id: FEATURE-005
epic: EPIC-002
status: baselined
delivery_status: delivered
depends_on: [FEATURE-004]
---

# FEATURE-005 Block-wise Anchor

## 能力边界

`scope: block` 使用唯一标题 selector 定位 ATX heading block，并只对该块的精确 UTF-8
字节计算 SHA-256。

## 验收

- locator 缺失、无目标或歧义分别诊断；
- 块外变化不使 block anchor 过期；
- block 边界由共享 Markdown 实现确定。

## 交付证据

`src/markdown.rs`、`src/checks/selectors.rs`、`src/checks/anchors.rs` 及 selector/anchor 测试。
