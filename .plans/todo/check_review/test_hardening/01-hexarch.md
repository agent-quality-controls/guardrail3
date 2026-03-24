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
| `RS-HEXARCH-06` | 23 | `rule_06.rs` has 43 tests | yes | yes | yes | yes | yes | partial | partial | converged at the rule/test level under repeated fresh attack rounds; walker now recursively scans newly recovered ignored directories so ignored invalid leaves no longer disappear and ignored valid leaves no longer misclassify. The suite now covers broad invalid-leaf and hybrid sweeps with exact counts, valid nested hex and placeholder variants, files-only invalid leaves, symlink-leaf non-ownership, inner-only ownership, ignored-leaf branch parity across invalid/valid/hybrid states, permission-denied parity, destroyed-parent nested non-ownership, and TS/packages non-ownership. |
| `RS-HEXARCH-12` | 4 | `rule_12.rs` has 9 tests | yes | partial | yes | n/a | partial | partial | no | moved to `*_tests/`; now proves broad all-app src/ attacks plus false-positive controls for file-named `src` and inner hex src |

### Workspace coverage

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-07` | 20 | `rule_07.rs` has 1 test | yes | yes | yes | n/a | yes | partial | partial | converged under repeated fresh 4-agent attack rounds. Real bugs fixed: discovered workspace crates now come from owned hex leaves instead of any nested `Cargo.toml`, and workspace-member matching now uses semantic resolution for normalized/glob members instead of raw string equality. Shared workspace-member facts also now preserve `.` / `./` / `""` app-root members and reject absolute members from the app-boundary path so they cannot fail open. The sidecar now covers golden, one-app multiplicity with per-crate attribution, nested-inner missing-member ownership, normalized/glob semantics for both outer and nested-inner paths, nested Cargo projects inside real leaves staying out of scope, `packages/*` and non-Rust app non-ownership, and cross-rule ownership splits against `RS-HEXARCH-08/09/10`. |
| `RS-HEXARCH-08` | 4 | `rule_08.rs` has 1 test | yes | partial | yes | n/a | partial | yes | partial | converged under repeated fresh 4-agent attack rounds. Rule wording now says “invalid workspace config” rather than implying TOML syntax failure only, and the sidecar now includes a realistic package-style manifest rewrite built from the golden fixture’s real workspace manifest, a one-app package-style ownership case, and non-string `[workspace].members[...]` fail-closed ownership. |
| `RS-HEXARCH-09` | 2 | `rule_09.rs` has 1 test | yes | partial | yes | n/a | yes | no | partial | converged under repeated fresh 4-agent attack rounds. The main remaining work was ownership realism: tests now use the real `packages/shared-types` escape target instead of synthetic paths, and rule 09 explicitly stays out of ownership for absolute members that belong to `RS-HEXARCH-10`. |
| `RS-HEXARCH-10` | 8 | `rule_10.rs` has 1 test | yes | yes | yes | n/a | yes | no | partial | converged under repeated fresh 4-agent attack rounds. The shared workspace-member facts layer now preserves app-root members and rejects absolute members as out-of-boundary instead of normalizing them away. The sidecar now pins app-root, package-escape, sibling-app (`../backend/...`), glob, realistic absolute-path, and same-app reentry cases, with explicit ownership separation from `RS-HEXARCH-07/08/09`. |
| `RS-HEXARCH-11` | 7 | `rule_11.rs` has 1 test | partial | yes | yes | n/a | yes | yes | partial | materially hardened but still awaiting the broad family rerun as the final convergence signal. Real bug fixed: root workspace members are now resolved semantically before ownership, so normalized, absolute, app-subpath, and glob members no longer evade `RS-HEXARCH-11`. The sidecar now covers normalized app-member, absolute app-member, app-subpath, `apps/*` glob, normalized `packages/*` non-hit, and malformed-root fail-closed ownership. |

### Dependency and boundary rules

| Rule | Current tests | Old corpus signal | Golden | Broad attack | Multi-root | Nested-root | False-positive | Fail-closed | Severity exact | Notes |
|------|---------------|-------------------|--------|--------------|------------|-------------|----------------|-------------|----------------|-------|
| `RS-HEXARCH-13` | 6 | legacy `test_hex_arch_checks.rs` only | yes | yes | yes | n/a | yes | partial | partial | materially hardened under repeated attack rounds. Real bugs fixed: cross-app normal path deps now stay with `RS-HEXARCH-24`, and same-app omitted internal targets no longer fail open just because they are missing from workspace membership. The sidecar now covers golden, out-of-tree path lookalikes, cross-rule ownership splits with `RS-HEXARCH-18/24`, and exact broad graph attribution. |
| `RS-HEXARCH-14` | 2 | legacy `test_hex_arch_checks.rs` only | partial | yes | yes | n/a | yes | no | yes | materially hardened. Real bug fixed: inventory no longer fabricates “resolved” entries for nonexistent paths. The sidecar now proves exact inventory messages for resolved normal/dev/build/target path deps and a broken-path non-inventory non-hit. |
| `RS-HEXARCH-15` | 6 | legacy `test_hex_arch_checks.rs` only | yes | partial | yes | n/a | yes | yes | no | parse-error fail-closed added; now proves golden non-hit, single-app and all-app omission, and non-app non-hit |
| `RS-HEXARCH-16` | 1 | new rule, no old big suite | partial | partial | yes | n/a | yes | no | partial | materially hardened. Real bug fixed: layered-looking patch/replace paths now count only when they resolve to a real crate target, so nonexistent path strings no longer false-positive. |
| `RS-HEXARCH-17` | 5 | new rule, no old big suite | partial | yes | yes | n/a | yes | no | partial | materially hardened. Real bugs fixed: inherited same-app path deps now resolve relative to the workspace root, and inherited cross-app path deps stay with `RS-HEXARCH-24`. The sidecar now also pins version-only non-hits, renamed ownership against `RS-HEXARCH-18`, and allowed renamed inherited paths. |
| `RS-HEXARCH-18` | 10 | new rule, no old big suite | partial | yes | yes | n/a | yes | no | yes | materially hardened. Real bugs fixed: only real target-backed renamed edges count, and cross-app renamed target deps now stay with `RS-HEXARCH-24`. The sidecar now covers realistic fixture-backed same-app renamed violations, allowed renamed same-app non-hits, workspace-inherited renamed ownership, renamed dev/target ownership splits with `RS-HEXARCH-20/25`, broken same-app targets, omitted-member seams, and exact alias/package message semantics. |
| `RS-HEXARCH-19` | 6 | new rule, no old big suite | yes | yes | partial | n/a | yes | no | yes | materially hardened. Real collector bug fixed: target-specific edges no longer participate in same-layer cycle detection. The sidecar now covers golden non-hit, fixture-backed same-layer cycle reporting, omitted-member non-cycles, target-specific non-cycles, mixed-layer filtering, and exact result-count/title semantics. |
| `RS-HEXARCH-20` | 7 | new rule, no old big suite | yes | yes | yes | n/a | yes | no | yes | materially hardened. Real bugs fixed: dev-direction warnings now require a real target and stay out of cross-app ownership, leaving `RS-HEXARCH-24` to own cross-app dev deps. The sidecar now covers golden, exact warning messages, renamed dev ownership against `RS-HEXARCH-18`, inherited workspace-dev ownership against `RS-HEXARCH-17`, broken same-app target non-hits, and cross-app dev ownership. |
| `RS-HEXARCH-21` | 14 | legacy `test_hex_arch_checks.rs` + newer policy | yes | yes | yes | n/a | yes | yes | partial | materially hardened. Real bug fixed: same-app `domain/` and `ports/` paths are now treated as allowed only when they resolve to discovered workspace members; omitted pure-layer crates now error as non-workspace pure-layer targets instead of failing open. The rule also now stays out of cross-app ownership for `RS-HEXARCH-24`. The sidecar now covers golden, build and target deps in scope, dev deps out of scope, inherited external/alias behavior, cross-app non-ownership, real backend ports allow-case, omitted pure-layer members, and out-of-tree pure-looking paths. |
| `RS-HEXARCH-22` | 14 | new rule, no old big suite | yes | yes | no | n/a | yes | yes | partial | materially hardened. Shared source discovery now fails closed when `src/lib.rs` / `src/main.rs` is missing instead of scanning orphan root files as fake entrypoints. The sidecar now covers golden, fail-closed parse and missing-entrypoint precedence, balanced counts, DTO-only and private-trait non-hits, multi-file aggregation, reachable-module filtering, non-ports scope, inline modules, and nested file layouts. |
| `RS-HEXARCH-23` | 12 | new rule, no old big suite | yes | yes | no | n/a | yes | yes | partial | materially hardened. Shared source discovery hardening now also applies here, so missing entrypoints fail closed instead of scanning orphan root files. The sidecar now covers golden, fail-closed parse and missing-entrypoint precedence, non-adapter scope, `pub(crate)` / `pub(super)` non-hits, inline-module and nested-file hits, and unreachable/test-only public-trait non-hits. |
| `RS-HEXARCH-24` | 5 | new rule, no old big suite | yes | yes | yes | n/a | yes | no | partial | materially hardened. Real bug fixed: cross-app boundary errors now require a real resolved target, so broken cross-app paths no longer false-positive. The sidecar covers golden, broad cross-app leaks across normal/dev/build/target sections, `packages/` non-hits, external same-name collisions, and broken-target non-hits. |
| `RS-HEXARCH-25` | 5 | new rule, no old big suite | yes | yes | yes | n/a | yes | no | partial | materially hardened. Real bugs fixed: target-direction errors now require a real resolved target and stay out of cross-app ownership, leaving `RS-HEXARCH-24` to own cross-app target deps. The sidecar now covers golden, forbidden target edges across all target table kinds, external same-name non-hits, broken same-app target-path non-hits, and cross-app ownership boundaries. |

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

1. Finish the broad `rs_hexarch_` family rerun and use any failures to determine which of `RS-HEXARCH-11..25` still need another adversarial round.
2. Update `11-hexarch-agent-brief.md` to reflect the post-`25` state instead of the stale “stop at 11” snapshot.
3. Keep the shared ProjectTree ignored-whole-directory blind spot in view as a cross-rule structural backlog item rather than misattributing it to one finished rule.

## Success condition

Every `RS-HEXARCH-*` rule has:
- golden coverage
- at least one broad attack-vector test
- exact owned hit/non-hit assertions
- folder-based test module if the rule is large
