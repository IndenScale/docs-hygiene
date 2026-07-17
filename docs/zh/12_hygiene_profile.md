# 文档卫生画像

画像评估器把治理深度与规则执行严重程度分别报告。画像是一个向量，不能替代各维度
结果。

## 配置

```yaml
hygieneProfile:
  dimensions:
    structure: { target: controlled, required: true }
    identity: { target: controlled, required: true }
    dependency: { target: controlled, required: true }
    topology:
      applicability: notApplicable
      rationale: 当前不治理语义依赖图。
```

适用且必需的维度参与总体最低等级计算。可选维度仍然报告，但不会降低总体结果。
`notApplicable` 维度必须提供理由且不能声明目标。
适用的拓扑维度若以 `controlled` 为目标，还必须显式配置 `governance.topology` 策略；
仅发现关系图只能证明 `basic`。

## 兼容迁移

迁移期间，`documentContracts.maturity.declared` 只映射结构目标：

| 旧成熟度 | 结构目标 |
| --- | --- |
| `seed` | `basic` |
| `growing` | `controlled` |
| `maintained` | `controlled` |
| `governed` | `governed` |

显式结构目标必须与旧值映射一致；冲突会成为配置错误，不使用静默优先级。旧成熟度不会
设置身份、依赖或拓扑目标。

## 评估

每个原子不变量只属于一个维度和一个最低成熟度。只有该等级及所有更低等级的每个适用
不变量都通过时，才检测为达到该等级。以下结果不能算作通过：

- checker 产生可见失败；
- checker 被显式 disabled；
- 失败被旧 suppression 隐藏；
- 不变量实现仍是部分或缺失。

没有适用性证据的 inactive 规则不参与对应不变量。旧 suppression 命中时，画像会在
`suppressionReasons` 中保留其配置理由，使迁移债务仍然可审计。执行状态保持独立：
advisory 或 warning 可以不阻断 CI，但对应不变量失败仍会阻止检测成熟度提升。

## 输出与 CI

```bash
docs-hygiene profile
docs-hygiene profile --format json
docs-hygiene profile --fail-below-target
```

JSON 使用 `docs-hygiene.profile.v1`，包含项目事实快照、有序执行决策、文档模板覆盖与
绑定、规范化治理图及拓扑指标、各维度目标与检测结果、N/A 理由、不变量证据、可选总体
结果及所有必需目标是否满足。失败开关是可选的，命令会先输出报告再返回非零状态。

## 当前交付边界

原子不变量评估、旧值映射、N/A 排除、suppression 不构成证据、目标门禁、可复用模板
绑定与生命周期迁移、规范化治理边、标题 selector 解析、Fan-In/Fan-Out 与循环分析，
以及显式拓扑阈值、带版本的语法中立引用收集、多粒度多锚校验、传递影响、身份生命周期
、权威迁移、关键 Pin 策略和可移植离线 commit snapshot 已经交付；预算、趋势及可审计
例外执行仍属于计划能力。
权威覆盖账本见
[SPEC-003 C-007](definition/spec/spec-003/constraints/atomic-invariants.md)。
