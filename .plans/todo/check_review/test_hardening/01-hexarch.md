# Hexarch Hardening Lane

## Why first

`hexarch` is the riskiest family:
- large structural surface
- nested roots
- dependency edges
- old deliberate test corpus
- known incompleteness in migrated test depth

## Deliverables

1. Convert heavy rule tests to folder-based modules where needed.
2. Build a rule-by-rule attack matrix for `RS-HEXARCH-*`.
3. Port old adversarial ideas by attack class, not by raw file count.
4. Add exact-set assertions for multi-root and nested-root behavior.

## Priority rule groups

### Structural roots
- `RS-HEXARCH-01..06`
- attack classes:
  - golden
  - missing required dirs across all Rust hex roots
  - unexpected siblings across all Rust hex roots
  - optional `macros/`
  - nested root parity
  - false positives against non-owned roots

### Workspace coverage
- `RS-HEXARCH-07..11`
- attack classes:
  - missing members everywhere
  - extra members everywhere
  - out-of-boundary members
  - malformed Cargo.toml fail-closed

### Dependency/boundary rules
- `RS-HEXARCH-13..25`
- attack classes:
  - all illegal edge permutations per direction
  - renamed deps
  - inherited workspace deps
  - target/dev edges
  - cross-app leaks
  - malformed boundary config fail-closed

## Explicit gaps to close

- unreadable/unparsable source fail-open
- malformed `guardrail3.toml` fail-open
- direct proof that nested hex roots and top-level roots are attacked together

## Success condition

Every `RS-HEXARCH-*` rule has:
- golden coverage
- at least one broad attack-vector test
- exact owned hit/non-hit assertions
- folder-based test module if the rule is large
