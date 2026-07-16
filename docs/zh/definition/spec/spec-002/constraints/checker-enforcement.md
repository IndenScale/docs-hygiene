---
id: SPEC-002-C-005
status: baselined
---

# C-005 Checker 执行

inactive 规则族不产生诊断，也不调用外部进程。advisory 与 warning 决策把派生诊断
分别限制为 Info 与 Warning；error 决策保留 checker 配置的严重程度语义。
