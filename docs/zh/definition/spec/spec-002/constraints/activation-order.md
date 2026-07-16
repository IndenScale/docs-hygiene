---
id: SPEC-002-C-003
status: baselined
---

# C-003 激活顺序

`disabled` 始终产生 inactive，`required` 始终产生 error。处于 `auto` 时，显式功能
策略和结构适用性优先于规模信号；纯规模信号最高只产生 advisory。
