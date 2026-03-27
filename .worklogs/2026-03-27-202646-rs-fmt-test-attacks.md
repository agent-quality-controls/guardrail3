# RS-FMT Test Attack Coverage Hardening

**Date:** 2026-03-27 20:26
**Scope:** `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_05_per_crate_override_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_08_dual_file_conflict_tests/mod.rs`

## Summary
Ran an adversarial test pass against the self-hosted `RS-FMT` family after stabilization. The pass did not expose a new production bug, but it did expose several missing coverage branches, which were added as focused sidecar tests plus one test-only family runner hook for `RS-FMT-01`.

## Context & Problem
The prior stabilization pass made `RS-FMT` self-host and self-validate cleanly, but that only proves the happy-path family shape and the specific regressions already known. The next risk was test incompleteness: fail-closed branches, alternate config-file forms, and nested-path attribution can silently rot if they are not exercised directly.

The user asked to "run test attacks", so the goal of this pass was to attack the tests rather than expand feature scope. During that work, fresh top-level `cargo run` and `cargo test` commands against `apps/guardrail3/Cargo.toml` were blocked by unrelated nested-workspace state in `families/clippy`, so the source-based family validation step had to be executed through the already-built `guardrail3` binary instead of a fresh top-level Cargo invocation.

## Decisions Made

### Add family-level attack coverage for root `.rustfmt.toml`
- **Chose:** Add a test-only `run_family(root: &Path)` helper in `rs_fmt_01_exists.rs` and use it to validate that a root `.rustfmt.toml` plus valid root metadata yields zero findings.
- **Why:** `RS-FMT-01` is about config existence, but the bug surface here is family behavior, not just the local rule helper. Attacking the full family catches interactions with the other `fmt` rules without pushing semantic filtering logic into the sidecar.
- **Alternatives considered:**
  - Reuse `run_check` only — rejected because it cannot model the whole-family clean path for `.rustfmt.toml`.
  - Filter family results inside the sidecar — rejected because that tripped `RS-TEST-16` by making the sidecar own semantic result interpretation.

### Cover fail-closed branches explicitly
- **Chose:** Add direct tests for missing `[toolchain].channel` in `rust-toolchain.toml` and missing root `Cargo.toml` for edition checks.
- **Why:** These are the kinds of branches that regress quietly when discovery or parsing changes. They are also central to the handoff intent that malformed or incomplete root metadata must not silently disable enforcement.
- **Alternatives considered:**
  - Rely on existing malformed-file tests — rejected because malformed and missing-required-field are distinct failure modes.
  - Leave coverage implicit via self-validation — rejected because self-validation only proves the compliant case.

### Attack alternate and nested config forms directly
- **Chose:** Add tests for nested plain `rustfmt.toml` overrides and nested dual-file conflict attribution.
- **Why:** The family supports both `rustfmt.toml` and `.rustfmt.toml`, and path attribution matters for actionable findings. Those cases were not fully locked down by the existing tests.
- **Alternatives considered:**
  - Test only dotted nested files — rejected because plain nested `rustfmt.toml` is equally valid and equally risky.
  - Assert only that a conflict exists somewhere — rejected because the emitted path is part of the contract.

## Architectural Notes
The new helper in `rs_fmt_01_exists.rs` is test-only and intentionally thin: it delegates to `crate::check_test_root(root)` rather than exposing extra runtime seams or moving orchestration into the test module. That preserves the family architecture while still allowing attack tests to exercise family-level interactions.

This pass stayed within the required sidecar pattern. No family-wide grouped test file was introduced, and each added attack stays in the rule-specific sidecar directory already used by the `fmt` family.

## Information Sources
- `.plans/todo/checks/rs/fmt.md` — rule intent and expected family behavior
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/facts.rs` — config kind and normalized family facts
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/inputs.rs` — typed inputs and rule fan-out shape
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists.rs`
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable.rs`
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_05_per_crate_override.rs`
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch.rs`
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_08_dual_file_conflict.rs`
- `.worklogs/2026-03-27-200723-stabilize-rs-fmt-family.md` — prior stabilization context and detector hardening decisions

## Open Questions / Future Considerations
- Top-level Cargo execution remains blocked by unrelated `families/clippy` nested-workspace lint inheritance state. Once that is fixed, the `fmt` attack pass should be rerun through fresh top-level `cargo run` / `cargo test` to remove dependence on an already-built binary.
- Other stabilized families will likely need similar attack passes focused on fail-closed branches and exact path attribution, especially where self-validation currently proves only the golden path.

## Key Files for Context
- `AGENTS.md` — repo workflow, worklog policy, and current Rust-only scope
- `.plans/todo/checks/rs/fmt.md` — `RS-FMT` rule inventory and intended behavior
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists.rs` — root config existence rule plus the test-only family runner hook
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable.rs` — fail-closed logic around nightly-only keys and toolchain parsing
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch.rs` — edition enforcement and missing-root-manifest behavior
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists_tests/mod.rs` — family-level clean-path attack for root `.rustfmt.toml`
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable_tests/mod.rs` — missing-channel fail-closed coverage
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch_tests/mod.rs` — missing-root-`Cargo.toml` fail-closed coverage
- `.worklogs/2026-03-27-200723-stabilize-rs-fmt-family.md` — previous handoff implementation record

## Next Steps / Continuation Plan
1. Fix the unrelated top-level nested-workspace lint inheritance breakage in `families/clippy` so fresh top-level Cargo commands work again.
2. Re-run `RS-FMT` validation through fresh top-level `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate ...` once the workspace is healthy.
3. Apply the same attack-pass method to the next stabilized Rust family: first cover fail-closed branches, then alternate config forms, then exact finding-path attribution.
