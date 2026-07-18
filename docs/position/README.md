# Product Position

This directory records the product model used to reason about Docs Hygiene.
Position documents explain the picture that guides terminology and requirements;
they do not replace UL, PRDs, Issues, or Artifact evidence.

## Documents

- [Cognitive Asset Governance Model](02_cognitive_asset_governance_model.md)
- [Open Engineering Asset Model](01_open_engineering_asset_model.md)

## Change Routing

- A change to shared product meaning belongs in one term file under `docs/engineering/ul/`.
- A change to desired product behavior belongs in an atomic member under `docs/engineering/prd/`.
- Change-scoped acceptance and evidence belong in an Issue.
- A change to the explanatory model belongs here.
- A claim about shipped behavior must be checked against root-level code,
  configuration, and tests.
