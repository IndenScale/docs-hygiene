---
id: FEATURE-004
epic: EPIC-002
status: baselined
delivery_status: delivered
depends_on: [FEATURE-003]
---

# FEATURE-004 File-wise Anchor

## 能力边界

`scope: file` 使用 SHA-256 锁定单个 canonical 目标文件的精确字节，并保持既有内联
Wiki Link hash 兼容。

## 验收

- 文件内容变化产生稳定的 expected/actual digest 诊断；
- selector 与 file hash 并存时仍锁定整文件；
- JSON 省略默认 file scope 时保持兼容。

## 交付证据

`src/checks/anchors.rs`、`src/checks/reference_collectors.rs` 与 anchor 测试。
