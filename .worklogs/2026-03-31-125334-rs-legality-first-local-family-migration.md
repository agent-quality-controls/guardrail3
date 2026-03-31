# Rust Legality-First Local Family Migration

**Date:** 2026-03-31 12:53
**Scope:** `apps/guardrail3/crates/app/rs/{README.md,legality,family_mapper,runtime}`, Rust family runtimes/tests for `arch`, `toolchain`, `cargo`, `clippy`, and `deny`, plus Rust family plan docs under `.plans/by_family/rs` and `.plans/todo/checks/rs`

## Summary
Completed the legality-first routing migration for the current Rust family set that was under active change. The shared Rust pipeline now computes structure and legality before workspace-local families run, `arch` reports topology and placement failures, and local families consume only legal local surfaces while keeping pure rule tests separate from routed-family tests.

## Context & Problem
The previous ownership-routing work had built the attachment layer but stopped halfway through the architecture the user asked for. The mapper and some families still behaved as if they were the owners of placement legality, and several cargo/deny/toolchain/clippy tests were still proving old standalone-package or misplaced-file behavior.

The user’s corrections in this session forced the stronger split:
- `ProjectTree` is only the generic repo snapshot
- Rust needs one pre-family structure/attachment pass
- legality has to be computed before local-family routing
- `arch` is the reporting surface for that legality, not just another family among peers
- workspace-local families must not be given repo-wide illegal placement to judge
- routed-family tests must use legal workspace shapes only
- pure rule semantics should be tested directly with typed inputs when the routed pipeline is not the thing under test

The work therefore had two parts:
- implement and wire the legality-aware routing architecture
- clean the family suites so they match that architecture instead of encoding the older one

## Decisions Made

### Treated legality as a shared Rust stage, not as family execution order
- **Chose:** keep `ownership` as the shared attachment evidence layer, add `legality` as the shared legality derivation stage, and make `runtime`/`family_mapper` consume legality facts before local-family fanout.
- **Why:** the user was right that “run `arch` first” is the wrong mental model. The real dependency is pipeline order, not family order.
- **Alternatives considered:**
  - Let workspace-local families judge misplaced repo-wide files — rejected because it turns them into hacked mixed-scope families.
  - Add a second global placement family separate from `arch` — rejected because topology and placement are the same structural domain.

### Made `arch` the structural owner of workspace-local family placement
- **Chose:** move workspace-local artifact placement legality into `arch`, with new `RS-ARCH-09` through `RS-ARCH-16` coverage for top-level workspaces, loose packages, nested workspaces, declared-member exactness, member-path escapes, auxiliary workspace requirements, and workspace-local file placement.
- **Why:** whether a local-family file is legal depends on the same whole-tree Rust structure facts as root topology. Splitting those judgments across two global owners would duplicate the architecture domain.
- **Alternatives considered:**
  - Keep placement inside local families — rejected because illegal files outside a legal workspace either disappear or force mixed-scope hacks.
  - Encode one `arch` rule per raw filename without shared structure — rejected because it would turn `arch` into a filename catalog instead of a structural family.

### Kept local families content-only and removed stale placement ownership
- **Chose:** strip repo-global placement/shadowing ownership out of the migrated local families and leave them with legal-workspace-local content checks only.
- **Why:** once legality is shared and routed before invocation, local families should validate content, not rediscover whether the file should exist there.
- **Alternatives considered:**
  - Preserve local family fallback routes “just for tests” — rejected because it would keep the wrong production model alive behind the test harness.

### Reclassified cargo tests by what they were actually proving
- **Chose:** keep routed family tests only for legal workspace-root policy scenarios, and convert the malformed-owned-policy-root case in `RS-CARGO-14` into a direct typed-input rule test.
- **Why:** that case was not a routed-family behavior anymore. Under legality-first routing, malformed owned policy roots are filtered by legality and reported by `arch`; the cargo rule itself still needs a direct rule-level test for its error formatting semantics.
- **Alternatives considered:**
  - Reintroduce a synthetic route fallback in cargo test helpers — rejected because the user explicitly called that out as coping out.
  - Leave the malformed routed test in cargo and force mapper/runtime to surface it anyway — rejected because it violates the architecture we just established.

### Updated routed cargo fixtures to legal workspace policy roots
- **Chose:** rewrite the failing cargo workspace-policy suites so they use legal workspace-root manifests and workspace lint tables instead of old `pkg/` roots and package-local `[lints.*]` tables.
- **Why:** those tests were green only under the old weaker routing contract. Under the new contract they were proving the wrong surface.
- **Alternatives considered:**
  - Preserve the old fixtures and special-case them in runtime — rejected because it would reintroduce standalone-package behavior through tests.

