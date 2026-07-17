---
id: SPEC-003-C-018
status: baselined
---

# C-018 可审计超级节点例外

supernode exception 把一个唯一例外身份绑定到一个精确治理 node 和 `fanIn`/`fanOut`
方向。它声明高于对应全局限制的例外预算、非空理由、owner、approver、expiry 及有序带日期
度数历史；不存在 wildcard matcher。

只有在元数据有效、节点超过全局限制、当前度数不超过例外预算且至少有一条非未来历史时，
例外才生效，并且只放宽该 node/direction。到期、元数据无效、缺历史或超过例外预算都会
恢复普通阈值失败；节点回到全局限制内会使声明闲置并产生清理证据。

`topologyExceptions` 报告当前度数、两级预算、带符号剩余额度、最近 observation、趋势差、
生命周期状态、审计元数据及反向传递影响。稳定 `DH_TOPOLOGY_003` 至 `005` 分别区分
无效/过期、闲置和历史故障。

已应用例外不阻断，但画像把 `topology.thresholds` 标为 `excepted`，绝非 Passed。旧
suppression 按 [C-005](exceptions.md) 保持 unverified。预算、审计例外和趋势机制提供已交付
的 governed 级不变量实现；活跃例外仍会阻止更低层阈值不变量证明 controlled 成熟度。
