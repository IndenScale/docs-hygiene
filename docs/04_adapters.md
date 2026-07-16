# Adapters

Docs Hygiene uses **Adapter** configuration to call external documentation
tools. This keeps the project focused on policy while mature tools keep their
own rule ecosystems.

## Markdownlint

The first adapter is markdownlint.

```yaml
adapters:
  markdownlint:
    enabled: true
    command: markdownlint-cli2
    args:
      - README.md
      - README_ZH.md
      - CHANGELOG.md
      - "docs/**/*.md"
```

Docs Hygiene does not parse markdownlint output yet. If the command exits with
a non-zero status, Docs Hygiene emits `DH_ADAPTER_001` and includes the adapter
output in the diagnostic message.

## Recommended Boundary

Use markdownlint for Markdown syntax and formatting rules.

Use Docs Hygiene for repository-level policy:

- required documentation entry files
- numbered docs structure
- language-representation parity
- language thresholds
- concept foreign keys
