# 规则

Docs Hygiene 输出稳定的诊断码。第一版规则面保持收敛，专注项目文档卫生。

## 入口文件

`DH_REQUIRED_001` 表示 `requiredFiles` 中声明的必需文件不存在。

## 编号文档

`DH_NAME_001` 表示 docs 根目录下的 Markdown 文件名不符合 `docs.filenamePattern`。

`DH_SEQ_001` 表示编号文档组中存在断号。

`DH_SEQ_002` 表示编号文档组中存在重复编号。

## Slug 身份

`DH_SLUG_001` 表示 slug 非法或命中保留字、规范化或大小写碰撞、alias 冲突、同一稳定
身份的 canonical/localized slug 漂移、权威来源缺失，或重命名策略尚未落实。

## 大小

`DH_SIZE_001` 表示文档超过 `docs.maxLines`。

## ASCII 字符画

启用 `docs.forbidAsciiArt` 后，`DH_ASCII_001` 表示文档正文或 `text`、`ascii`、`diagram` fenced block 中存在连续的 ASCII 字符画块。`python`、`bash`、`yaml` 等代码示例不会触发该规则；普通 Markdown 表格和水平分隔线也不会触发。

## 语言表示

`DH_REPRESENTATION_001` 表示 canonical 文档缺少本地化表示。

`DH_REPRESENTATION_002` 表示本地化表示缺少 canonical 文档。

## 语言

`DH_LANG_001` 表示文档低于配置的最小 CJK 比例。

`DH_LANG_002` 表示文档高于配置的最大 CJK 比例。

## 文档契约

`DH_CONTRACT_001` 表示缺少必要语义章节。

`DH_CONTRACT_002` 表示缺少必要字段。

`DH_CONTRACT_003` 表示必要章节仍包含显式占位符。

`DH_CONTRACT_004` 表示必要章节没有按照配置顺序出现。

`DH_TEMPLATE_001` 表示模板或 Profile 身份无效、声明重复、绑定未知、表达式无效，或解析
后的成员身份重复。

`DH_TEMPLATE_002` 表示已配置模板没有任何 Profile 绑定。

`DH_TEMPLATE_003` 表示模板 revision 元数据或兼容 Profile pin 需要迁移；只有在治理级
文档契约成熟度下才阻断。

`DH_TEMPLATE_004` 表示 Profile pin 位于模板声明的兼容窗口之外，始终阻断。

`DH_MATURITY_001` 根据配置的项目规模信号建议提高已声明的 Profile 成熟度。

`DH_KIND_001` 表示 Document Kind 注册表绑定不一致。

`DH_KIND_002` 表示类型化 frontmatter Schema revision 缺失、兼容但过期或不兼容。

`DH_FRONTMATTER_001` 表示字段类型、枚举、格式、未知字段、条件或跨字段不变量违规。

## 概念

`DH_CONCEPT_001` 表示高亮概念引用缺少概念定义文件。

`DH_CONCEPT_002` 表示概念定义文件没有被 docs 引用。

## 项目根目录链接

`DH_LINK_001` 表示项目根目录内的行内 Markdown Link、引用式链接定义或图片目标没有解析到
已有文件或目录。围栏代码、行内代码、同文档片段和外部 URI scheme 不参与这项检查；
外部 URL 的在线可达性仍由 adapter 负责。

## 治理关系图

`DH_ACTIVATION_001` 表示项目事实激活了尚未显式配置的规则族。诊断包含激活状态、
有序证据和逐规则覆盖路径；纯规模激活只产生 Info。

`DH_GOVERNANCE_001` 表示 Manifest 无法读取或解析、语义身份重复、生命周期状态无效，或仍使用已经移除的文档级 `version`、Manifest 级 `references` 字段。

`DH_REFERENCE_001` 表示指向 Library 身份的语义 Wiki Link 缺失、无法解析、精化方向无效或内容哈希过期。

`DH_SELECTOR_001` 表示语义 Wiki Link 的标题 selector 无法在 canonical 受管目标中解析。

`DH_LIBRARY_001` 表示 Library 目录成员缺失、格式无效、身份重复或未在 Manifest
中声明，适用于递归 Library Tree，也包括仍使用已经移除的叶子级 `version` 或 `source` 元数据。

`DH_BODY_001` 表示目录型 PRD 或 Spec Body Package 出现相同的结构或本地化错误，或已声明的 Implementation Body 成员格式错误、重复、路径不安全或不存在；已经移除的叶子级 `version` 或 `source` 元数据也属于无效输入。

`DH_CLAIM_001` 表示核心 Library 权威无效、已确认重复被禁止、迁移逾期，或受控摘录
缺少 block pin / pin 已过期。

`DH_PIN_001` 至 `DH_PIN_006` 分别报告关键依赖缺 Pin、scope 不足、算法不允许、内容
变化、审计年龄过期，以及策略或声明无效。

`DH_SNAPSHOT_001` 至 `DH_SNAPSHOT_007` 在不访问远程仓库的情况下区分 portable
snapshot 登记、repository、commit、path、digest、签名和生命周期故障。

`DH_DERIVATION_001` 表示通过 `formalizes` 或 `realizes` 建立的相邻精化层级 Body 派生
缺失、无法解析、类型错误或不完整。

`DH_DERIVATION_002` 表示通过 `projects` 建立的相邻精化层级 Library 投影缺失、
无法解析、类型错误或不完整。

`DH_TOPOLOGY_001` 表示某个受管身份按不同邻居计算的 Fan-In 或 Fan-Out 超过显式阈值；
启用 `forbidCycles` 后，`DH_TOPOLOGY_002` 表示检测到有向循环组。

`DH_TOPOLOGY_003` 至 `DH_TOPOLOGY_005` 报告审计超级节点例外无效/过期、闲置清理候选
及度数历史缺失或无效；超过自身预算的例外恢复 `DH_TOPOLOGY_001`。

## Adapter

`DH_ADAPTER_001` 表示外部 adapter 执行失败。
