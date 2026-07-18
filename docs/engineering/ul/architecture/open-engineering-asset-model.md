---
id: DH-OPEN-ENGINEERING-ASSET-MODEL
status: baselined
---

# Open Engineering Asset Model

An open engineering model with two durable document structures and two
location-independent delivery roles:

- UL preserves shared language and long-lived constraints.
- PRD preserves product intent, boundaries, and requirements.
- Issue owns change-scoped acceptance, coordination, and delivery evidence.
- Artifact is any implementation or evidence object, including code, tests,
  configuration, generated output, SDK content, and commits.

Only UL and PRD require repository document structures. Issue and Artifact are
identified by adapters and relations, not by fixed directories. The model does
not impose Definition, Implementation, Glossary, SDK, or refinement-level axes.