### Accepted that some files are genuinely shared across families
- **Chose:** keep `clippy.toml` attached to both `clippy` and `garde` because `garde` explicitly depends on the covering clippy ban surface.
- **Why:** a black-box attack showed `arch` reporting a nested `clippy.toml` as illegal for both families. That is noisy, but it is not a leak; it reflects real shared ownership already described in the garde family contract.
- **Alternatives considered:**
  - Treat the duplicate report as a bug and collapse ownership to clippy-only — rejected because it would silently break garde’s declared dependency on clippy policy.

## Architectural Notes
- `ProjectTree` remains language-agnostic.
- Rust now effectively has:
  - shared structure/attachment discovery (`placement` + `ownership`)
  - shared legality derivation (`legality`)
  - `arch` as the reporting family for shared legality
  - mapper-built family surfaces
  - runner-built per-invocation slices
- `family_mapper` now knows legality and filters workspace-local family surfaces to legal local files.
- `runtime/src/runners.rs` fans workspace-local families out once per legal workspace instead of treating the whole repo as one local invocation.
- `toolchain` no longer owns illegal placement, ancestor shadow drift, or descendant shadowing rules; those structural cases moved to `arch`.
- `cargo` now has a cleaner split between:
  - routed legal workspace policy-root tests
  - direct rule-input tests for `RS-CARGO-14` failure formatting
- `deny` and `clippy` were kept green under the new routing contract, which means the architecture change did not just work for cargo.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-31-073815-mapper-family-consumption-hardening.md`
- `.worklogs/2026-03-31-002143-shared-ownership-routing-migration.md`
- `.worklogs/2026-03-30-223032-rs-scope-docs-and-ownership-spec.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/todo/checks/rs/arch.md`
- `.plans/by_family/rs/{arch,cargo,deny}.md`
- shared Rust code under:
  - `apps/guardrail3/crates/app/rs/legality`
  - `apps/guardrail3/crates/app/rs/family_mapper`
  - `apps/guardrail3/crates/app/rs/runtime`
  - `apps/guardrail3/crates/app/rs/ownership`
- family code under:
  - `apps/guardrail3/crates/app/rs/families/arch`
  - `apps/guardrail3/crates/app/rs/families/toolchain`
  - `apps/guardrail3/crates/app/rs/families/cargo`
  - `apps/guardrail3/crates/app/rs/families/clippy`
  - `apps/guardrail3/crates/app/rs/families/deny`
- CLI attack runs against temp fixtures using lean `guardrail3 --no-default-features --features family-*`

## Open Questions / Future Considerations
- `clippy.toml` as a shared `clippy` + `garde` placement surface is structurally correct today, but if the duplicate `RS-ARCH-16` reporting becomes too noisy the reporting layer may need a grouping or de-duplication policy later.
- `deps`, `release`, and `garde` still need the same final pass of architecture scrutiny if they are going to be held to the exact same legality-first/content-only split as the families migrated here.
- The current repo still has many in-flight changes outside this migration. This commit should be treated as the checkpoint where the legality-first architecture becomes real for the active family set, not as the final end-state of every Rust family.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust architecture spec after the legality-stage clarification
- `apps/guardrail3/crates/app/rs/legality/src/lib.rs` — shared legality derivation for legal roots and illegal family-file placement
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — legality-aware family surface mapping
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — file/root view filtering for legal local surfaces
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — per-workspace invocation fanout
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — arch orchestration over shared legality facts
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement.rs` — reporting of illegal workspace-local family-file placement
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/lib.rs` — toolchain after placement-rule removal
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs` — cargo orchestration after legality-first test cleanup
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/member_policy/rs_cargo_14_input_failures_tests/cases.rs` — direct typed-input cargo rule test for malformed owned policy root failures
- `.worklogs/2026-03-31-073815-mapper-family-consumption-hardening.md` — prior step that widened local-family candidate visibility before this legality-first cleanup
- `.worklogs/2026-03-31-002143-shared-ownership-routing-migration.md` — original ownership-routing implementation this checkpoint corrected

## Next Steps / Continuation Plan
1. Apply the same legality-first/content-only cleanup to the remaining workspace-local families that still carry legacy shape assumptions, especially `deps`, `release`, and any remaining `garde` route tests that still assume pre-legality visibility.
2. Decide whether `arch` should keep emitting one placement result per owning family when a file is genuinely shared, or whether reporting should collapse shared-file placement into a grouped message while preserving ownership facts internally.
3. If the team wants stricter end-to-end guarantees, add shared runtime CLI-style tests for the placement split so the black-box cases exercised here become part of the repo test suite instead of remaining manual attack runs.
