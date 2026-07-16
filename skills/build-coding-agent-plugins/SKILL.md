---
name: build-coding-agent-plugins
description: Package an existing repository as an installable plugin for Codex, Claude Code, and Kimi Code while sharing Agent Skills across products. Use when comparing coding-agent extension mechanisms, adding product manifests or marketplaces, converting a repository into a plugin, or deciding between skills, hooks, MCP servers, commands, and repository instructions.
---

# Build Coding Agent Plugins

Treat the repository as the product and each agent manifest as a thin adapter. Keep reusable behavior in shared Agent Skills; keep executable domain logic in the repository's CLI or MCP server.

## Workflow

1. Read [references/plugin-mechanisms.md](references/plugin-mechanisms.md) before choosing layouts or install commands. Re-check the linked official documentation when current schema details matter.
2. Identify the repository's stable interface: CLI commands, JSON output, MCP tools, or scripts. Do not reproduce that logic in prompts.
3. Create shared skills under `skills/<skill-name>/SKILL.md`. Use portable frontmatter (`name` and `description`) unless a product-specific feature is essential.
4. Add only the manifests required by the target products:
   - Codex: `.codex-plugin/plugin.json`; use `.agents/plugins/marketplace.json` for repository distribution.
   - Claude Code: `.claude-plugin/plugin.json`; add `.claude-plugin/marketplace.json` when distributing a marketplace.
   - Kimi Code: `kimi.plugin.json` or `.kimi-plugin/plugin.json`; direct GitHub installation does not require a marketplace.
5. Keep all manifest paths inside the plugin root. Assume marketplace installation copies the plugin to a managed cache.
6. Add hooks, MCP servers, commands, agents, or apps only when the repository really provides them. Never declare placeholder components.
7. Validate JSON, validate each skill, then run each available product validator. Test from a copied or cached installation, not only from the source tree.
8. Document runtime prerequisites explicitly. A skill can teach an agent how to call a CLI, but it does not by itself install that CLI.

## Design Rules

- Prefer one cross-product skill over three duplicated prompt files.
- Keep product-specific behavior in manifests or dedicated companion files.
- Use MCP only when the agent needs structured tools or a long-running service. Prefer a CLI with JSON output for deterministic local checks.
- Use hooks only for lifecycle enforcement that must run automatically. Do not hide ordinary workflow steps in hooks.
- Use repository instructions (`AGENTS.md`, `CLAUDE.md`, and equivalents) for conventions that apply only while editing that repository; use a plugin for reusable installation across repositories.
- Pin or version releases deliberately because all three products use managed copies or caches for installed plugins.
- Treat third-party plugins as executable supply-chain inputs. Keep install-time behavior minimal and require explicit trust for external commands.

## Verification

Verify at least:

```bash
python3 /path/to/skill-creator/scripts/quick_validate.py skills/<skill-name>
python3 /path/to/plugin-creator/scripts/validate_plugin.py .
claude plugin validate .
```

Run only validators that are installed. For Kimi Code, inspect `/plugins info <id>` after installation and reload into a new session.
