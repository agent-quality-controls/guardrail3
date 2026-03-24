# Hexarch Agent Brief

You own the `rs/hexarch` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/01-hexarch.md`
6. `.plans/todo/checks/rs/hexarch.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`

## Current status

Already done in this lane:
- coverage matrix and old-to-new rule mapping live in `01-hexarch.md`
- exhaustive execution order lives in `16-hexarch-execution-plan.md`
- `RS-HEXARCH-15` no longer fails open on malformed `guardrail3.toml`
- `RS-HEXARCH-22/23` no longer fail open on unreadable/unparsable Rust source
- source collection for `RS-HEXARCH-22/23` now descends into inline `mod { ... }` blocks, so hidden inline traits/impls no longer evade source-based checks
- source collection for `RS-HEXARCH-22/23` now follows the reachable module graph from `src/lib.rs` / `src/main.rs` and skips `#[cfg(test)]` items, so orphan `.rs` files and test-only traits/impls no longer distort source-rule results
- normal dependency edges no longer infer internal layer/app ownership from raw path segments alone, so out-of-tree paths with names like `domain`, `ports`, `app`, or `adapters` no longer masquerade as owned hex members
- dependency-direction ownership now requires actual path-backed member resolution rather than bare same-name fallback, so external/version-only same-name crates no longer masquerade as internal edges
- `RS-HEXARCH-01..25` now use rule-specific `*_tests/` directories
- `RS-HEXARCH-01..25` now have at least one stronger exact-hit or graph-shaped attack test instead of only loose “some result exists” checks
- `RS-HEXARCH-13` now proves out-of-tree path deps with layer-like names do not false-positive as owned direction edges
- `RS-HEXARCH-17` now proves version-only inherited deps do not masquerade as internal edges, and inherited renamed path deps are owned by rule 17
- `RS-HEXARCH-18` now excludes workspace-inherited renamed deps so it no longer double-reports rule-17 ownership
- `RS-HEXARCH-20` now proves forbidden direct dev edges warn, target dev edges stay owned by `RS-HEXARCH-25`, and out-of-tree path deps with layer-like names do not false-positive
- `RS-HEXARCH-21` now proves dev-deps stay out of scope, build-deps stay in scope, inherited workspace externals are checked, workspace-inherited aliases resolve to real package names, and out-of-tree `domain` / `ports` path deps do not fail open as fake pure internals
- `RS-HEXARCH-22` now proves balanced counts, DTO-only ports crates, non-ports crates, private-trait ports crates, and multi-file aggregation
- `RS-HEXARCH-22` now also proves unreachable orphan files and test-only impls do not count toward trait dominance
- `RS-HEXARCH-23` now proves pristine golden non-hit, non-adapter non-hit, `pub(crate)` non-hit, nested-file hit, inline-module hit, fail-closed parse errors, and unreachable/test-only public traits staying out of scope
- `RS-HEXARCH-15` now proves golden non-hit, single-app omission, all-app omission, non-app non-hit, and parse-error fail-closed ownership
- `RS-HEXARCH-19` now proves one-hit same-layer cycle exactness, mixed-layer non-hit, exact result shape, and the collector no longer misreports cycles containing an unlayered member as same-layer
- `RS-HEXARCH-24` now proves cross-app leaks across normal/dev/build/target dependency sections, plus golden and `packages/` non-hits
- `RS-HEXARCH-25` now proves forbidden target edges across `target.dependencies`, `target.dev-dependencies`, and `target.build-dependencies`, plus golden non-hit
- `RS-HEXARCH-01` is now documented and tested as app-root-only; nested hex-root `crates/` absence is explicitly not owned by rule 01
- `RS-HEXARCH-01..03` now cover old replacement-shaped attacks, not just directory removal
- `RS-HEXARCH-02` now sees top-level files under owned `crates/` roots, so stray root files can no longer evade the exact-contents rule
- `RS-HEXARCH-04..06` now have much deeper old-corpus coverage around file replacement, `.gitkeep` boundaries, nested ownership, and valid-vs-invalid placeholder variants
- `cargo test --manifest-path apps/guardrail3/Cargo.toml rs_hexarch_ -- --nocapture` is currently green (`115 passed`) after the recent dependency-ownership and stale-test fixes

Current active tranche:
- explicit adversarial protocol across `RS-HEXARCH-01..25`: 4 adversarial passes per rule, repeated until a fresh round stops finding meaningful hardening work

Next tranche after that:
- severity-exactness and remaining false-positive/fail-closed backfill where still missing

