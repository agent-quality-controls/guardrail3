# Rust Hardening Follow-ups From Archived Audit Sweep

**Date:** 2026-03-23
**Scope:** Cross-family Rust hardening backlog extracted from archived `tests_guardrails.md` and `audit/`

## Purpose

The top-level audit backlog and test-guardrail notes have been reviewed against the current Rust-only codebase and active `checks/rs` plans.

Most of that material is now historical and belongs in `legacy/`.

This file keeps only the still-live Rust/shared follow-up work that did **not** already have clear ownership in an active family plan.

## Live follow-up items

### 1. Rust mutation-hook detection is still weaker than the hook contract

- `RS-TEST-08` currently proves only coarse non-comment-line presence.
- It still does not align with the stricter executable-command model expected by the hook plans.
- The concrete next step is to make `RS-TEST-08` reuse or match real executable-command parsing rather than substring-style detection.

### 2. Self-validation / negative-testing backlog is still largely unowned

- The old test/self-validation audit still leaves one real meta-quality gap:
  - guardrail3 should have explicit negative/self-validation coverage goals
  - mutation-resistance hardening of per-rule tests should be an intentional later phase, not accidental
- This is a project-level hardening backlog item, not a single-family rule.

### 3. Canonical drift protection for fmt/toolchain is weaker than cargo/deny

- `rs/fmt` and `rs/toolchain` still rely on hardcoded canonical expectations without an explicit active-plan note or consistency-test requirement tying them to generated modules.
- Add a later hardening pass similar in spirit to the stronger cargo/deny canonical-drift handling.

### 4. Whole-type `#[garde(skip)]` ownership is still unclear

- Current `rs/code` planning/implementation clearly owns field-level `#[garde(skip)]`.
- Old source-scan audit identified whole-struct/type `#[garde(skip)]` as a bypass surface.
- That broader ownership needs to be made explicit, likely in `rs/code` and/or `rs/garde`.

### 5. Finish retiring remaining legacy Rust validator wiring and helper dependencies

- Multiple audit batches converged on the same residual debt:
  - old `app/rs/validate/*` paths still exist and still influence helper/parsing behavior
  - new families like `rs/code` and `rs/test` still depend on legacy `ast_helpers`
- The migration-closure criterion is not written down clearly.
- Active improvement target:
  - either finish migrating the remaining legacy Rust validation paths/helpers into family-local code
  - or explicitly retire/delete them

### 6. Deny generation has a concrete effective-profile bug

- `deny.toml` generation still uses workspace-level `profile` where per-app/per-root `effective_profile` should drive the baseline.
- This is a concrete generator/checker contract bug and needs a regression test.

### 7. Hook generation still diverges on `workspace_root`

- Full generate patches `GUARDRAIL3_RUST_WORKSPACE`, but narrower generate/install paths still emit the raw `.` default from the template.
- Unify all hook-generation paths on one `workspace_root` contract.

### 8. Embedded module registry completeness is still incomplete

- The confirmed missing Rust-relevant generator surface is `DENY_BANS_LIBRARY_IO`.
- Expose it through `list-modules` / `show-module`.

### 9. CLI/domain routing is stale relative to the current Rust family model

- The user-facing validate/report path still models Rust as coarse `code/architecture/release/tests` domains.
- Hook routing is especially stale:
  - `ValidateDomains` has no hook-/garde-specific dimension
  - hook validation is still gated only on `domains.code`
- The planning contract for the replacement now lives in:
  - `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`
- Remaining work is implementation against that cutover spec, not further planning.

### 10. Shared hook prerequisite-tool diagnostics are still incomplete

- Audit found one narrow but real gap:
  - explicit prerequisite diagnostics for `git` / `cargo` in hook validation/generation paths are not clearly owned in current hook plans
- This belongs in shared/Rust hook planning rather than in archived audit notes.

### 11. Add explicit prerequisite-tool diagnostics to hook validation/generation

- Current hook tool checks cover only part of the actual prerequisite surface.
- The generated hook and hook validator still rely on tools such as:
  - `git`
  - `cargo`
  - `guardrail3`
  - `cargo-dupes`
- These should have explicit shared/Rust hook ownership instead of remaining implicit runtime assumptions.

## Archived source material this file replaces

- `.plans/todo/tests_guardrails.md`
- `.plans/todo/audit/`

Those notes should remain available only as historical/adversarial reference after archival.

## Active-plan review notes

