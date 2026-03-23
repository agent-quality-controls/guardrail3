# Hooks And CLI

## Hook family migration gaps

- `HOOK-SHARED` / `HOOK-RS` do not yet exist as migrated new-architecture families.
- Current hook validation still lives in legacy hook code and uses weak text matching where plans require executable-line / parsed-command semantics.

## Shared hook hardening still missing

- `HOOK-SHARED-10` is overclaimed relative to actual generated shell flags.
- `HOOK-SHARED-11` / `12` are incomplete for shebangs and modular-script permissions.
- `HOOK-SHARED-18` / `19` are not implemented at plan-grade rigor.
- Shared structural hardening still missing:
  - shebang validation
  - modular-script execute bits
  - `exit 0` bypass detection
  - `--no-verify` comment bans
  - real dispatcher syntax
  - concrete lockfile-command validation
  - fail-open wrapper detection

## Rust-specific hook hardening still missing

- `HOOK-RS-08..13` and `15` still rely on weak legacy validation shape.
- `HOOK-RS-14` fail-closed `guardrail3` availability is not enforced.
- `HOOK-RS-16` config-change-triggered Rust validation is not enforced.
- Missing or weak Rust hook validations include:
  - `guardrail3 validate --staged` step detection
  - `cargo clippy -D warnings` validation
  - `cargo test --workspace` validation
  - `guardrail3` / `cargo-dupes` tool-install checks

## Mutation-hook detection gap

- `RS-TEST-08` currently proves only coarse non-comment-line presence.
- It does not align with the stricter executable-command model expected by the hook plans.
- Make `RS-TEST-08` reuse or match real executable-command parsing rather than substring-style detection.

## Hook generation gaps

- Full generate patches `GUARDRAIL3_RUST_WORKSPACE`, but narrower generate/install paths still emit the raw `.` default from the template.
- Unify all hook-generation paths on one `workspace_root` contract.
- Generated hook template still has a shell-logic bug around stylelint condition precedence.

## Hook prerequisite tool diagnostics

- Current hook tool checks cover only part of the actual prerequisite surface.
- The generated hook and hook validator still rely on:
  - `git`
  - `cargo`
  - `guardrail3`
  - `cargo-dupes`
- These need explicit shared/Rust hook ownership.

## CLI / routing mismatch

- The user-facing validate/report path still models Rust as coarse `code/architecture/release/tests` domains.
- Hook routing is especially stale:
  - `ValidateDomains` has no hook-/garde-specific dimension
  - hook validation is still gated only on `domains.code`
- Reconcile CLI/reporting contract with the Rust-only architecture.
