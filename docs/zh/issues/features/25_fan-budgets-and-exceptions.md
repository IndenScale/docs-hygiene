---
id: FEATURE-025
epic: EPIC-007
status: baselined
delivery_status: delivered
depends_on: [FEATURE-023]
---

# FEATURE-025 Fan-In/Fan-Out 预算与超级节点例外

## 能力边界

按不同已解析邻居计算 Fan-In/Fan-Out，执行全局阈值和 cycle 策略；合法超级节点使用精确
node/direction、预算、理由、Owner、Approver、expiry 和 degree history 例外。

## 验收

- 重复链接和平行 relation 不放大度数；
- 普通节点、相反方向和超例外预算继续阻断；
- 过期、闲置、缺历史或无效例外分别诊断且不能证明 Passed。

## 交付证据

`src/checks/topology.rs`、topology reports/profile 和 topology CLI/tests。