### Batch 1: hooks, fmt/toolchain, cargo/deps

#### Hooks are still plan-ahead-of-code

- `HOOK-SHARED` / `HOOK-RS` do not yet exist as migrated new-architecture families.
- Current hook validation still lives in legacy hook code and uses weak text matching in places where the active plans require executable-line / parsed-command semantics.
- Concrete gaps called out by adversarial review:
  - `HOOK-SHARED-18` / `19` not implemented at plan-grade rigor
  - `HOOK-RS-14` fail-closed `guardrail3` availability not enforced
  - `HOOK-RS-16` config-change-triggered Rust validation not enforced
  - `HOOK-SHARED-10` overclaimed relative to actual generated shell flags
  - `HOOK-SHARED-11` / `12` incomplete for shebangs and modular-script permissions
  - `HOOK-RS-08..13` and `15` still rely on weak legacy validation shape
  - generated hook template still has at least one shell-logic bug around stylelint condition precedence

#### fmt/toolchain plans are stale and scope-misaligned

- `fmt.md` and `toolchain.md` still mark implemented rules as `Planned`.
- Both plans read like workspace-aware families, but current implementations are effectively repo-root-only.
- Concrete decisions needed next round:
  - either implement actual Rust-root/workspace-aware discovery
  - or narrow the plans to explicit repo-root-only semantics
- Specific under-implemented points:
  - `RS-TOOLCHAIN-CONFIG-02` claims library-specific behavior but toolchain facts currently carry no profile context
  - `RS-FMT-CONFIG-03` and `RS-FMT-CONFIG-04` compare only against repo-root toolchain/Cargo metadata, which is too shallow for multi-root semantics

#### cargo/deps plans are stale, and both families still have real scope gaps

- `cargo.md` and `deps.md` still mark implemented rules as `Planned` and should be updated before more auditing.
- `rs/cargo` is still repo-root-only, not Rust-root-aware:
  - nested workspaces / standalone package roots do not get cargo-family coverage today
- `rs/deps` still misses target-specific dependency tables:
  - `target.*.dependencies`
  - `target.*.build-dependencies`
  - `target.*.dev-dependencies`
- `RS-CARGO-CONFIG-04` implementation is broader than the plan text and the plan should either own or trim those additional warnings.
- `RS-CARGO` still has an unrecorded fail-open on malformed `guardrail3.toml`:
  - profile-sensitive rules silently lose `profile_name` when config parsing fails
  - add explicit profile-input failure handling or stop depending on silently parsed config for those rules

### Batch 2: clippy/deny, code/garde, hexarch/test

#### clippy/deny still have generator-checker drift

- Per-crate `deny.toml` generation still appears to ignore effective per-root profile selection.
- `RS-DENY-CONFIG-16` policy has since been tightened to match the stricter implementation: extra registries are forbidden, not merely inventoried.
- Clippy generation and validation still disagree on global-state bans for non-library pure/service contexts.
- The canonical module registry still does not fully expose the deny profile surface needed by the active deny contract.

#### code/garde plans are stale and still blur migration closure

- `code.md` still reads like a future-tense migration document even though `rs/code` is implemented.
- `rs/code` still depends on legacy `ast_helpers` for several suppression-related parsing paths, so migration closure is overstated.
- `garde.md` still marks implemented rules as planned.
- `garde.md` still overstates missing extractor-ban work that is already implemented.
- `RS-GARDE-CONFIG-05` rule text overclaims wrapper enforcement; current implementation only validates clippy method-ban completeness.
- `garde.md` used to carry live prose requirements with no explicit rule IDs.
  Current state after the garde hardening pass:
  - `RS-GARDE-AST-05` now owns field-level garde quality checks
  - `RS-GARDE-AST-06` now owns `#[garde(dive)]`
  - `RS-GARDE-AST-07` now owns explicit `ctx` surface wiring
  - wrapper-based boundary guidance remains intentionally checker-adjacent and is enforced through clippy ban surfaces rather than a garde-local AST rule
- `RS-GARDE-AST-04` plan text should explicitly mention both `query_as!` and `query_as_unchecked!`.

#### hexarch/test still have real fail-open and semantic-scope issues

- `rs/hexarch` still fails open on unreadable/unparsable Rust sources for `RS-HEXARCH-22/23`.
- `rs/hexarch` also fails open on malformed `guardrail3.toml` for boundary-config / `allowed_deps` checks.
- `hexarch.md` still points readers at legacy old-code paths instead of the live family.
- `RS-TEST-16` is narrower than the plan:
  - current code only checks integration/sidecar test files, not all test-bearing files
  - current “effective line” counting is weaker than the production-code model described in the plan
