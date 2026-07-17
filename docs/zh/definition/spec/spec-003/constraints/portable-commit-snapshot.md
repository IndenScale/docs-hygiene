---
id: SPEC-003-C-017
status: baselined
---

# C-017 可移植 Commit 快照

`docs-hygiene.snapshot.v1` manifest 以无凭据稳定 repository 身份和完整 commit OID 标识
一份外发制品。有序 entry 绑定治理 target、原仓库 path、本地 payload path、file/block
scope、可选 block locator 与 SHA-256 digest；commit 不是 portable entry scope。

frontmatter anchor 可携带类型化 `snapshot` provenance，包括 manifest 身份、repository
身份、commit OID 和原路径。离线校验要求这些声明与一个已登记 entry 相等，以普通
file/block 语义验证本地 payload，并继续验证当前 canonical 目标。`DH_SNAPSHOT_001` 至
`007` 分别区分登记、repository、commit、path、digest、签名和生命周期故障。

可选 Ed25519 签名覆盖 manifest 除 `signature` 外全部字段按 schema 字段顺序形成的紧凑
UTF-8 JSON。受信公钥属于项目策略；repository 拒绝 URL 和带凭据语法。已声明签名总会
验证，策略也可强制签名。

只有 `active` snapshot 可证明 anchor。`replaced` snapshot 指向已登记 active 后继；
replaced 与 `revoked` 都声明 `retainUntil`。外部历史保持审计证据而非语义权威。

`import-snapshot` 默认只输出 `docs-hygiene.snapshot-import.v1` 只读计划，显式 `--apply`
后才从用户提供的本地 Git checkout 读取精确 commit、验证所有 entry 并原子写入 payload。
它不会进行网络获取或修改已签名 manifest；常规 `check` 对 portable snapshot 不调用 Git
或网络。
