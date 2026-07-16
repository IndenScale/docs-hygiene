---
id: SPEC-001-C-010
status: baselined
---

# C-010 Domain Fan-out Budget

The checker counts each direct Markdown leaf or child Domain in a canonical Library manifest as one member. [[GLOSSARY-DOMAIN-FANOUT-BUDGET|Fan-out]] emits only the highest applicable diagnostic per Domain: warning from `warningAt` and error from `errorAt`, with `errorAt > warningAt >= 1`.