- `test.md` is materially stale about implementation state and should stop presenting the family as old-validator-only / planned.
- `rs/test` still depends on legacy `ast_helpers` for some parsing logic, so migration closure debt is broader than `rs/code` alone.

### Batch 3: release, architecture, cross-cutting check docs

#### release is implemented but still violates parts of its own plan and the architecture contract

- `RS-RELEASE-05..07` and `RS-BIN-01..02` still overclaim “actual execution step” semantics while relying on broad string matching over parsed workflow data.
- `RS-RELEASE-12` is only partially fail-closed:
  - discovery-critical failures emit an error
  - but dependent repo/crate coverage rules can still run from incomplete facts
- residual semantic-baseline hardening from the archived semver-release template is still unimplemented:
  - `release-plz.toml` workspace settings
  - `cliff.toml` git settings / parser coverage
- `rs/release` still drifts from the architecture plan’s minimal-input contract:
  - some rules still consume family-sized aggregates
  - support helpers still contain `ProjectTree` / filesystem-dependent extraction work that should live in orchestrator/facts code
- `code_fixes.md` surfaced one additional concrete release bug:
  - `readme = false` is not yet honored correctly by README checks

#### cross-cutting bug fixes still live outside family plans

- `rs/toolchain` still needs explicit channel-shape handling for `beta` and pinned `nightly-*` instead of lumping them into generic pinned info.
- `rs/fmt` still needs explicit canonical-drift protection against generated baseline, and `toolchain` should get the same kind of safeguard.
- `RS-FMT` still has an unrecorded fail-open/defaulting bug:
  - malformed `Cargo.toml` / `rust-toolchain.toml` gets silently dropped
  - some fmt rules then default or skip instead of surfacing input-integrity failure
- `RS-TOOLCHAIN` also still fails open on malformed `Cargo.toml` for MSRV checks and should get family-scoped input-integrity handling.

#### hook/check cross-cutting docs still point at real work

- `hooks_deploy_audit.md` is right that the hook implementation is still far behind the active hook plans:
  - `guardrail3 validate --staged` step detection missing
  - `cargo clippy -D warnings` validation missing
  - `cargo test --workspace` validation missing
  - `guardrail3` / `cargo-dupes` tool-install checks missing
  - Rust config-change trigger enforcement missing
  - shared hook structural hardening still missing:
    - shebang validation
    - modular-script execute bits
    - `exit 0` bypass detection
    - `--no-verify` comment bans
    - real dispatcher syntax
    - concrete lockfile-command validation
    - fail-open wrapper detection
- `hooks/ts.md` should likely be archived or relabeled legacy-only; no meaningful Rust/shared carry-forward remains there.

#### tighten the wording of this backlog itself

- Item 6 should stop saying deny-generation profile handling merely “appears to drift”; it is a concrete generator bug.
- Item 7 should be narrowed to the concrete `workspace_root` split between full generate and narrower generate/install paths.
- Item 9 should be narrowed to the actual live CLI/domain mismatch, especially around hooks, instead of a broad “reporting is stale” note.

#### additional plan hygiene discovered in the exhaustive sweep

- Several active Rust family plans still point `Current code` at legacy validator files instead of the live `app/rs/checks/rs/*` families:
  - `cargo.md`
  - `deps.md`
  - `fmt.md`
  - `toolchain.md`
  - `code.md`
  - `release.md`
- `code.md` still contains stale cross-checker action items that are already implemented (`std::process::abort`, `std::any::Any`, `unreachable_pub`, `lazy_static`) and should be cleared.
- `deploy/ts.md`, `hooks/ts.md`, and `hooks_deploy_audit.md` should be archived or relabeled legacy-only; no new Rust/shared requirement remains there beyond what is already tracked here.

#### newly confirmed family-specific concrete bugs

- `readme = false` is still not honored correctly by `rs/release` README checks and should be a top-level tracked release bug.
- `RS-DENY-CONFIG-16` policy decision is now explicit: extra registries are forbidden.

## Candidate future Rust rules extracted from `NEW_CHECKS.md`

These are Rust-only candidate guardrails that still look materially relevant after the active family sweep. They are not yet owned by current family plans as explicit rules.

### High-signal candidates not yet owned

