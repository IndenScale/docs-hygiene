# Ubiquitous Language

Registry ID: `docs-hygiene`

Registry version: `0.2.0`

This registry defines the language used by Docs Hygiene product references,
requirements, diagnostics, tests, and adapters. IDs are stable semantic
identities; display names may be translated without changing the identity.

## Product Concepts

| ID | Standard term | Definition | Capability status |
| --- | --- | --- | --- |
| `DH-PRODUCT` | Docs Hygiene | The policy engine that checks repository intent surfaces locally and in CI. | Current |
| `DH-INTENT-CONTROL-PLANE` | Intent Control Plane | The authoritative references, requirements, decisions, acceptance criteria, and evidence relationships that constrain implementation. | Product direction |
| `DH-THREE-LAYER-MODEL` | Three-Layer Model | Product architecture organizing Bodies, Libraries, traceability, and Evidence across Intent, Definition, and Implementation. | Adopted |
| `DH-INTENT-LAYER` | Intent Layer | Layer whose Body is PRD and whose Reference Library is UL. | Adopted |
| `DH-DEFINITION-LAYER` | Definition Layer | Layer whose Body is Spec/Test Definition and whose Reference Library is Glossary. | Adopted |
| `DH-IMPLEMENTATION-LAYER` | Implementation Layer | Layer whose Body is Code/Configuration and whose Reference Library is the SDK that packages shared implementation primitives. | Adopted |
| `DH-REFERENCE-LIBRARY` | Reference Library | Reusable language, definition identities, or implementation primitives consumed by Bodies in one layer. | Adopted |
| `DH-GOVERNED-BODY` | Governed Body | A governed assertion expressing concrete intent, definition, or implementation in one layer. | Adopted |
| `DH-EVIDENCE-PLANE` | Evidence Plane | Observations of whether implementation satisfies definition and delivers intent benefit, rather than a fourth asset layer. | Adopted |
| `DH-EXECUTION-TRUTH` | Execution Truth | Current behavior demonstrated by code, configuration, tests, and runtime evidence. | Current |
| `DH-MANAGED-DOCUMENT` | Managed Document | A repository document owned by a configured docs base or document contract. | Current |
| `DH-DOCUMENT-CONTRACT` | Document Contract | A path-inferred, maturity-aware contract for required fields and semantic sections. | Current |
| `DH-CONCEPT-REFERENCE` | Concept Reference | A declared relationship from a managed document to a governed concept identity. | Current, filename-backed |
| `DH-SEMANTIC-CONTRACT` | Semantic Contract | A contract governing typed concepts, fixed references, local concepts, and semantic change proposals. | Product direction |
| `DH-TRACEABILITY-CONTRACT` | Traceability Contract | A contract requiring an intent relationship to reach acceptance and validation evidence. | Product direction |
| `DH-COGNITIVE-DEBT` | Cognitive Debt | Unresolved divergence or ambiguity across shared language, requirements, implementation behavior, metrics, and evidence. | Product direction |
| `DH-REVIEW-ITEM` | Semantic Review Item | A machine-identified semantic question that requires an accountable human decision rather than automatic resolution. | Product direction |

## Product Actions

| ID | Standard action | Successful result | Capability status |
| --- | --- | --- | --- |
| `CMD-CHECK-REPOSITORY-DOCS` | Check repository documentation | Return deterministic diagnostics for the configured policy surface. | Current |
| `CMD-INFER-DOCUMENT-CONTRACT` | Infer a document contract | Select the first matching profile from repository path and filename. | Current |
| `CMD-VALIDATE-CONCEPT-REFERENCE` | Validate a concept reference | Confirm that a highlighted concept has a corresponding concept definition. | Current |
| `CMD-ORCHESTRATE-ADAPTER` | Orchestrate an adapter | Run a configured external checker and normalize failure as a Docs Hygiene diagnostic. | Current |
| `CMD-VALIDATE-SEMANTIC-MANIFEST` | Validate a semantic manifest | Check typed and versioned UL references, local concepts, and change proposals. | Product direction |
| `CMD-GENERATE-REVIEW-QUEUE` | Generate a semantic review queue | Produce reproducible review items without deciding their business meaning. | Product direction |
| `CMD-VALIDATE-INTENT-TRACE` | Validate an intent trace | Check that governed intent reaches acceptance criteria and validation evidence. | Product direction |

## Invariants

1. Docs Hygiene must distinguish current capability from product direction.
2. Code and tests establish execution truth; intent documents do not claim that
   an implementation exists without executable evidence.
3. Shared concepts have one stable identity. A local refinement or competing
   meaning must be declared instead of entering prose anonymously.
4. Deterministic defects may block CI. Ambiguous semantic judgments become
   review items and are not decided by an LLM on behalf of the team.
5. Baselined intent uses fixed semantic versions. Historical intent must not
   drift when a registry changes.
6. Docs Hygiene governs intent contracts; it does not generate technical plans
   or prescribe an implementation task sequence.

## Results and Benefits

| ID | Standard result | Observable evidence |
| --- | --- | --- |
| `RESULT-POLICY-PASSED` | Policy passed | The CLI exits successfully with no blocking diagnostics. |
| `RESULT-POLICY-FAILED` | Policy failed | The CLI reports stable diagnostic codes, paths, and locations. |
| `RESULT-REVIEW-REQUIRED` | Semantic review required | A reproducible review item names the source, concept relation, and reason. |
| `BENEFIT-EARLY-DRIFT-DETECTION` | Early intent-drift detection | Broken or incomplete intent relationships are visible before implementation amplifies them. |
| `BENEFIT-REPLAYABLE-INTENT` | Replayable historical intent | A reviewer can resolve the exact concepts and acceptance meaning used by a baseline. |
| `BENEFIT-LAYERED-TRACEABILITY` | Layered traceability | A reviewer can locate broken relationships and missing evidence along the Body and Library axes. |

## Change Rules

1. A semantic change increments the registry version.
2. Renaming a display term does not create a new ID when its meaning is stable.
3. Splitting, merging, narrowing, or extending meaning requires a recorded
   relation and impact review.
4. Product requirements must pin this registry version and enumerate the
   governed concepts they consume.
5. Changes to three-layer relationships must review their impact on PRD,
   Glossary, Spec, and SDK projections together.
