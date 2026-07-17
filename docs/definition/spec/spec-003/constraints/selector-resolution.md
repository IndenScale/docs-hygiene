---
id: SPEC-003-C-011
status: proposed
---

# C-011 Heading Selector Resolution

A semantic Wiki Link may append an optional heading selector after its stable
target identity:

```text
[[ID#heading-slug]]
[[ID#heading-slug|label]]
[[ID#heading-slug@sha256:<64-hex>|label]]
```

The selector grammar is one or more lowercase ASCII alphanumeric words joined
by single hyphens. It resolves against level-one through level-six ATX Markdown
headings in the canonical governed target. Heading slugs lowercase ASCII
alphanumerics, collapse every intervening run of whitespace, punctuation, or
formatting characters to one hyphen, and trim separators. Fenced code does not
produce headings. The selected slug must occur exactly once; duplicate heading
slugs are ambiguous and invalid.

The normalized governance edge retains the selector independently from relation
kind and content anchor. A selector can therefore coexist with the existing
whole-file SHA-256 anchor; it does not change that anchor into a block hash.
Absent selectors preserve file-level reference behavior exactly.

An explicitly declared frontmatter anchor may instead use `scope: block` and
reuse the selector as its locator under
[C-013](scoped-content-anchors.md). Inline selector-plus-hash syntax remains
whole-file scoped for compatibility.

`DH_SELECTOR_001` reports a selector that cannot read its target or does not
resolve to an ATX heading. The diagnostic points to the Wiki Link source line
and relates the canonical target. Canonical and localized Body representations
must preserve the same ordered set of target identity, selector, and optional
content-hash signatures.

The governed `dependency.selector` invariant applies when at least one
normalized semantic edge carries a selector. It passes only when the
`governance.identity` checker reports no selector diagnostic; disabled or
suppressed execution remains non-evidence. Line ranges, natural-language
fragments, and cross-project addressing are outside this constraint.
Block-scoped hashing is defined separately by C-013.
