---
id: FEATURE-011
epic: EPIC-003
status: baselined
delivery_status: delivered
depends_on: [FEATURE-009, FEATURE-010]
---

# FEATURE-011 执行状态、例外与兼容迁移

## 能力边界

inactive/advisory/warning/error 与成熟度分离；disabled、suppression 和 exception 的证据
资格显式；旧成熟度和规则模式确定性迁移。

## 验收

- 严重程度不会改变符合性结果；
- suppression 和未执行不能证明通过；
- 旧配置映射冲突成为配置错误而非静默覆盖。

## 交付证据

`src/activation.rs`、`src/profile.rs`、`src/profile/configuration.rs`、兼容性约束和相关测试。