## Attack protocol

This lane is now using the literal exhaustive protocol requested by the user:
- every `RS-HEXARCH-*` rule must get repeated fresh attack rounds until it converges
- each round must use 4 adversarial agents
- agents must understand rule intent first, not just literal code behavior
- the purpose is to find edge cases and rule bugs, not to confirm that the rule passes

Per rule:
1. round 1: launch 4 agents with split angles
   - intent vs implementation
   - missing scenarios / old-corpus parity
   - false positives / ownership boundaries
   - fixture and mutation realism
2. patch the real bugs or test gaps found
3. repeat fresh 4-agent rounds against the updated rule until the newest round stops producing meaningful hardening improvements
4. only then mark the rule done and move to the next one

Current status:
- `RS-HEXARCH-01`: complete under repeated fresh 4-agent attack rounds; remaining issue is only wording quality (`missing crates/ directory` also covers empty/unusable cases)
- `RS-HEXARCH-02`: complete under repeated fresh 4-agent attack rounds. Two real rule bugs were fixed during convergence:
  - post-`follow_links(true)` required child symlinks were silently accepted by name until `ProjectTree` started preserving immediate symlink-child identity for facts/rules
  - a symlink named `.gitkeep` was incorrectly exempted until rule 02 started allowing only a real `.gitkeep` file, not symlinked `.gitkeep`
  The suite is now at 43 tests and covers broad child-symlink parity, outer `adapters/` special-case ownership, nested compound exactness, loose `Cargo.toml` and `.gitignore` root files, `.gitkeep/` directory behavior, mixed outer and nested attacks, all-four-missing with `.gitkeep`, and non-owned nested-lookalike `packages/*` shapes. Fresh agents now say the rule looks converged.
- `RS-HEXARCH-03`: complete under repeated fresh 4-agent attack rounds. Two real rule/facts bugs were fixed during convergence:
  - directional facts were materializing absent parent `adapters/` / `ports/` containers, causing rule 03 to over-own cases that belong to rule 02
  - directional child symlink identity was being dropped, so symlinked `inbound/` / `outbound/` could pass by name, and an unexpected symlinked child could be silently skipped
  The suite is now at 19 tests and covers broad adapters/ports parity, both-missing directional cases, broad unexpected-dir sweeps, deep unexpected-dir exactness, ownership boundaries, nested reachability, and directional child symlink cases. Fresh agents now say the rule looks converged.
- `RS-HEXARCH-04`: complete under repeated fresh 4-agent attack rounds. One real shared-snapshot bug was fixed during convergence:
  - immediate ignored loose files inside already-discovered dirs could disappear before structural rules saw them, until `ProjectTree` started patching immediate raw file children instead of only symlinks
  The sidecar is now at 26 tests and covers broad owned loose-file attacks, multiple-file aggregation, near-miss placeholder files, replacement semantics, real-`.gitkeep` boundaries, symlinked `.gitkeep` non-exemption, mixed cross-rule ownership with `RS-HEXARCH-02/03/05`, missing-parent and destroyed-nested non-ownership, and TS/packages non-ownership. Fresh agents now say the only remaining concern is the broader shared-walker ignored-whole-directory tradeoff, not a rule-04-specific bug.
- `RS-HEXARCH-05`: complete under repeated fresh 4-agent attack rounds. Two real rule/facts bugs were fixed during convergence:
  - symlink-only containers could be misdescribed as `is empty` until rule 05 started building its `contains files (...)` detail from both real files and symlink files
  - child directory symlinks were still counting as real subdirectories until container facts/inputs preserved `symlink_dirs`, rule 05 excluded them from the “has real dirs” check, and leaf collection stopped materializing fake rule-06 leaves for symlinked child dirs
  The sidecar is now at 17 tests and covers broad empty-container sweeps, broad files-only sweeps, broad safe-container replacement parity, nested-only isolation, `.gitkeep` suppression, symlink-only and symlink-dir edges, symlinked `.gitkeep` non-exemption, missing-container non-ownership, and TS/packages non-ownership. Fresh agents now say the only remaining concern is the same shared-walker ignored-whole-directory tradeoff, not a rule-05-specific bug.
