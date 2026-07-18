# Product Position

This directory records the product model used to reason about Docs Hygiene.
Position documents explain the picture that guides terminology and requirements;
they do not replace the UL directory, PRDs, Glossary directory, or specifications.

## Documents

- [Cognitive Asset Governance Model](02_cognitive_asset_governance_model.md)
- [Three-Dimensional Software Documentation Profile](01_three_dimensional_governance_model.md)

## Change Routing

- A change to shared product meaning belongs in one term file under `docs/intent/ul/`.
- A change to desired product behavior belongs in an atomic member under `docs/intent/prd/`.
- A formal definition belongs in `docs/definition/`.
- A change to the explanatory model belongs here.
- A claim about shipped behavior must be checked against root-level code,
  configuration, and tests.
