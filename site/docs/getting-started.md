---
title: 快速开始
description: 构建 Docs Hygiene，为项目生成初始策略并运行第一次检查。
---

# 快速开始

这一页完成三件事：构建 DH、生成项目策略、运行第一次检查。

## 前置条件

- Rust 1.85 或更高版本；
- 一个使用 Git 管理的项目目录；
- 可以在本地或 CI 中运行命令。

## 构建 CLI

当前版本从源码构建：

```bash
git clone https://github.com/IndenScale/docs-hygiene.git
cd docs-hygiene
cargo build --release
```

生成的二进制位于 `target/release/docs-hygiene`。可以直接使用这个路径，也可以将其加入自己的工具目录。

## 为项目生成初始策略

进入需要治理的项目，运行：

```bash
/path/to/docs-hygiene scaffold .
```

脚手架会创建最小文档结构和 `docs-hygiene.yml`。如果目标文件已经存在，DH 不会悄悄覆盖它们。

## 运行第一次检查

```bash
/path/to/docs-hygiene check .
```

默认行为是：

- `error` 使命令返回失败；
- `warning` 显示建议，但不阻断；
- `info` 提供观察结果。

准备把 warning 也作为门禁时，运行：

```bash
/path/to/docs-hygiene check . --fail-on-warning
```

## 理解为什么一条规则被激活

DH 不要求项目先选择统一的成熟度等级。它根据当前项目事实决定规则是否适用，再由显式策略确定执行强度。

```bash
/path/to/docs-hygiene explain-rules .
/path/to/docs-hygiene explain-rules . --format json
```

输出会说明发现了哪些事实、规则当前是 `inactive`、`advisory`、`warning` 还是 `error`，以及项目配置是否覆盖了自动判断。

## 查看治理画像

```bash
/path/to/docs-hygiene profile .
```

画像分别报告结构、身份、依赖和拓扑能力，不把不同问题压缩成一个模糊的总分。

## 下一步

- 阅读 [DH 如何工作](how-it-works.md)，理解 finding 的来源；
- 阅读 [项目配置](configuration.md)，把检查范围对齐到真实项目；
- 阅读 [接入 CI](ci.md)，建立持续反馈。
