---
id: PRD-004-DELIVERY
status: proposed
---

# 交付状态

推进中。SPEC-003 已定义累积画像评估、N/A 与 suppression 语义、旧成熟度映射、边
规范化和每个交付切片的证据要求。实现已集中兼容规则族元数据并注册原子不变量。带版本
的 `profile` 报告在一份输出中组合项目事实、独立执行决策、各维度目标与检测结果、N/A
理由和不变量证据。旧成熟度映射、冲突检测、suppression 不构成证据和可选低于目标 CI
门禁均已可执行并由 fixture 覆盖。

后续切片已把语义引用、锁定引用、派生和投影规范化为统一的有序边记录，并公开确定性的
图、Fan 与循环指标；显式阈值通过独立的 `governance.topology` 规则族执行。受控结构切片
现已增加可复用文档模板注册表、确定性合并、绑定覆盖率、稳定诊断、画像证据和内联
Profile 兼容。治理级结构切片进一步增加 revision 兼容窗口、精确 Profile pin、只读迁移
计划和原子兼容迁移。本仓库的五个 Profile 已全部绑定到一个带 revision 的共享模板；
结构目标达到治理级，所有已配置目标均通过。依赖 selector 切片现已解析标题 selector、
确定性寻址 canonical ATX slug、保持本地化签名、在规范化边记录 selector 证据，并产生
`DH_SELECTOR_001`。统一引用模型切片现已让 Wiki Link、Markdown Link 与 frontmatter
声明进入带版本的 occurrence IR；显式语法与上下文策略驱动唯一的语法无关边规范化器，
同时保持诊断与边 JSON 不变。多粒度锚切片增加 frontmatter 多锚声明、file/block SHA-256
校验、兼容的 file JSON，以及默认关闭且逐项留证的本地 commit 校验。确定性反向传递
影响现已把每种已解析语义边传播到关系图报告和依赖画像证据。身份生命周期切片现已跨
资产与 Package 身份校验状态义务和 `supersededBy`，拒绝终止
目标并报告有序权威迁移。预算、趋势和可审计例外执行仍未交付。
