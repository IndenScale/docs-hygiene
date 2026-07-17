---
id: EPIC-001
status: baselined
delivery_status: delivered
depends_on: []
---

# EPIC-001 文档基础：命名、内容与基础引用

## 能力链

`Naming → Document Kind → Content Contract → Stable Identity → File Reference`

本 Epic 先确定文件如何被发现和命名，再确定内容结构，最后建立稳定身份和文件级寻址。
Block、Hash、语义依赖链和图分析不属于本 Epic。

## Features

1. [FEATURE-001 命名、Kind 与稳定身份](../features/01_naming-kind-and-identity.md)
2. [FEATURE-002 内容契约、Schema 与脚手架](../features/02_content-contracts-and-scaffolding.md)
3. [FEATURE-003 Library/Body 与文件级引用](../features/03_library-body-and-file-reference.md)

## Epic 验收

- 任一受管路径可确定性推导 Kind 和身份；
- 合法内容可以生成、校验和迁移；
- 文件级引用在不依赖 selector、hash 或图算法时即可解析。