- **Typed error discipline for public APIs**
  - extend beyond current `RS-CODE-25`
  - cover `anyhow`, `String`, and possibly require a crate-local typed error surface (`error.rs` / `error/` module or equivalent)

- **`pub(crate)` discipline / API surface minimization**
  - detect bare `pub` in non-`lib.rs` modules unless re-exported intentionally
  - this is broader than current `unreachable_pub` / re-export checks

- **No public fields on public structs**
  - exception for deliberate patterns such as `#[non_exhaustive]`
  - currently unowned

- **`Debug` on public types**
  - verify or enforce `missing_debug_implementations` for relevant profiles, or add source-level inventory when config is absent

- **`#[non_exhaustive]` on public enums**
  - likely warn-level for library-oriented crates
  - currently unowned as an explicit family rule

- **`#[must_use]` on public `Result` / `Option` functions**
  - currently only partially covered indirectly by lint configuration possibilities
  - not owned as an explicit Rust rule

- **No dependency types leaked through public API**
  - detect public signatures exposing external crate types where the crate should preserve abstraction boundaries

- **No unnecessary owned params in public functions**
  - e.g. `String` vs `&str`, `Vec<T>` vs `&[T]`, `PathBuf` vs `&Path`
  - library/API-discipline rule candidate

- **Internal module graph constraints**
  - cycle detection
  - fan-out limit
  - fan-in concentration warning
  - these are distinct from current crate/dependency-layer checks

- **Dependency pressure rules for libraries**
  - direct dependency count cap
  - transitive dependency depth pressure

- **Structural organization rules**
  - max module depth
  - max flat file count without subdirectories

- **Generic parameter count cap**
  - currently unowned as an explicit rule

- **String-based dispatch warning**
  - flag match expressions with many string literal arms where an enum should likely exist

### Already substantially covered by current families or canonical config

- Facade-only `lib.rs`:
  - covered by `RS-CODE-27`
- No public inline module bodies in `lib.rs`:
  - covered by `RS-CODE-28`
- No wildcard re-exports:
  - covered by `RS-CODE-26`
- Parameter count / bool parameter pressure:
  - already covered via clippy baseline / config contract
- Trait method count pressure:
  - already covered by `RS-CODE-29`
- No stdout/stderr / no process control / no `todo!` / no global mutable state:
  - already largely covered by `RS-CODE`, `RS-CLIPPY`, and canonical clippy bans

### Explicitly not part of the current contract

- `missing_docs` enforcement:
  - intentionally not active in the current Rust contract
  - do not reintroduce from `NEW_CHECKS.md` without an explicit product decision

### Batch 4: direct family review of clippy, deny, release

#### clippy still needs explicit source-of-truth closure

- Generator and checker still disagree on global-state bans outside `library` profile:
  - generation includes pure-layer service crates
  - checker only expects library-profile bans
- Add an explicit generator-vs-checker consistency test; the plan’s exact-match contract is still not enforced by one direct parity test.
- Clarify structural scope: generator-side grouped tests in `domain/modules/clippy` still violate the stricter one-rule/one-test direction if those module tests remain part of the active source-of-truth story.
- Update `clippy.md` so it describes the real profile/layer decision model instead of the earlier narrower explanation for `RS-CLIPPY-14`.

#### deny still needs policy and parity cleanup

- `deny.md` still marks the whole family as `Todo`; statuses need to reflect the implemented family.
- Keep `RS-DENY-CONFIG-16` plan/code aligned on the stricter policy:
  - forbid any non-crates.io registry
  - keep generator, checker, and docs exact about the accepted crates.io forms
- Add a real generator-vs-checker parity test for deny baseline.
- Replace stale hardcoded canonical service fixture in deny tests with generator-derived or parity-checked baseline.
- Update `deny.md` to describe the real generator surface, not an oversimplified single-file canonical source.
- Remove stale “known bugs to fix during migration” text now that many of those bugs are already fixed.

#### release still has concrete semantic and discovery gaps

- `release.md` still points readers at old validator files instead of the live `rs/release` family.
- `RS-PUB-10` / `11` still miss `workspace = true` local path dependency edges inherited from `[workspace.dependencies]`.
- `RS-RELEASE-05..07` still overclaim actual execution/wiring semantics while using broad substring-style checks over parsed workflow content.
- Promote the residual `release-plz.toml` / `cliff.toml` semantic-baseline hardening from prose into explicit future rules, or narrow the plan so it stops implying that baseline is already covered.
