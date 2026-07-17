# 关键依赖 Pin

`governance.criticalDependencies` 把选中的规范化治理边从“可选锚定”提升为显式 Pin
策略。未命中任何 matcher 的边仍可不带 Pin。

## 策略

```yaml
governance:
  pinAuditLog: .docs-hygiene/pin-updates.jsonl
  criticalDependencies:
    - id: reviewed-contracts
      match:
        sourceKinds: [body]
        targetKinds: [library]
        relations: [references, projects]
        sourcePaths: [docs/definition/**/*.md]
        targetPaths: [docs/intent/ul/**/*.md]
        sourceIds: [SPEC-1]
        targetIds: [RETRY-POLICY]
      require:
        algorithms: [sha256]
        minimumScope: block
        forbidWholeFile: true
        maxAgeDays: 90
```

每个非空 matcher 维度之间采用 AND，同一列表内采用 OR；空列表表示通配。Kind 为
`body` 或 `library`，relation 为 `references`、`formalizes`、`realizes`、`projects`，
不受当前边是否已有 Pin 影响。路径是项目相对 glob，身份列表使用稳定治理 ID。

`algorithms` 接受 `sha256` 和 `git`。`minimumScope` 的顺序是
`file < commit < block`：commit 比工作树文件快照更强，block 的空间选择性最高。
`forbidWholeFile` 同时拒绝 file 和 commit，因此要求 SHA-256 block anchor。
配置 `maxAgeDays` 后还要求合法 `updatedAt`、非空 `updatedBy` 和非空 `reason`。

## 诊断

- `DH_PIN_001`：不存在伴随的锁定边；
- `DH_PIN_002`：scope 低于策略或 whole-file 被禁止；
- `DH_PIN_003`：没有 Pin 使用允许的算法；
- `DH_PIN_004`：声明有效，但目标内容已变化；
- `DH_PIN_005`：审计元数据不完整或超过 `maxAgeDays`；
- `DH_PIN_006`：策略/anchor 声明无效或无法验证。

每条诊断都标识直接 source、关联 canonical target 位置，并包含目标的确定性反向传递
影响，从而区分内容漂移和声明损坏。底层 `DH_REFERENCE_001` anchor 校验继续生效。

## 审计元数据

显式 frontmatter anchor 可以记录接受证据：

```yaml
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <64-hex>
    scope: block
    locator: retry-contract
    updatedAt: 2026-07-17
    updatedBy: alice
    reason: reviewed upstream retry semantics
```

新字段对非关键 anchor 可选，因此旧项目兼容；带 `maxAgeDays` 的关键策略会要求三项
全部存在。

## 只读计划与显式 Apply

```bash
docs-hygiene update-pins . \
  --actor alice \
  --reason "reviewed upstream contract" \
  --format json

docs-hygiene update-pins . \
  --policy reviewed-contracts \
  --target RETRY-POLICY \
  --actor alice \
  --reason "accepted retry revision" \
  --apply
```

默认只读。版本化 `docs-hygiene.pin-update.v1` 计划包含旧/新 digest、policy、source、
target、relation、selector、actor、reason 和日期。重复 `--policy`、`--target` 可选择
子集。未知选择、selector 无法解析、anchor 损坏、不安全路径、不支持的算法/scope 组合
或未提交的 commit 目标都会在任何写入前阻止整个选中计划。

`--apply` 以带回滚保护的原子批次更新 frontmatter anchor，并把同一记录追加到
`governance.pinAuditLog`。当所需 scope 可解析时，缺失的 reference 或垂直边 Pin 可在
受管 Markdown 内容中生成；scope 或算法已不再满足要求的既有 Pin 会被迁移。commit 更新
只在 `HEAD` 目标 blob 与工作树相等时执行。`docs-hygiene check` 永不修改或接受指纹。

画像中的 `dependency.scoped-anchor` 继续证明机制可用；
`dependency.critical-pins` 只在至少配置一条关键依赖策略时适用，并独立证明策略全满足。