- `RS-HEXARCH-06`: complete under repeated fresh 4-agent attack rounds. One final shared-snapshot bug was fixed during convergence:
  - newly recovered ignored directories were visible as dirs but were not recursively scanned in the same patch pass, so ignored valid leaves could still misclassify until `ProjectTree` started recursively patching newly recovered ignored dirs
  The sidecar is now at 23 tests and covers broad invalid-leaf and hybrid sweeps with exact counts, valid nested hex and placeholder variants, files-only invalid leaves, symlink-leaf non-ownership, inner-only ownership, ignored-leaf branch parity across invalid/valid/hybrid states, permission-denied parity, destroyed-parent nested non-ownership, and TS/packages non-ownership. Fresh agents now say the rule is effectively converged.
- `RS-HEXARCH-07`: complete under repeated fresh 4-agent attack rounds. Round 1 found two real implementation bugs and several depth gaps:
  - rule 07 was comparing raw member strings to discovered crate dirs, so valid workspace semantics like `./...`, `../` normalization, trailing slashes, and globs would false-positive as missing members
  - discovered workspace crates were being found by a raw `Cargo.toml under crates/` scan, so nested Cargo projects inside a real crate leaf could be misclassified as required workspace members
  The fix derives discovered crate dirs from actual hex leaf facts instead of raw tree scan, adds semantic member resolution for normalized/glob member entries, and makes rule 07 skip non-workspace app Cargo so ownership stays with `RS-HEXARCH-08`.
  Round 2 found a remaining family mismatch: `RS-HEXARCH-07` had become semantically stricter than `RS-HEXARCH-09/10`, so valid normalized/glob internal members could still false-positive under the neighboring rules. The fix upgrades workspace member facts to keep raw + normalized + resolved semantics and aligns `RS-HEXARCH-07/09/10` on that shared interpretation.
  Round 3 found two more fail-open gaps in that shared member interpretation:
  - workspace members `.` and `./` were normalizing away before `RS-HEXARCH-10` could own them, so invalid app-root members disappeared entirely
  - absolute workspace members like `/crates/domain/types` were being normalized as if they were internal relative members, so they could silently pass as valid coverage
  Those cases are now fixed in the shared workspace-member facts layer and covered by rule-10 ownership/path-resolution tests. The sidecar now covers golden, one-app multiplicity with per-crate attribution, nested-inner missing-member ownership, normalized/glob semantics for both outer and nested-inner paths, nested Cargo projects inside real leaves staying out of scope, `packages/*` and non-Rust app non-ownership, and mixed ownership-boundary splits across `RS-HEXARCH-07/09/10`. Fresh agents stopped finding meaningful further gaps.
