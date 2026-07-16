# Coding Agent 插件机制对比

资料核对日期：2026-07-17。下列结论只保留可复用的结构与工作流；字段级细节在实施前应重新核对官方文档。

## 结论

三者已经在 `SKILL.md` 上形成可共享的最小公分母，但“插件”不是同一个封装格式：

| 产品 | 必需入口 | 可打包能力 | 分发方式 | 安装后行为 |
| --- | --- | --- | --- | --- |
| Codex | `.codex-plugin/plugin.json` | Skills、MCP、Apps、Hooks、Assets | `.agents/plugins/marketplace.json`；CLI 添加 marketplace；桌面端安装 | 安装副本进入 Codex cache；新任务加载更新 |
| Claude Code | `.claude-plugin/plugin.json`（快速试验可省略） | Skills、Agents、Hooks、MCP、LSP、Monitors、Settings | `.claude-plugin/marketplace.json`，或 `--plugin-dir` 本地测试 | marketplace 插件复制到 `~/.claude/plugins/cache`；路径不能越出插件根目录 |
| Kimi Code | `kimi.plugin.json` 或 `.kimi-plugin/plugin.json` | Skills、Session Start、Commands、MCP、Hooks | `/plugins install <path-or-url>`，也可提供自定义 marketplace JSON | 安装副本进入 `$KIMI_CODE_HOME/plugins/managed/<id>`；需 `/reload` 或新会话 |

## 共同架构

建议仓库根目录采用以下布局：

```text
repo/
├── .codex-plugin/plugin.json
├── .claude-plugin/plugin.json
├── kimi.plugin.json
├── .agents/plugins/marketplace.json
├── .claude-plugin/marketplace.json
├── skills/
│   └── product-workflow/
│       ├── SKILL.md
│       ├── scripts/
│       └── references/
└── product source, CLI, or MCP server
```

共享层只使用 Agent Skills 开放格式：目录名稳定，`SKILL.md` frontmatter 至少提供 `name` 与 `description`。各产品的额外 frontmatter 字段可能不同，不应写入共享 skill，除非确认其他产品会安全忽略。

## Codex

- Skill 是可复用工作流；Plugin 是可安装的分发包。先写 Skill，确需跨团队分发、MCP、App 或 Hook 时再封装 Plugin。
- manifest 位于 `.codex-plugin/plugin.json`，组件目录位于插件根目录，而不是 `.codex-plugin/` 内。
- 仓库级 marketplace 位于 `.agents/plugins/marketplace.json`。插件位于仓库根目录时，可让 marketplace 的 Git source 指向该仓库；插件位于子目录时使用 `git-subdir` 或本地相对路径。
- Codex 桌面端安装的是缓存副本，不应依赖插件根目录以外的文件。
- 本地更新需要重新安装并在新任务中测试。

官方资料：

- [Codex manual](https://developers.openai.com/codex/codex-manual.md)
- [Build plugins](https://learn.chatgpt.com/docs/build-plugins)
- [Build skills](https://learn.chatgpt.com/docs/build-skills)

## Claude Code

- 独立 `.claude/` 配置适合单仓库试验；Plugin 适合版本化、跨项目安装和 marketplace 分发。
- 标准组件包括 `skills/`、`agents/`、`hooks/`、`.mcp.json`、`.lsp.json` 等；插件 Skill 会加上插件命名空间。
- `claude --plugin-dir ./path` 适合本地测试；正式分发使用 `.claude-plugin/marketplace.json`。
- marketplace 安装会复制到缓存，`../` 引用不会随插件复制，必须把运行时文件放在插件根目录之内。
- 使用 `claude plugin validate .` 验证 manifest、marketplace 和组件 frontmatter。

官方资料：

- [Create plugins](https://code.claude.com/docs/en/plugins)
- [Plugins reference](https://code.claude.com/docs/en/plugins-reference)
- [Create and distribute a plugin marketplace](https://code.claude.com/docs/en/plugin-marketplaces)
- [Discover plugins](https://code.claude.com/docs/en/discover-plugins)

## Kimi Code

- 当前 Kimi Code Plugin 是 Skills、Commands、MCP 与 Hooks 的安装单元；旧版 `plugin.json` 工具插件说明已被当前 `kimi.plugin.json` 机制取代，不应混用旧 schema。
- manifest 优先读取根目录 `kimi.plugin.json`，其次读取 `.kimi-plugin/plugin.json`。
- 可从本地目录、ZIP 或 GitHub URL 直接安装；GitHub 仓库根目录可以直接作为插件。
- `skills` 必须使用插件根目录内的 `./` 路径。`sessionStart.skill` 只注入 Skill 文本，不执行脚本。
- 安装源会被复制到 managed 目录，修改原目录后必须重新安装；启停或更新后需要 `/reload` 或新会话。
- 当前安装范围是用户级，而不是项目级。

官方资料：

- [Plugins](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/plugins)
- [Agent Skills](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/skills)
- [MCP](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/mcp)

## 选择扩展面

| 需求 | 首选 |
| --- | --- |
| 教 Agent 一套可复用流程 | Skill |
| 为 Agent 提供确定性本地能力 | CLI + JSON，Skill 负责调用 |
| 提供结构化远程或长驻工具 | MCP |
| 在生命周期事件中强制执行 | Hook |
| 单仓库长期约束 | `AGENTS.md` / `CLAUDE.md` / 项目配置 |
| 跨仓库安装与版本更新 | Plugin + Marketplace |

不要把核心业务算法写进 `SKILL.md`。提示词适合解释何时调用、如何解释结果、哪些操作需要授权；可重复验证的规则应留在 CLI、MCP 服务或测试中。
