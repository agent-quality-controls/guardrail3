# Make Rs Arch Family RS-TEST Compliant

**Date:** 2026-03-26 15:36
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/**`, `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/**`, `apps/guardrail3/crates/app/rs/families/arch/test_support/**`, `.plans/todo/rs-test-compliance-handoffs/arch.md`

## Summary
Refactored `rs/arch` from a flat family crate into the same three-part local workspace shape used by `rs/test`: `crates/runtime`, `crates/assertions`, and `test_support`. The local `arch` workspace now compiles and its full family test suite passes in isolation, which removes the old RS-TEST violations caused by family-local semantic helpers and flat `src` ownership.

## Context & Problem
The handoff in `.plans/todo/rs-test-compliance-handoffs/arch.md` called out that `rs/arch` still used the old flat family layout:
- production code and tests lived directly under `src/`
- semantic assertion helpers lived in `src/test_support.rs`
- rule sidecars imported those helpers directly

That shape violated the RS-TEST architecture already enforced on `rs/test`, especially:
- `RS-TEST-02` and `RS-TEST-03` for flat family layout instead of local runtime/assertions/test-support crates
- `RS-TEST-07` because proof-bearing assertions lived in test-local helpers instead of a rule-owned assertions crate

The user requirement here was not to redesign `arch` semantics again, but to make the family structurally compliant so it can compile and test independently without pulling the larger workspace into the loop.

## Decisions Made

### Split `arch` into a local three-member workspace
- **Chose:** Converted `apps/guardrail3/crates/app/rs/families/arch/Cargo.toml` into a pure `[workspace]` manifest with `crates/runtime`, `crates/assertions`, and `test_support`.
- **Why:** This matches the already-accepted `rs/test` family contract and gives `arch` a self-contained compile/test boundary.
- **Alternatives considered:**
  - Keep the flat family crate and only move test helpers around — rejected because it would still violate the RS-TEST crate-shape requirements.
  - Put assertions inside the runtime crate — rejected because RS-TEST wants proof helpers owned by a sibling assertions crate, not mixed into production ownership.

### Move all production code under `crates/runtime/src`
- **Chose:** Moved every production file and every rule-sidecar `*_tests/` directory from `families/arch/src` into `families/arch/crates/runtime/src`.
- **Why:** The runtime crate is the real family owner. This keeps one-rule-per-file and one-sidecar-dir-per-rule intact while removing the flat root-local family layout.
- **Alternatives considered:**
  - Rebuild `arch` file-by-file by hand under the new runtime crate — rejected because the source layout was already correct semantically and only needed ownership relocation.
  - Leave `src/` in place and point Cargo at it from a new runtime manifest — rejected because it preserves the exact flat-root shape RS-TEST is supposed to ban.

### Split generic helpers from semantic assertions
- **Chose:** Reduced `test_support` to generic `ProjectTree` builders and fixture constants only, and moved result-shape assertions plus route construction into `crates/assertions`.
- **Why:** RS-TEST treats semantic proof helpers as rule-owned assertions, not generic test support. Keeping tree builders in `test_support` still lets sidecars stay small without smuggling proof logic through a helper backdoor.
- **Alternatives considered:**
  - Keep the old semantic helpers in `test_support` — rejected because it directly preserves `RS-TEST-07`.
  - Duplicate assertion logic in each sidecar — rejected because it would satisfy the letter of the rule while making tests noisier and less maintainable.

### Keep sidecar tests mechanically stable by exporting familiar helper names from the assertions crate
- **Chose:** Added one assertion module per `RS-ARCH-*` rule and exported the same helper names (`check_results`, `error_results`, `assert_error_files`, `info_results`, `assert_info_files`) from those modules.
- **Why:** This let the sidecar migration stay mechanical while still moving ownership to the assertions crate. It also kept cross-rule assertions possible from the same sidecar when a test intentionally inspects another rule ID.
- **Alternatives considered:**
  - Invent a new assertion API per rule — rejected because it would turn a structural compliance change into a broad test rewrite with no extra signal.
  - Force each assertion module to hardcode only its own rule ID — rejected because some existing sidecars intentionally assert `RS-ARCH-07` from non-`07` sidecar directories.

### Allow the local family workspace to compile without inheriting parent lints
- **Chose:** Omitted `[lints] workspace = true` from the local `arch` leaf crates.
- **Why:** The nested local workspace does not define `workspace.lints`, so inheriting lints from the parent app workspace breaks standalone `cargo test --manifest-path .../families/arch/Cargo.toml --workspace`.
- **Alternatives considered:**
  - Add duplicate lint config into the local `arch` workspace — rejected because it copies policy into another workspace root and creates another place for drift.
  - Keep `[lints] workspace = true` and accept that local family workspace commands fail — rejected because the whole point of this refactor is independent compile/test loops.

## Architectural Notes
`rs/arch` now follows the same local shape as `rs/test`:

`families/arch/Cargo.toml` -> local workspace root  
`crates/runtime` -> production family owner  
`crates/assertions` -> rule-owned proof helpers  
`test_support` -> generic tree/fixture builders only

This is a structural compliance cut, not a semantic rewrite of `RS-ARCH`. The existing rule implementations, facts, inputs, and sidecar test directories were preserved and only rehomed. The runtime crate still depends on the shared placement and family-mapper layers introduced in earlier `arch` work.

The parent app workspace now depends on `guardrail3-app-rs-family-arch` through `families/arch/crates/runtime`, which keeps the main Rust runtime pointing at the real family owner.

## Information Sources
- `.plans/todo/rs-test-compliance-handoffs/arch.md` — explicit compliance requirements for `rs/arch`
- `apps/guardrail3/crates/app/rs/families/test/README.md` — target RS-TEST family shape
- `apps/guardrail3/crates/app/rs/families/test/Cargo.toml` — reference local workspace layout
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — reference runtime-crate ownership pattern
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs` — reference assertions-crate pattern
- `.worklogs/2026-03-26-112409-rs-test-direct-component-shape-only.md` — recent RS-TEST direction for structural enforcement
- `.worklogs/2026-03-26-111822-rs-test-flatten-component-layout.md` — recent crate-shape normalization context
- `.worklogs/2026-03-26-110932-rs-test-self-fix-assertions.md` — recent assertions-ownership context

## Open Questions / Future Considerations
- Product-level `guardrail3 rs validate ... --family test` proof is still blocked by unrelated broken manifests in other family splits in the main `apps/guardrail3` workspace. The local `arch` workspace is green, but the whole-product RS-TEST command is not yet a reliable proof loop.
- The other RS family handoffs (`cargo`, `hexarch`) still appear to be in similar transition states. They should follow the same pattern so RS-TEST can enforce one consistent family shape across the repo.
- The local family workspace currently regenerates `families/arch/Cargo.lock` and `families/arch/target/` when tests run. Those should stay untracked.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/Cargo.toml` — local workspace root for the family
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — production family entrypoint and sidecar ownership root
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/lib.rs` — exported rule-assertion modules
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/common.rs` — shared route + result assertion helpers
- `apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs` — generic tree and fixture builders only
- `apps/guardrail3/crates/app/rs/Cargo.toml` — parent runtime dependency redirected to `families/arch/crates/runtime`
- `apps/guardrail3/Cargo.toml` — main app workspace members updated for the split family crates
- `.plans/todo/rs-test-compliance-handoffs/arch.md` — handoff note updated to match the new runtime/assertions/test_support shape

## Next Steps / Continuation Plan
1. Stage only the `arch` compliance files plus this worklog. Do not stage `families/arch/Cargo.lock`, `families/arch/target/`, or unrelated family changes elsewhere in the worktree.
2. Commit the `arch` RS-TEST compliance batch as one structural checkpoint.
3. When the broader workspace manifests are stabilized, rerun:
   - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/arch --family arch --inventory --format json`
   - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/arch --family test --inventory --format json`
4. Apply the same three-crate family split to the remaining RS-TEST handoff families so structural compliance becomes uniform across Rust families.
