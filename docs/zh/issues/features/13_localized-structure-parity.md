---
id: FEATURE-013
epic: EPIC-004
status: baselined
delivery_status: delivered
depends_on: [FEATURE-002, FEATURE-012]
---

# FEATURE-013 本地化路径与内容结构同位

## 能力边界

localized tree 保持 canonical 的路径投影、文件名、编号、Package 直属成员、Document Kind、
frontmatter Schema 和稳定章节结构。

## 验收

- 路径、编号、成员集合和 Kind 漂移分别诊断；
- localized 标题可使用配置别名但保持 section ID；
- canonical/localized 使用同一内容契约。

## 交付证据

`src/checks/package_localization.rs`、repository structure/document contract checks 及本地化测试。
