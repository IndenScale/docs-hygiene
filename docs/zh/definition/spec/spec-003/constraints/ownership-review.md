---
id: SPEC-003-C-019
status: baselined
---

# C-019 责任、复核日落与知识冗余

启用责任治理后，每个 `baselined` 或 `current` 受管资产、Package 领域和 Package 叶子都要
声明一个可解析 active Owner、当前复核期限，以及至少两个不同 active person 的理解确认。
Principal 身份是项目内稳定、离线的策略记录；person 与 group 使用不同前缀，group 列出
唯一且直接的 person 成员。未展开 group 不能满足责任要求，任何 group 都不能作为个人理解证据。

每条确认记录 person principal 和日期。未来、陈旧、重复、不可解析、group 或 inactive
person 的确认不计数。Owner 只有通过自己的有效个人确认才能占一席。复核期限进入配置的
临近窗口时产生 warning，过期后产生 error。

Reset 是精确身份的显式操作，默认 dry-run。apply 只写选中身份的新期限，以及
`lastReset` 时间、active person 执行人和理由，并把同一记录原子追加到配置的审计日志。
普通内容变化不会续期；无效、歧义或未推进期限的计划保持零写入。

演进中身份可以提前准备元数据而不受门禁。终止身份免除持续责任，但继续遵循 C-015
生命周期和终止目标义务。每个已建立的迁移后继都按自身声明检查，不能继承前任确认。

报告公开责任、当前复核和知识冗余覆盖率、期限风险及逐身份 bus factor。
`DH_OWNERSHIP_001`、`DH_REVIEW_001/002` 和 `DH_KNOWLEDGE_001` 分别作为三个已交付治理级
身份不变量的证据。旧 suppression 只会让匹配不变量变为 unverified，不能证明责任或冗余。
