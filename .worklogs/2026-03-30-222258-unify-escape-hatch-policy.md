# Unify Escape Hatch Policy Across Rust Families

**Date:** 2026-03-30 22:22
**Scope:** `packages/reason-policy`, `apps/guardrail3/crates/domain/config/types.rs`, Rust family runtimes and tests for `code` consumers, `test`, `deny`, `clippy`, `cargo`, `fmt`, `garde`, and `hexarch`

## Summary
Introduced a shared reason-quality validator and migrated the major Rust escape-hatch surfaces onto one contract: missing or weak reasons fail, documented escape hatches stay visible, and families that own bypass surfaces now emit explicit count warnings instead of hiding documented debt as inventory.

The implementation covered both native reason carriers and sidecar-backed escape hatches. Native carriers were updated in `test`, `deny`, and `clippy`; sidecar-backed escape hatch lookup was threaded through `cargo`, `fmt`, `garde`, and `hexarch`.

## Context & Problem
The repo already had multiple local “escape hatch” surfaces, but they behaved inconsistently. Some only checked that a reason field existed, some hid documented exceptions as inventory, and some had no shared quality bar at all. The user wanted one durable policy:

- every escape hatch must have a reason
- weak reasons must fail, not pass
- documented escape hatches remain suspicious and visible
- escape-hatch accumulation must be obvious

The earlier `code` family work established the intended direction, but the rest of the Rust families had not yet converged on that model.

## Decisions Made

### Shared reason validation moved into a dedicated workspace crate
- **Chose:** keep the reusable reason-quality logic in `packages/reason-policy/crates/reason-policy`
- **Why:** the validator is pure policy, not family runtime orchestration; families should depend on a leaf utility, not on `app/rs/runtime`
- **Alternatives considered:**
  - put the validator in `app/rs/runtime` — rejected because that inverts dependency direction and turns the orchestrator crate into a utility sink
  - leave validators family-local — rejected because that guarantees drift in thresholds and placeholder handling

### Documented escape hatches are warnings, not inventory
- **Chose:** documented escape hatches emit visible `Warn` findings and count summaries
- **Why:** documented bypasses are still active debt; hiding them behind `--inventory` makes the normal output less useful
- **Alternatives considered:**
  - keep documented bypasses as inventory — rejected because it hides precisely the surfaces the user wants to track
  - make documented bypasses `Info` — rejected because it understates the risk of suppressions and overrides

### Weak reasons fail closed
- **Chose:** reason text must be non-empty, at least 2 words, at least the minimum character threshold, and not an obvious placeholder
- **Why:** presence-only checks were too easy to game with `temp`, `legacy`, or one-token filler
- **Alternatives considered:**
  - semantic NLP-style validation — rejected as too fuzzy and likely to drift
  - presence-only validation — rejected because it does not materially harden escape hatches

### Sidecar-backed families use explicit selectors
- **Chose:** `cargo`, `fmt`, `garde`, and `hexarch` match escape-hatch reasons through exact `(family, file, kind, selector)` keys
- **Why:** these formats do not always carry a native reason slot, and fuzzy matching would be fragile
- **Alternatives considered:**
  - family-local heuristics based only on file path or lint name — rejected because they are ambiguous and easy to misapply
  - no sidecar support for config-based escape hatches — rejected because those families still need enforceable documentation

## Architectural Notes
- `packages/reason-policy` now owns `validate_reason_text`, `ReasonIssue`, and the placeholder / word-count / length checks.
- `GuardrailConfig` now carries shared `escape_hatches`, enabling sidecar-backed policy lookup across families.
- Families keep local extraction and ownership logic, but delegate reason-quality validation to the shared crate.
- `cargo` selectors use `"<lint-family>:<lint-name>"`.
- `fmt` uses a coarse `ignore` selector because the whole `ignore` entry is the escape hatch.
- `garde` selectors use `"<macro>@L<line>"` so query macros remain exact even under aliasing.
- `hexarch` selectors use patch/replace identity tied to the resolved layered target surface.

## Information Sources
- Existing family rule implementations and tests under `apps/guardrail3/crates/app/rs/families/*`
- Shared policy discussion captured in `.plans/todo/checks/2026-03-30-shared-escape-hatch-policy.md`
- Existing `code` family reason-policy work from the current branch state
- Local test results from:
  - `cargo test --manifest-path packages/reason-policy/Cargo.toml`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-cargo --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
  - aggregate family sweep over `code`, `test`, `deny`, `clippy`, `cargo`, `fmt`, `garde`, and `hexarch`

## Open Questions / Future Considerations
- `cargo`, `fmt`, `garde`, and `hexarch` now rely on sidecar registry entries. If more families need this, the selector matching may deserve a tiny shared helper crate instead of repeating tuple lookup logic.
- Placeholder lists and thresholds are still intentionally simple. If teams start gaming them in new ways, update the shared crate rather than letting families fork policy.
- Some escape-hatch surfaces in other families still lack native or sidecar-backed reason channels; those should be migrated onto this same contract instead of inventing per-family behavior.

## Key Files for Context
- `packages/reason-policy/crates/reason-policy/src/lib.rs` — shared reason-quality contract
- `apps/guardrail3/crates/domain/config/types.rs` — shared `escape_hatches` config model
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lint_support.rs` — cargo selector and sidecar lookup helpers
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_07_ignore_escape_hatch.rs` — sidecar-backed documented ignore policy
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/inventory/rs_garde_09_query_as_inventory.rs` — documented `sqlx::query_as!` bypass handling
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_policy/rs_hexarch_16_patch_replace_bypass.rs` — forbidden-but-documented bypass behavior
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/assertion_quality/rs_test_04_ignore_reason.rs` — native attribute reason validation in tests
- `.plans/todo/checks/2026-03-30-shared-escape-hatch-policy.md` — target-state design that drove this implementation

## Next Steps / Continuation Plan
1. Migrate the remaining Rust escape-hatch families that still use ad hoc reason handling or no reason handling at all onto `guardrail3-reason-policy`.
2. If more sidecar-backed surfaces appear, extract shared escape-hatch registry matching instead of duplicating `(family, file, kind, selector)` scans.
3. Audit repo-local `guardrail3.toml` files and add the missing escape-hatch entries so self-host output reflects documented debt rather than blanket “missing reason” failures.
