---
id: SPEC-001-C-010
status: baselined
---

# C-010 Domain Fan-out Budget

checker 把 canonical Library Manifest 的每个直属 Markdown 叶子或子 Domain 计为一个成员。[[GLOSSARY-DOMAIN-FANOUT-BUDGET|Fan-out]] 对每个 Domain 只产生最高适用诊断：从 `warningAt` 开始 warning，从 `errorAt` 开始 error，并要求 `errorAt > warningAt >= 1`。
