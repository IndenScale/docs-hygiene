---
id: SPEC-003-C-016
status: baselined
---

# C-016 关键依赖 Pin 策略

关键依赖策略按 source/target 引用关系 Kind、规范化 relation、source/target 路径 glob 与
稳定身份集合选择规范化边。空 matcher 维度表示通配；未命中的边无需 anchor 仍然合法。

要求分别约束允许算法、最低 scope、whole-file scope 与最大审计年龄。scope 强度固定为
`file < commit < block`。`DH_PIN_001` 至 `DH_PIN_006` 区分缺失、scope 不足、算法不允许、
内容变化、审计过期和声明无效；诊断公开直接依赖方与目标反向影响。

Frontmatter anchor 可携带 `updatedAt`、`updatedBy` 和 `reason`，配置最大年龄后必须三项
齐全。`update-pins` 除非显式 `--apply` 否则只读，支持 policy/target 子集，在任何写入前
验证完整选中计划，并以带回滚保护的原子批次同时写入更新声明和配置的 JSONL 审计日志。
`check` 永不接受新 digest。

版本化计划为 `docs-hygiene.pin-update.v1`。每项 change 包含 policy、source、target、
规范化 relation、source/target 路径、算法、scope、selector、旧/新 digest、日期、actor
和 reason。
