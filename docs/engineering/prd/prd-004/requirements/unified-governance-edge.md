---
id: PRD-004-FR-001
status: baselined
---

# Unified Governance Edge

The checker shall normalize semantic references and pinned references into a
typed dependency edge model. Issue adapters may add `addresses`, `dependsOn`,
and `evidencedBy` relations without imposing repository topology. Resolution,
selectors, pins, lifecycle, impact, and topology policy shall operate on that
model. Navigational Markdown Links shall remain path-integrity inputs and shall
not become semantic dependencies merely because they are links.
