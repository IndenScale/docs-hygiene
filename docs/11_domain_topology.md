# Domain Topology and Fan-out

Docs Hygiene models UL and Glossary as recursive Library Domain trees. The
tree makes semantic boundaries explicit without asking the checker to infer
meaning from prose.

## Domain

A Domain is a stable semantic boundary inside a Library. A Domain directory
has a `manifest.yml` with `kind: domain`, a stable `id`, lifecycle `status`, and
a non-empty list of direct `members`. A root UL or Glossary manifest is the
authority for its complete Domain tree even though the root does not repeat
`kind: domain`.

## Sub Domain

A Sub Domain is a Domain declared as one direct child of another Domain. It is
the same governed node type, not another refinement level or reference
relation. Introduce one when repository evidence supports a stable boundary,
such as terminology clusters, ownership, or distinct downstream projections.

## Fan-out Budget

Fan-out counts one Domain's direct members only. Each Markdown leaf and child
Domain counts as one; descendants below a child Domain do not count toward the
parent. Checks run on the canonical UL and Glossary trees, while localized
representations preserve the same topology without duplicate diagnostics.

Defaults are inclusive:

- 0–14 direct members: no diagnostic;
- 15–49 direct members: `DH_DOMAIN_001` warning;
- 50 or more direct members: `DH_DOMAIN_001` error.

Only the highest applicable diagnostic is emitted for one Domain. The count is
a structural reviewability signal, not proof of a semantic grouping. Tools and
Agents may propose Sub Domains, but must not move members or invent identities
from the count alone.

## Configuration

```yaml
governance:
  domainFanout:
    warningAt: 15
    errorAt: 50
```

`warningAt` must be at least 1 and `errorAt` must be greater than `warningAt`.
Override the values for a project-specific review budget. Disable the checker
when a deliberately flat Library uses a different governance mechanism:

```yaml
rules:
  governance.domain-fanout:
    mode: disabled
```

Structural errors such as missing manifests, unsafe paths, duplicate identity,
or undeclared children remain independently enforced by `DH_LIBRARY_001`.