- `RS-HEXARCH-08`: complete under repeated fresh 4-agent attack rounds. The rule now owns semantic-invalid-workspace config as well as package-style app manifests, with wording updated to “invalid workspace config” instead of implying TOML syntax failure only. The sidecar now includes a realistic package-style manifest rewrite built from the golden fixture’s real workspace manifest, a one-app package-style ownership case, and non-string `[workspace].members[...]` fail-closed ownership.
- `RS-HEXARCH-09`: complete under repeated fresh 4-agent attack rounds. The main depth work was ownership realism rather than a checker bug: tests now use the real `packages/shared-types` escape target instead of synthetic paths, and rule 09 explicitly stays out of ownership for absolute root-escaped members that belong to `RS-HEXARCH-10`.
- `RS-HEXARCH-10`: complete under repeated fresh 4-agent attack rounds. The shared workspace-member facts layer now preserves app-root members (`.`, `./`, and `""`) and rejects absolute members as out-of-boundary instead of normalizing them away. The sidecar now also pins the sibling-app seam (`../backend/crates/domain/engine`) plus realistic absolute-path mutations against the copied fixture’s real temp path, so rule 10 owns cross-app, package-escape, app-root, glob, and absolute-member boundary violations while `RS-HEXARCH-07/09` stay clean.
- `RS-HEXARCH-11`: materially hardened, pending final family-wide rerun. Real semantic bug fixed: the rule no longer compares raw root workspace member strings only, so normalized, glob, app-subpath, and absolute app-root members can no longer evade ownership. The sidecar now covers normalized app-member, absolute app-member, app-subpath, `apps/*` glob, and normalized `packages/*` non-hit coverage.
- `RS-HEXARCH-12`: materially hardened. The sidecar now covers the golden non-hit, all-app and one-app `src/` ownership, TS-app non-ownership, file-named `src` non-hit, inner-hex non-hit using a real nested path, and non-Rust-file positives under `src/`.
- `RS-HEXARCH-13`: materially hardened. Cross-app path deps now stay with `RS-HEXARCH-24`, omitted same-app internal targets no longer fail open when omitted from workspace membership, and the sidecar now covers golden, out-of-tree path lookalikes, broad exact attribution, and ownership seams against `RS-HEXARCH-18/24`.
- `RS-HEXARCH-14`: materially hardened. Inventory no longer fabricates resolved entries for nonexistent paths, and the sidecar now proves exact multi-section inventory messages plus broken-path non-inventory non-hits.
- `RS-HEXARCH-16`: materially hardened. Layered-looking patch/replace targets now count only when they resolve to a real crate target, so nonexistent patch paths no longer false-positive.
- `RS-HEXARCH-17`: materially hardened. Inherited same-app path deps now resolve relative to workspace root, inherited cross-app deps stay with `RS-HEXARCH-24`, and the sidecar covers version-only non-hits, renamed ownership against `RS-HEXARCH-18`, and allowed renamed inherited paths.
- `RS-HEXARCH-18`: materially hardened. Only real target-backed renamed edges now count, cross-app renamed target deps stay with `RS-HEXARCH-24`, and the sidecar covers realistic fixture-backed same-app renamed violations, allowed renamed same-app non-hits, workspace-inherited renamed ownership, renamed dev/target ownership splits with `RS-HEXARCH-20/25`, broken same-app targets, omitted-member seams, and exact alias/package message semantics.
- `RS-HEXARCH-19`: materially hardened. Same-layer cycle detection no longer includes target-specific edges, and the sidecar now covers golden non-hit, fixture-backed same-layer cycle reporting, omitted-member non-cycles, target-specific non-cycles, mixed-layer filtering, and exact result-count/title semantics.
- `RS-HEXARCH-20`: materially hardened. Dev-direction warnings now require a real target and stay out of cross-app ownership, leaving `RS-HEXARCH-24` to own cross-app dev deps. The sidecar now covers golden, exact warning messages, renamed dev ownership against `RS-HEXARCH-18`, inherited workspace-dev ownership against `RS-HEXARCH-17`, broken same-app target non-hits, and cross-app dev ownership.
- `RS-HEXARCH-21`: materially hardened. Same-app `domain/` and `ports/` paths are now treated as allowed only when they resolve to discovered workspace members; omitted pure-layer crates error as non-workspace pure-layer targets instead of failing open. The rule also now stays out of cross-app ownership for `RS-HEXARCH-24`. The sidecar now covers golden, build and target deps in scope, dev deps out of scope, inherited external/alias behavior, cross-app non-ownership, a real backend `domain -> ports/outbound/repo` allow-case, omitted pure-layer members, and out-of-tree pure-looking paths.
- `RS-HEXARCH-22`: materially hardened. Shared source discovery now fails closed when `src/lib.rs` / `src/main.rs` is missing instead of scanning orphan root files as fake entrypoints. The sidecar now covers golden, fail-closed parse and missing-entrypoint precedence, balanced counts, DTO-only and private-trait non-hits, multi-file aggregation, reachable-module filtering, non-ports scope, inline modules, and nested file layouts.
- `RS-HEXARCH-23`: materially hardened. Shared source discovery hardening now also applies here, so missing entrypoints fail closed instead of scanning orphan root files. The sidecar now covers golden, fail-closed parse and missing-entrypoint precedence, non-adapter scope, `pub(crate)` / `pub(super)` non-hits, inline-module and nested-file hits, and unreachable/test-only public-trait non-hits.
- `RS-HEXARCH-24`: materially hardened. Cross-app boundary errors now require a real resolved target, so broken cross-app paths no longer false-positive. The sidecar covers golden, broad cross-app leaks across normal/dev/build/target sections, `packages/` non-hits, external same-name collisions, and broken-target non-hits.
- `RS-HEXARCH-25`: materially hardened. Target-direction errors now require a real resolved target and stay out of cross-app ownership, leaving `RS-HEXARCH-24` to own cross-app target deps. The sidecar covers golden, forbidden target edges across all target table kinds, external same-name non-hits, broken same-app target-path non-hits, and cross-app ownership boundaries.

## Current uncommitted file set

The current live work is not committed yet. The key files are:

- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs`
  - `WorkspaceCoverageFacts` is being upgraded from raw member strings to semantic workspace-member facts
  - discovered workspace crates now come from owned hex leaves, not any arbitrary nested `Cargo.toml`
  - member normalization / resolution logic is being centralized here
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/inputs.rs`
  - workspace coverage input is being updated to carry semantic member facts into rules
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_07_workspace_members_match_crate_dirs.rs`
  - now skips non-workspace apps
  - now matches discovered crate dirs against resolved member semantics instead of raw strings
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_09_no_extra_workspace_members.rs`
  - in-flight alignment to semantic workspace-member facts so valid normalized/glob internal members do not false-positive as phantom members
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_10_members_within_app_boundary.rs`
  - in-flight alignment to semantic workspace-member facts so valid normalized internal members do not false-positive as out-of-boundary
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_07_workspace_members_match_crate_dirs_tests/`
  - widened rule-07 sidecar with golden, path-resolution, discovery-boundary, multiplicity, nested-inner, and ownership-split coverage
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`

## External blockers right now

Targeted Cargo reruns are not giving a clean signal yet because the dirty tree has unrelated compile blockers outside hexarch:

- missing hooks modules from `apps/guardrail3/crates/app/rs/checks/hooks/rs/mod.rs`
  - latest seen names:
    - `hook_rs_11_gitleaks_step_present`
    - `hook_rs_12_cargo_dupes_step_present`
- unrelated release test move errors:
  - `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_01_binary_release_workflow_tests/bypasses.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_02_linux_target_tests/bypasses.rs`

Because of that, use:
- `rustfmt` on touched hexarch files for immediate hygiene
- the rule intent + diff review + fresh adversarial rounds to continue hardening
- full `cargo test ... rs_hexarch_07 ...` only after those unrelated blockers are cleared again

## Immediate next steps

When resuming, do this in order:

1. Finish the broad `cargo test --manifest-path apps/guardrail3/Cargo.toml rs_hexarch_ -- --nocapture` rerun on `CARGO_TARGET_DIR=target/hexarch`
2. Use any failing rule from that rerun as the next active rule and resume fresh 4-agent rounds on that rule immediately
3. If the family rerun is green, update `01-hexarch.md` to mark `RS-HEXARCH-11..25` converged at the lane level instead of merely materially hardened
4. Then decide whether any remaining work is only doc/worklog/commit hygiene or whether another family-wide adversarial pass is still justified

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
- `apps/guardrail3/tests/unit/rs_arch_01/`
- `apps/guardrail3/tests/fixtures/r_arch_01/`

## What you are trying to prove

The family should survive broad structural attacks against:
- all Rust app hex roots
- nested hex roots
- workspace-member coverage
- dependency-direction rules
- boundary config bypasses

One test = one attack vector.

That test should mutate the golden fixture everywhere the vector should matter:
- all matching top-level hex roots
- all matching nested hex roots
- all matching workspace members or dependency edges

## Known gaps already identified

- many rules are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- migrated test depth is weaker than the old deliberate corpus
- `RS-HEXARCH-01..03` improved exact hit-set coverage but still do not match old-corpus breadth
- `RS-HEXARCH-13..25` still need deeper family-level breadth even after the first graph-shaped rewrite
- need direct proof that nested and top-level roots are attacked together
- `crates/macros/` is optional and must be allowed without weakening the rest of the structure
- the golden fixture should be treated as structure/config baseline; source-rule tests must explicitly call out when they depend on real Rust source content
- targeted Cargo verification is currently blocked by unrelated release-family compile drift already present in the tree, not by the hexarch changes themselves
- structural `RS-HEXARCH-01..06` still lag the old corpus on some mixed-combination attacks and severity-exactness, but the main replacement-shaped gaps are now covered

## Required attack classes

### Structural roots
- golden
- missing required dirs across all owned hex roots
- illegal extra sibling across all owned hex roots
- nested root parity
- optional `macros/`
- false positives against non-owned or non-Rust roots

### Workspace coverage
- missing members everywhere
- extra members everywhere
- out-of-boundary members
- malformed Cargo.toml fail-closed

### Dependency / boundary
- illegal direction edges across all matching members
- renamed dependency bypass
- inherited workspace dependency bypass
- target/dev edge variants
- cross-app leaks
- malformed boundary config fail-closed

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-HEXARCH-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact owned hit/non-hit assertions are used
- semantic bugs found during hardening are fixed or written into the lane file

## Resume here

If starting fresh in a new session:
1. read `01-hexarch.md` for the current coverage matrix
2. read `16-hexarch-execution-plan.md` for execution order
3. resume from the family-wide `rs_hexarch_` rerun result first; if green, update the docs to mark the lane converged, and if not, take the first failing rule as the next active one
4. update both `01-hexarch.md` and this brief after each completed rule group

## Do not

- port old tests mechanically
- keep grouped or loose “some result exists” assertions
- change rule policy silently just to satisfy a test
