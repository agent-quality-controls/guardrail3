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
- targeted Cargo verification is currently blocked by unrelated release-family compile drift already present in the tree, not by the hexarch lane itself

## Coverage matrix

Status key:
- `yes` = present in current tests
- `partial` = some coverage exists, but not to the lane standard
- `no` = missing

### Structural roots

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-01` | 33 | `rule_01.rs` has 45 tests | yes | yes | yes | yes | yes | partial | partial | converged under repeated 4-agent attack rounds; now proves owned outer hit set, broad missing/empty/file and symlink attacks, single-app isolation, discovery-scope ownership for newly discovered Rust apps, `packages/*` non-ownership, coexistence with `RS-HEXARCH-12`, Unicode/spaced app discovery, and that nested missing/file/empty/broken-symlink/symlink-loop `crates/` cases are not owned because rule 01 is app-root-only |
| `RS-HEXARCH-02` | 43 | `rule_02.rs` has 42 tests | yes | yes | yes | yes | yes | partial | partial | converged under repeated fresh 4-agent attack rounds; walker now preserves immediate symlink-child identity while still following links for discovery, so rule 02 again treats required child symlinks as invalid exact-contents entries instead of silently accepting them by name. The suite now covers valid/broken/non-dir child symlinks, outer `adapters/` symlink reachability, nested `.gitkeep` parity, `.gitignore` and loose `Cargo.toml` at the root, `.gitkeep/` as an unexpected directory, a maximally mixed single-root attack, a nested mixed attack with nested path exactness, all-four-missing-with-`.gitkeep`, non-owned nested-lookalike shapes under `packages/*`, and the final `.gitkeep`-symlink loophole where only a real `.gitkeep` file is exempt. |
| `RS-HEXARCH-03` | 19 | `rule_03.rs` has 48 tests | yes | yes | yes | yes | yes | partial | partial | converged under repeated fresh 4-agent attack rounds; facts now only materialize existing directional containers, so absent or file-replaced parent `adapters/` / `ports/` stay owned by `RS-HEXARCH-02` instead of false-positiving here. Directional child symlink identity now flows into rule 03, so symlinked `inbound/` / `outbound/` no longer satisfy the rule, and unexpected symlinked children are still reported as unexpected. The suite now covers broad adapters/ports parity, both-missing directional cases, broad unexpected-dir sweeps, deep unexpected-dir exactness, ownership boundaries, nested reachability, and directional child symlink cases. |
| `RS-HEXARCH-04` | 26 | `rule_04.rs` has 39 tests | yes | yes | yes | yes | yes | partial | partial | converged at the rule/test level under repeated fresh attack rounds; walker now preserves immediate ignored file children in already-discovered dirs so ignored loose files do not disappear from owned containers, and the suite now covers broad owned loose-file hits, multiple-file aggregation, near-miss placeholder files, replacement semantics, real-`.gitkeep` boundaries, symlinked `.gitkeep` non-exemption, mixed cross-rule ownership with `RS-HEXARCH-02/03/05`, missing-parent and destroyed-nested non-ownership, and TS/packages non-ownership. Remaining concern is only the broader shared-walker tradeoff where an ignored whole directory can still vanish before structural rules see it. |
| `RS-HEXARCH-05` | 17 | `rule_05.rs` has 39 tests | yes | yes | yes | yes | yes | partial | partial | converged at the rule/test level under repeated fresh attack rounds; container facts now preserve symlink-dir identity so rule 05 no longer treats symlinked child dirs as real subdirectories or materializes fake rule-06 leaves. The suite now covers broad empty-container sweeps, broad files-only sweeps, broad safe-container replacement parity, nested-only isolation, `.gitkeep` suppression, symlink-only and symlink-dir edges, symlinked `.gitkeep` non-exemption, missing-container non-ownership, and TS/packages non-ownership. Remaining concern is only the same shared-walker ignored-whole-directory tradeoff. |
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
| `RS-HEXARCH-13` | 2 | legacy `test_hex_arch_checks.rs` only | no | partial | yes | n/a | yes | no | no | moved to `*_tests/`; now proves forbidden direct dependency edges across a broader graph and out-of-tree path deps with layer-like names staying out of scope |
| `RS-HEXARCH-14` | 1 | legacy `test_hex_arch_checks.rs` only | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves exact inventory messages for multiple resolved path deps |
| `RS-HEXARCH-15` | 6 | legacy `test_hex_arch_checks.rs` only | yes | partial | yes | n/a | yes | yes | no | parse-error fail-closed added; now proves golden non-hit, single-app and all-app omission, and non-app non-hit |
| `RS-HEXARCH-16` | 1 | new rule, no old big suite | no | partial | yes | n/a | partial | no | no | moved to `*_tests/`; now proves layered patch/replace targets error while non-layered targets do not |
| `RS-HEXARCH-17` | 3 | new rule, no old big suite | no | partial | yes | n/a | yes | no | partial | moved to `*_tests/`; now proves forbidden inherited workspace edges across a broader graph, version-only inherited deps staying external, and inherited renamed path deps being owned once by rule 17 |
| `RS-HEXARCH-18` | 2 | new rule, no old big suite | no | partial | yes | n/a | yes | no | partial | moved to `*_tests/`; now proves forbidden renamed edges across a broader graph, external renamed deps without internal path resolution staying clean, and workspace-inherited renamed deps staying with rule 17 |
| `RS-HEXARCH-19` | 3 | new rule, no old big suite | no | partial | partial | n/a | partial | no | partial | moved to `*_tests/`; now proves one-hit same-layer cycle exactness, mixed-layer non-hit, and exact result shape; collector hardened so cycles with unlayered members no longer count as same-layer |
| `RS-HEXARCH-20` | 3 | new rule, no old big suite | no | partial | yes | n/a | yes | no | partial | moved to `*_tests/`; now proves forbidden dev edges warn while allowed dev edges do not, target dev edges stay with rule 25, and out-of-tree paths with layer-like names do not false-positive |
| `RS-HEXARCH-21` | 7 | legacy `test_hex_arch_checks.rs` + newer policy | no | partial | yes | n/a | yes | partial | no | moved to `*_tests/`; now proves dev-deps stay out, build-deps stay in, inherited workspace externals still error, workspace-inherited aliases resolve to real package names, and out-of-tree `domain` / `ports` path deps no longer fail open as fake pure internals; still needs broader allowlist/config breadth |
| `RS-HEXARCH-22` | 4 | new rule, no old big suite | yes | partial | no | n/a | partial | yes | no | collector fail-closed added; source analysis now descends into inline modules and follows only reachable module files from `lib.rs` / `main.rs`, so orphan files and `#[cfg(test)]` impls no longer perturb the rule; now proves balanced-count, DTO-only, private-trait, non-ports, multi-file aggregation, and unreachable/test-only non-hits |
| `RS-HEXARCH-23` | 6 | new rule, no old big suite | yes | partial | no | n/a | yes | yes | no | collector fail-closed added; source analysis now descends into inline modules and follows only reachable module files from `lib.rs` / `main.rs`, so orphan files and `#[cfg(test)]` public traits no longer perturb the rule; now proves golden non-hit, non-adapter non-hit, `pub(crate)` non-hit, nested-file hit, inline-module hit, and unreachable/test-only non-hits |
| `RS-HEXARCH-24` | 5 | new rule, no old big suite | yes | partial | yes | n/a | yes | no | no | moved to `*_tests/`; now proves cross-app leaks across dependency/dev/build/target sections, plus golden, `packages/` non-hits, and external same-name collisions staying out of scope |
| `RS-HEXARCH-25` | 3 | new rule, no old big suite | yes | partial | yes | n/a | yes | no | no | moved to `*_tests/`; now proves forbidden target edges across all three target table kinds, golden non-hit, and target-specific external same-name collisions staying out of scope |

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

1. Resume repeated fresh attack rounds at `RS-HEXARCH-06`, carrying forward the same “4 agents until convergence” protocol used earlier in the structural tranche.
2. Then continue dependency breadth for `RS-HEXARCH-14/16/19/24/25`, especially severity-exactness and remaining workspace/target ownership edges.
3. Keep the shared ProjectTree ignored-whole-directory blind spot in view as a cross-rule structural backlog item rather than misattributing it to one finished rule.

## Success condition

Every `RS-HEXARCH-*` rule has:
- golden coverage
- at least one broad attack-vector test
- exact owned hit/non-hit assertions
- folder-based test module if the rule is large
