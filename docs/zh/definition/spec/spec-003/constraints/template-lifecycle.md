---
id: SPEC-003-C-010
status: proposed
---

# C-010 文档模板生命周期

治理级文档模板声明正整数 `revision` 和兼容下界 `compatibleFrom`。下界默认等于当前
revision，因此对旧 revision 的兼容必须显式声明。有效窗口包含两端，且必须满足
`1 <= compatibleFrom <= revision`。

每个已绑定 Profile 使用 `templateRevision` 固定模板 revision。等于当前 revision 的 pin
是最新状态；兼容窗口内较低的 pin 可以迁移；窗口外或高于当前 revision 的 pin 不兼容。
`DH_TEMPLATE_003` 报告缺失 revision 元数据、缺失 pin 和兼容但过期的 pin：低于治理级时
只提供建议，治理级时阻断。`DH_TEMPLATE_004` 始终阻断不兼容 pin。

`docs-hygiene migrate-templates` 补充缺失 pin，并把兼容 pin 推进到当前 revision。
`--check` 只读生成迁移计划，并在需要变更时失败。迁移是原子的：任何未知、无效、重复、
无 revision 或不兼容绑定都会阻止全部写入。文本和 JSON 输出以确定顺序列出变更、未变
Profile、阻断项和是否已应用；JSON 使用 `docs-hygiene.template-migration.v1`。

只有可复用模板覆盖通过、每个模板都有有效 revision 窗口、每个 Profile 固定到模板当前
revision，且不存在待迁移或不兼容项时，`structure.template-migration` 才达到治理级。
suppression 不构成通过证据。

模板 revision 表示策略兼容性，不属于受管文档身份；Git 仍负责文档内容历史。迁移命令
只推进绑定元数据。两个 revision 是否兼容必须由策略所有者显式声明，不能据此隐式改写
文档内容。
