---
id: SPEC-003-C-010
status: proposed
---

# C-010 Document Template Lifecycle

A governed document template declares a positive integer `revision` and a
`compatibleFrom` lower bound. The lower bound defaults to the current revision,
so compatibility with older revisions must be explicit. The valid window is
inclusive and must satisfy `1 <= compatibleFrom <= revision`.

Every bound profile pins `templateRevision`. A pin equal to the current
revision is current. A lower pin inside the compatibility window is migratable;
a pin outside the window or above the current revision is incompatible.
`DH_TEMPLATE_003` reports missing revision metadata, missing pins, and compatible
outdated pins. It is advisory below governed maturity and blocking at governed
maturity. `DH_TEMPLATE_004` always blocks incompatible pins.

`docs-hygiene migrate-templates` adds missing pins and advances compatible pins
to the current revision. `--check` performs a read-only migration plan and fails
when changes are required. Migration is atomic: any unknown, invalid, duplicate,
unrevisioned, or incompatible binding prevents every write. Text and JSON output
list deterministic changes, unchanged profiles, blocks, and whether changes
were applied. JSON uses `docs-hygiene.template-migration.v1`.

The `structure.template-migration` invariant is governed only when reusable
template coverage passes, every template has a valid revision window, every
profile pins its template's current revision, and no migration or incompatibility
remains. Suppression is non-evidence.

Template revisions version policy compatibility, not governed document
identity. Git remains the history of document content. The migration command
advances binding metadata only; declaring that two revisions are compatible is
an explicit policy-owner decision and does not authorize implicit content
rewrites.
