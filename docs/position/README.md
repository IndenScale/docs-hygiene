# Product Position

This directory records the product model used to reason about Docs Hygiene.
Position documents explain the picture that guides terminology and requirements;
they do not replace the UL directory, PRDs, Glossary directory, specifications,
or implementation evidence.

## Documents

- [Reference and Subject Across Three Layers](01_reference_and_subject_across_three_layers.md)

## Change Routing

- A change to shared product meaning belongs in one term file under `docs/intent/ul/`.
- A change to desired product behavior belongs in an atomic member under `docs/intent/prd/`.
- A formal definition belongs in `docs/definition/`.
- A change to the explanatory model belongs here.
- A claim about shipped behavior must be checked against root-level code,
  configuration, tests, and runtime evidence.
