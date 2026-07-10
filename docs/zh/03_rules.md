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

## I18n

`DH_I18N_001` 表示根文档缺少本地化同位文档。

`DH_I18N_002` 表示本地化文档缺少根文档同位文档。

## 语言

`DH_LANG_001` 表示文档低于配置的最小 CJK 比例。

`DH_LANG_002` 表示文档高于配置的最大 CJK 比例。

## 概念

`DH_CONCEPT_001` 表示高亮概念引用缺少概念定义文件。

`DH_CONCEPT_002` 表示概念定义文件没有被 docs 引用。

## Adapter

`DH_ADAPTER_001` 表示外部 adapter 执行失败。
