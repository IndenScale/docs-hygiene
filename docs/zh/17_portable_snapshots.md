# 可移植 Commit 快照

可移植快照把 commit 证据保存为已登记的本地 payload。它扩展 SHA-256 file/block anchor，
使 `check` 不再依赖产出仓库、网络或 Git。`scope: repo` 继续作为本地完整仓库机制。

## 登记

```yaml
governance:
  portableSnapshots:
    manifests: [snapshots/vendor-release.yml]
    requireSignatures: true
    trustedKeys:
      vendor-release: <64-hex-ed25519-public-key>
```

空配置保持兼容：不执行 snapshot I/O，也不增加 Git 或网络依赖。每个已登记 manifest 和
本地 payload 都会校验，即使当前没有 anchor 消费它。

## 版本化 Manifest

```yaml
schemaVersion: docs-hygiene.snapshot.v1
id: vendor-release-1
repository: github:vendor/docs
commit: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
status: active
replacedBy: null
retainUntil: 2030-01-01
entries:
  - target: RETRY-POLICY
    path: docs/retry.md
    payload: payload/retry.md
    scope: block
    locator: retry-contract
    digest: <64-hex-sha256>
signature:
  algorithm: ed25519
  keyId: vendor-release
  value: <128-hex-signature>
```

`id` 是 snapshot 制品身份。`repository` 必须是 `github:organization/repository` 这类无
凭据稳定身份；URL、user-info、query 和 fragment 均被拒绝，不能成为权威。`commit` 是
完整 40 或 64 hex OID。entry 的 `path` 指向产出仓库路径，`payload` 相对当前 manifest。

file entry hash 完整 payload 字节。block entry 保留完整源 payload，但只 hash `locator`
唯一解析的 ATX 标题段，规则与普通 block anchor 相同。portable manifest 不允许 repo
scope。

## Anchor Provenance

```yaml
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <64-hex-sha256>
    scope: block
    locator: retry-contract
    snapshot:
      manifest: vendor-release-1
      repository: github:vendor/docs
      commit: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
      path: docs/retry.md
```

规范化 pinned edge 保留这份类型化 provenance。检查器同时验证 manifest 登记、
repository、commit、源 path、target、scope、locator、digest、本地 payload 和当前
canonical 目标；URL 永远不是 snapshot 身份。

## 签名与生命周期

Ed25519 签名覆盖 manifest 除 `signature` 外所有字段按 schema 字段顺序形成的紧凑 UTF-8
JSON，包括 schemaVersion、身份、repository、commit、生命周期字段及有序 entry 列表。
受信公钥放在项目策略中，私钥和凭据不得进入 manifest。`requireSignatures` 会要求签名；
只要声明了签名，无论开关如何都会验证。

状态为 `active`、`replaced`、`revoked`。只有 active snapshot 可证明 anchor。replaced
必须通过 `replacedBy` 指向另一个已登记 active snapshot；replaced 与 revoked 都必须有
合法 `retainUntil`，从而保留审计证据而不让外部历史成为项目语义 SSOT。

## 显式导入，离线检查

远程获取被刻意放在 checker 之外。先显式 clone/fetch 产出仓库，再只从本地 checkout
导入：

```bash
docs-hygiene import-snapshot . \
  --manifest snapshots/vendor-release.yml \
  --source /tmp/vendor-checkout

docs-hygiene import-snapshot . \
  --manifest snapshots/vendor-release.yml \
  --source /tmp/vendor-checkout \
  --apply
```

默认 `docs-hygiene.snapshot-import.v1` 计划只读。命令用本地 Git 从精确 commit 读取每个
声明路径，验证 file/block digest，只有 `--apply` 才原子生成全部 payload。它不会
clone/fetch、修改已签名 manifest 或保存 checkout 位置。导入完成后可删除 source
checkout，`check` 仍完全离线。

## 诊断与画像证据

- `DH_SNAPSHOT_001`：manifest 登记、schema、身份或 entry 无效；
- `DH_SNAPSHOT_002`：repository 身份或 provenance 不匹配；
- `DH_SNAPSHOT_003`：commit OID 或 provenance 不匹配；
- `DH_SNAPSHOT_004`：源/payload path 或 payload 可用性失败；
- `DH_SNAPSHOT_005`：target、scope、locator 或 digest 不匹配；
- `DH_SNAPSHOT_006`：签名缺失、不受信或无效；
- `DH_SNAPSHOT_007`：状态、替换或保留策略无效。

`dependency.scoped-anchor` 继续描述 anchor 机制支持；登记 manifest 后，独立适用的
`dependency.portable-snapshot` 证明完整离线可移植策略。
