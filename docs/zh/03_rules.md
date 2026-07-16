# 规则

Docs Hygiene 输出稳定的诊断码。第一版规则面保持收敛，专注仓库文档卫生。

## 入口文件

`DH_REQUIRED_001` 表示 `requiredFiles` 中声明的必需文件不存在。

## 编号文档

`DH_NAME_001` 表示 docs 根目录下的 Markdown 文件名不符合 `docs.filenamePattern`。

`DH_SEQ_001` 表示编号文档组中存在断号。

`DH_SEQ_002` 表示编号文档组中存在重复编号。

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

`DH_MATURITY_001` 根据配置的仓库规模信号建议提高项目声明的治理成熟度。

## 概念

`DH_CONCEPT_001` 表示高亮概念引用缺少概念定义文件。

`DH_CONCEPT_002` 表示概念定义文件没有被 docs 引用。

## 治理关系图

`DH_GOVERNANCE_001` 表示 Manifest 无法读取或解析、`id@version` 重复、语义版本
无效，或生命周期状态无效。

`DH_REFERENCE_001` 表示水平引用目标缺失，或 `Body -> Library` 的目标不是
同一精化层级 Library。

`DH_LIBRARY_001` 表示 Library 目录成员缺失、格式无效、身份重复或未在 Manifest
中声明，适用于递归 Library Tree。

`DH_BODY_001` 表示目录型 PRD 或 Spec Body Package 出现相同的结构或本地化错误。

`DH_DERIVATION_001` 表示通过 `formalizes` 或 `realizes` 建立的相邻精化层级 Body 派生
缺失、无法解析、类型错误或不完整。

`DH_DERIVATION_002` 表示通过 `projects` 建立的相邻精化层级 Library 投影缺失、
无法解析、类型错误或不完整。

## Adapter

`DH_ADAPTER_001` 表示外部 adapter 执行失败。
