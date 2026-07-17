---
id: SPEC-003-VERIFICATION
status: baselined
---

# 验证

自动化 Fixture 必须把下列能力作为一个当前基线共同证明：

- 注册表顺序确定、规则归属完整、成熟度累积、目标与检测结果分别报告、N/A 排除，且
  disabled 或 suppressed 规则不构成证据；
- 旧 Profile 映射、确定性模板合并、完整模板绑定、内联 Profile 兼容，以及模板和
  Document Kind 在兼容或阻断场景下的原子迁移；
- 边兼容、带版本的语法中立引用收集、显式 Markdown 非语义策略、语法无关规范化、
  确定性标题 selector、本地化签名同位和共享 slug 规范化；
- block、file、commit 锚定隔离，逐项多锚诊断，可选 commit 校验，可移植快照导入，
  关键依赖策略，显式 Pin 更新计划，原子写入和审计输出；
- 去重的反向传递闭包、确定性循环终止、不同邻居 Fan 指标、稳定循环组、拓扑阈值、
  趋势和限定作用域的超级节点例外；
- 生命周期义务、叶子与资产权威迁移、本地化后继同位、终止目标拒绝、Library claim
  权威与重复策略、Owner 期限、Review Reset 和双人理解；
- 组合文本与 JSON 画像输出保持稳定，且仓库 dogfood 在 warning-as-error 模式下通过。
