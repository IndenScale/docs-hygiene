# 可审计超级节点例外

全局 Fan-In/Fan-Out 限制仍是默认拓扑契约。supernode exception 只承认一个稳定身份在一个
方向上的有意超额，不是 wildcard、诊断 suppression，也不能证明全局阈值 Passed。

## 策略

```yaml
governance:
  topology:
    maxFanIn: 8
    maxFanOut: 12
    exceptions:
      - id: shared-retry-contract
        node: RETRY-POLICY
        direction: fanIn
        budget: 20
        reason: shared public retry semantics
        owner: platform-docs
        approvedBy: architecture-council
        expires: 2027-01-31
        history:
          - observedAt: 2026-04-01
            degree: 11
          - observedAt: 2026-07-01
            degree: 14
```

`id` 和 `(node, direction)` 必须分别唯一。`node` 是精确的已登记治理身份，direction 为
`fanIn` 或 `fanOut`；例外预算必须高于对应全局限制。`reason`、`owner`、`approvedBy`
及合法且尚未过期的 `expires` 均必填，不存在 glob 或 wildcard 字段。

history 日期必须合法、严格递增且不在未来。正在覆盖违规的例外至少需要一条 observation，
最近趋势为 `currentDegree - latestObservedDegree`。续期必须显式修改审批和 expiry 证据，
`check` 永不自动续期。

## 运行语义

只有当全部元数据有效、节点超过全局限制、存在最近历史且当前度数不超过例外预算时，例外
才是 `applied`。只有该 node+direction 免于 `DH_TOPOLOGY_001`；其他节点、相反方向、
cycle 策略和超过例外预算的部分仍然阻断。

标准 text/JSON 报告和画像为每条声明公开：

- `applied`、`idle`、`invalid`、`expired` 或 `exceeded` 状态；
- 当前度数、全局/例外预算及带符号剩余额度；
- 最近 observation 和趋势差；
- owner、approver、reason、expiry 与反向传递影响。

删除最后一条超额边后，例外变为 `idle` 并产生清理 warning。例外到期后变为 `expired`、
立即停止生效，底层阈值违规自动恢复。

## 诊断

- `DH_TOPOLOGY_001`：普通节点或已失效例外节点超预算；
- `DH_TOPOLOGY_002`：仍存在被禁止的有向环；
- `DH_TOPOLOGY_003`：例外身份、目标、方向、预算、审计字段或 expiry 无效；
- `DH_TOPOLOGY_004`：例外闲置，应删除；
- `DH_TOPOLOGY_005`：活跃例外历史缺失、无效、乱序或来自未来。

超过自身预算的例外状态为 `exceeded`，并产生普通 `DH_TOPOLOGY_001`。无效或过期声明
不能放宽策略。

## 画像证据与旧 Suppression

已应用例外保留为 `topologyExceptions` 证据。`topology.thresholds` 不变量结果为
`excepted`，因此非阻断例外不能证明 Passed 或提升拓扑观测成熟度。审计预算、公开例外和
趋势机制分别由 `topology.budgets`、`topology.public-exceptions`、`topology.trends` 表示。

旧 `suppressions` 继续兼容，但画像把它记录为 `unverified`；它不能替代审计 supernode
声明，也不能建立 governed 拓扑成熟度。
