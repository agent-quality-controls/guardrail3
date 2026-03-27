# Stabilize Toolchain Family

**Date:** 2026-03-27 20:30
**Scope:** `apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/toolchain/README.md`, `apps/guardrail3/crates/app/rs/families/toolchain/rust-toolchain.toml`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/*`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/assertions/src/*`, removal of legacy `apps/guardrail3/crates/app/rs/families/toolchain/src/*`

## Summary
Converted the `RS-TOOLCHAIN` family from the old single-crate layout into the stabilized family workspace shape with `crates/runtime` and `crates/assertions`, added a family README and self-hosted root `rust-toolchain.toml`, and moved all rule tests into owned `*_tests/mod.rs` sidecar directories. During the required attack pass, I fixed concrete detector weaknesses around malformed active inputs, missing root `Cargo.toml`, and version-like nightly/beta suffix bypasses.

## Context & Problem
The handoff for `.plans/todo/family-stabilization-handoffs/toolchain.md` was explicit that this was not a repo-cleanup task. The job was to make the `toolchain` family structurally self-hosted like the already stabilized Rust families, pass `RS-ARCH` and `RS-TEST` on its own root, and then attack the rules for trustworthiness rather than for lower repo-wide counts.

At the start of the work, the family still had the old shape:

- single crate rooted directly at `families/toolchain`
- flat `*_tests.rs` sidecars
- no family README
- no owned root `rust-toolchain.toml`

That matched the handoff snapshot exactly: unit tests were green, `RS-ARCH` was already clean, `RS-TEST` failed on flat sidecars, and `RS-TOOLCHAIN` failed because the family root did not carry its own toolchain contract.

## Decisions Made

### Split the family into the stabilized workspace shape
- **Chose:** Convert `families/toolchain` into a workspace root with `crates/runtime` and `crates/assertions`, and move the legacy `src/*` code into `crates/runtime/src/*`.
- **Why:** This matches the live family pattern already used by `test`, `arch`, `cargo`, `hexarch`, and `code`, which is what `RS-TEST` is enforcing structurally.
- **Alternatives considered:**
  - Keep the single-crate layout and only rename test files — rejected because it would still leave the family off-pattern and not actually “stabilized”.
  - Add only `crates/runtime` without `assertions` — rejected because the stabilized-family contract expects reusable assertion ownership to live outside runtime sidecars.

### Self-host the family with a root `rust-toolchain.toml`
- **Chose:** Add `apps/guardrail3/crates/app/rs/families/toolchain/rust-toolchain.toml` with `stable` plus `clippy` and `rustfmt`.
- **Why:** Without an owned root toolchain file, the family can never validate itself under `RS-TOOLCHAIN-01` or `RS-TOOLCHAIN-02`.
- **Alternatives considered:**
  - Leave the family root without a toolchain file and accept a permanent self-failure — rejected because that makes the family non-self-hosting.
  - Suppress self-validation for this family — rejected because it would undercut the point of stabilization.

### Convert flat rule tests into owned sidecar directories
- **Chose:** Replace `rs_toolchain_*_tests.rs` with per-rule `rs_toolchain_*_tests/mod.rs` directories and keep test data local to each rule.
- **Why:** This is the exact rule-specific sidecar pattern required by the Rust family architecture and enforced by `RS-TEST`.
- **Alternatives considered:**
  - Keep flat files and weaken `RS-TEST` expectations — rejected because the family should conform to the architecture, not special-case itself.
  - Collapse tests into one grouped family-wide file — rejected because the plan explicitly forbids grouped family test files.

### Add `assertions` as the reusable semantic home for rule result checks
- **Chose:** Create one assertion helper module per rule under `crates/assertions/src/`.
- **Why:** The stabilized families push reusable semantic result assertions out of runtime sidecars so local tests do not duplicate meaning-bearing expectations ad hoc.
- **Alternatives considered:**
  - Leave raw result assertions duplicated in each sidecar — rejected because it keeps the family structurally weaker than the specimen families.
  - Add a single generic helper only — rejected because the project trend is one reusable assertion module per rule owner.

### Fail closed on malformed active toolchain inputs
- **Chose:** Make `RS-TOOLCHAIN-02` error on non-string `channel` values and invalid `components` shapes, rather than degrading those cases into missing-channel or missing-component signals.
- **Why:** Malformed active inputs should not weaken enforcement; the plan explicitly calls out malformed required inputs as fail-closed surfaces.
- **Alternatives considered:**
  - Treat those as warnings — rejected because malformed active config is not merely incomplete policy, it is unreliable input.
  - Ignore invalid entries and inventory what still parses — rejected because that silently weakens the family.

### Fail closed when `RS-TOOLCHAIN-03` cannot trust root `Cargo.toml`
- **Chose:** Track root `Cargo.toml` presence explicitly and error when the file is missing or malformed for pinned-toolchain MSRV comparison; also error on invalid `rust-version` values.
- **Why:** `RS-TOOLCHAIN-03` exists to compare pinned stable toolchain against declared MSRV. If root `Cargo.toml` is missing or unreadable, inventing an “MSRV not declared” inventory result is misleading.
- **Alternatives considered:**
  - Keep treating missing root `Cargo.toml` as “no rust-version declared” — rejected because it turns broken input into a benign informational state.
  - Skip the rule entirely on missing/malformed Cargo input — rejected because that is exactly the fail-open hole the handoff warned against.

### Tighten channel parsing so version-like nightly/beta forms cannot bypass policy
- **Chose:** Classify any channel string containing `nightly` or `beta` as those disallowed channels, and stop trimming suffixes before version parsing.
- **Why:** The attack pass found that `1.85.0-nightly` and `1.85.0-beta` were being accepted as pinned stable versions because the old parser cut the string at the first `-`.
- **Alternatives considered:**
  - Keep the old split-at-dash logic and only block exact `nightly-*` / `beta-*` prefixes — rejected because version-like suffix forms are a real bypass.
  - Treat any dashed channel as unsupported — rejected because the real policy distinction is semantic nightly/beta vs pinned stable, not merely the presence of a dash.

## Architectural Notes
The family remains root-level and does not reintroduce family-local root discovery. It still consumes a single repository snapshot via `ProjectTree`, gathers family facts in `discover.rs`, projects a single `ToolchainRootInput`, and runs pure per-rule functions from `lib.rs`.

The structural change was intentionally narrow:

- `crates/runtime` owns production discovery, facts, inputs, and rule execution
- `crates/assertions` owns reusable rule-result assertions for tests
- there is no `test_support` crate because the family does not need shared fixture builders yet

The family now self-hosts both the structural family pattern and the root toolchain policy it enforces. The only remaining `RS-TOOLCHAIN` self-results on the family root are inventory infos for existence and required components, which is consistent with the current rule contract under `--inventory`.

## Information Sources
- `.plans/todo/family-stabilization-handoffs/toolchain.md`
- `.plans/todo/checks/rs/toolchain.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/*`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/*`
- prior worklogs:
  - `.worklogs/2026-03-27-192542-add-toolchain-and-fmt-handoffs.md`
  - `.worklogs/2026-03-27-192800-inventory-inline-unused-crate-deps-exemptions.md`

## Open Questions / Future Considerations
- The handoff’s “expected final state” said `RS-TOOLCHAIN` should be `0 errors, 0 warnings, 0 info`, but under the current rule contract and `--inventory`, a valid self-hosted family necessarily emits info inventory for `RS-TOOLCHAIN-01` and the positive `RS-TOOLCHAIN-02` checks. That success criterion should probably be restated as `0 errors, 0 warnings`.
- `RS-TOOLCHAIN-03` still intentionally does nothing for unpinned `stable` because the rule only compares pinned stable toolchain vs MSRV. If the project later wants stronger stable/MSRV guarantees, that would be a policy expansion rather than a detector bug fix.
- There is still no test that drives the full family through `crate::check(...)` with a synthetic `ProjectTree`; the current sidecars are rule-local. That matches family conventions, but a narrow top-level smoke test could still be useful.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md` — family contract and self-hosting explanation
- `apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml` — workspace root for the stabilized family
- `apps/guardrail3/crates/app/rs/families/toolchain/rust-toolchain.toml` — self-hosted root toolchain contract
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/lib.rs` — orchestrator entrypoint
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs` — root input discovery, including `Cargo.toml` presence/parse handling
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_02_channel_and_components.rs` — live channel/components rule with malformed-input and suffix-classification hardening
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_03_msrv_consistency.rs` — live MSRV consistency rule with fail-closed Cargo handling
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_02_channel_and_components_tests/mod.rs` — adversarial regressions for invalid channel/components and nightly/beta suffix bypasses
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_03_msrv_consistency_tests/mod.rs` — adversarial regressions for missing/malformed/invalid Cargo MSRV inputs
- `.worklogs/2026-03-27-192542-add-toolchain-and-fmt-handoffs.md` — the handoff provenance for this task

## Next Steps / Continuation Plan
1. Commit the stabilized `toolchain` family workspace plus the detector hardening in one scoped commit, without pulling in unrelated repo changes.
2. If the project wants literal `0 info` on self-validation, decide whether to stop using `--inventory` for the “green family root” check or to redefine the success criterion as `0 errors, 0 warnings`.
3. Apply the same stabilization flow to the next small family handoff, likely `fmt`: read the handoff, split into workspace shape, make it self-host under `RS-TEST`, then run an adversarial rule pass before cleanup.
