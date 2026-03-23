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

- unreadable/unparsable source fail-open for the rest of the family beyond `RS-HEXARCH-22/23`
- malformed `guardrail3.toml` fail-open for the rest of the boundary-driven path beyond `RS-HEXARCH-15`
- direct proof that nested hex roots and top-level roots are attacked together
- `dependency_facts` still infers internal layers from raw path segments, which leaves a false-positive risk for out-of-tree path deps whose directory names happen to contain `domain`, `ports`, `app`, or `adapters`
- targeted Cargo verification is still blocked by crate-wide `-D dead-code` noise, including existing unused helpers in hexarch/release test support

## Coverage matrix

Status key:
- `yes` = present in current tests
- `partial` = some coverage exists, but not to the lane standard
- `no` = missing

### Structural roots

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-01` | 6 | `rule_01.rs` has 45 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves owned outer hit set, nested no-cascade behavior, and file-vs-dir replacement at both outer and nested `crates/` roots |
| `RS-HEXARCH-02` | 7 | `rule_02.rs` has 42 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves required-dir replacement, outer-adapters replacement, nested/outer `macros`, and root loose-file visibility; top-level files under `crates/` no longer evade the rule |
| `RS-HEXARCH-03` | 5 | `rule_03.rs` has 48 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves directional file-vs-dir replacement across owned roots and nested no-cascade behavior, but broader ports/adapters parity still needs expansion |
| `RS-HEXARCH-04` | 5 | `rule_04.rs` has 39 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves loose-file replacement semantics, `.gitkeep`-only non-hits, and nested-root `.gitkeep` boundaries, while keeping files-only containers owned by rule 05 |
| `RS-HEXARCH-05` | 6 | `rule_05.rs` has 39 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves empty-container replacement semantics, nested ownership, `.gitkeep` suppression, and exact 20-hit broad emptying across owned safe containers |
| `RS-HEXARCH-06` | 7 | `rule_06.rs` has 43 tests | yes | partial | yes | partial | partial | partial | no | moved to `*_tests/`; now proves valid nested hex leaves with `.gitkeep`, invalid placeholder variants, and non-owned `packages/` false-positive control |
| `RS-HEXARCH-12` | 4 | `rule_12.rs` has 9 tests | yes | partial | yes | n/a | partial | partial | no | moved to `*_tests/`; now proves broad all-app src/ attacks plus false-positive controls for file-named `src` and inner hex src |

### Workspace coverage

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-07` | 2 | `rule_07.rs` has 1 test | partial | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves broad all-app missing-member attack and ownership split with rule 08 |
| `RS-HEXARCH-08` | 2 | `rule_08.rs` has 1 test | partial | partial | yes | n/a | partial | yes | no | moved to `*_tests/`; now proves broad all-app workspace-policy and parse-error coverage |
| `RS-HEXARCH-09` | 1 | `rule_09.rs` has 1 test | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves broad all-app phantom-member coverage |
| `RS-HEXARCH-10` | 1 | `rule_10.rs` has 1 test | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves broad all-app outside-boundary member coverage |
| `RS-HEXARCH-11` | 2 | `rule_11.rs` has 1 test | partial | partial | yes | n/a | partial | yes | no | moved to `*_tests/`; now proves broad root-workspace app leakage and malformed-root fail-closed coverage |

### Dependency and boundary rules

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-13` | 1 | legacy `test_hex_arch_checks.rs` only | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves forbidden direct dependency edges across a broader graph |
| `RS-HEXARCH-14` | 1 | legacy `test_hex_arch_checks.rs` only | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves exact inventory messages for multiple resolved path deps |
| `RS-HEXARCH-15` | 6 | legacy `test_hex_arch_checks.rs` only | yes | partial | yes | n/a | yes | yes | no | parse-error fail-closed added; now proves golden non-hit, single-app and all-app omission, and non-app non-hit |
| `RS-HEXARCH-16` | 1 | new rule, no old big suite | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves layered patch/replace targets error while non-layered targets do not |
| `RS-HEXARCH-17` | 1 | new rule, no old big suite | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves forbidden inherited workspace edges across a broader graph |
| `RS-HEXARCH-18` | 1 | new rule, no old big suite | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves forbidden renamed edges across a broader graph |
| `RS-HEXARCH-19` | 3 | new rule, no old big suite | no | partial | partial | n/a | partial | no | partial | moved to `*_tests/`; now proves one-hit same-layer cycle exactness, mixed-layer non-hit, and exact result shape; collector hardened so cycles with unlayered members no longer count as same-layer |
| `RS-HEXARCH-20` | 1 | new rule, no old big suite | no | partial | yes | n/a | partial | no | partial | moved to `*_tests/`; now proves forbidden dev edges warn while allowed dev edges do not |
| `RS-HEXARCH-21` | 5 | legacy `test_hex_arch_checks.rs` + newer policy | no | partial | yes | n/a | partial | partial | no | moved to `*_tests/`; now proves dev-deps stay out, build-deps stay in, inherited workspace externals still error, and pure domain/ports path deps stay clean; still needs broader allowlist/config breadth and out-of-tree path classification hardening |
| `RS-HEXARCH-22` | 4 | new rule, no old big suite | yes | partial | no | n/a | partial | yes | no | collector fail-closed added; source analysis now descends into inline modules; now proves balanced-count, DTO-only, private-trait, non-ports, and multi-file aggregation cases, but still needs broader fixture-backed ownership breadth |
| `RS-HEXARCH-23` | 6 | new rule, no old big suite | yes | partial | no | n/a | yes | yes | no | collector fail-closed added; source analysis now descends into inline modules; now proves golden non-hit, non-adapter non-hit, `pub(crate)` non-hit, nested-file hit, and inline-module hit |
| `RS-HEXARCH-24` | 4 | new rule, no old big suite | yes | partial | yes | n/a | yes | no | no | moved to `*_tests/`; now proves cross-app leaks across dependency/dev/build/target sections, plus golden and `packages/` non-hits |
| `RS-HEXARCH-25` | 2 | new rule, no old big suite | yes | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves forbidden target edges across all three target table kinds and golden non-hit |

## Old-to-new mapping starting point

- `apps/guardrail3/tests/unit/rs_arch_01/rule_01.rs` -> `RS-HEXARCH-01`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_02.rs` -> `RS-HEXARCH-02`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_03.rs` -> `RS-HEXARCH-03`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_04.rs` -> `RS-HEXARCH-04`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_05.rs` -> `RS-HEXARCH-05`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_06.rs` -> `RS-HEXARCH-06`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_07.rs` -> `RS-HEXARCH-07`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_08.rs` -> `RS-HEXARCH-08`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_09.rs` -> `RS-HEXARCH-09`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_10.rs` -> `RS-HEXARCH-10`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_11.rs` -> `RS-HEXARCH-11`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_12.rs` -> `RS-HEXARCH-12`
- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs` -> legacy dependency/boundary behavior that now maps across `RS-HEXARCH-13..25`

## First implementation slice

1. Backfill old-corpus edge cases for `RS-HEXARCH-01..12` where breadth is still below the original suites.
2. Continue source-rule breadth for `RS-HEXARCH-22/23`, then policy/dependency breadth for `RS-HEXARCH-20/21`.
3. Add missing severity-exactness assertions across the family.

## Success condition

Every `RS-HEXARCH-*` rule has:
- golden coverage
- at least one broad attack-vector test
- exact owned hit/non-hit assertions
- folder-based test module if the rule is large
