---
id: SPEC-002-C-006
status: baselined
---

# C-006 确定性输出

`explain-rules` 按稳定规则 ID 顺序输出决策。JSON 使用
`docs-hygiene.rule-activation.v1` Schema；路径遍历顺序不得影响事实、证据或输出顺序。
