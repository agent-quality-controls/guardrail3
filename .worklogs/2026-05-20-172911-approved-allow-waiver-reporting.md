# Approved Allow Waiver Reporting

## Summary

Fixed `g3rs-cargo/approved-allow-inventory` so a centrally waived approved allow entry no longer renders as "missing reason". The rule still emits a waiverable finding; the central waiver engine still downgrades it and prints the configured reason.

## Decisions Made

- Kept waiver matching out of the cargo rule package because the central waiver engine is the current architecture.
- Changed only the rule title and message from "missing reason" to "requires waiver" so the output is correct before and after central waiver application.
- Added a behavior verifier for the rendered CLI output because the bug was a false report contract after central waiver application, not TOML parsing.
- Verified the installed CLI from a clean temporary worktree because the main guardrail3 worktree has unrelated dirty app changes that currently break `cargo install`.

## Key Files

- `.plans/2026-05-20-172415-approved-allow-waiver-reporting.md`
- `scripts/verify-approved-allow-waiver-reporting.py`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/approved_allow_inventory.rs`
- `packages/shared/guardrail3-waivers/src/waiver.rs`
- `packages/shared/guardrail3-check-types/src/result.rs`

## Verification

- `cargo test --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/Cargo.toml --all-features`
- `cargo clippy --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo fmt --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/Cargo.toml --all -- --check`
- `G3RS_BIN=/Users/tartakovsky/.cargo/bin/g3rs python3 scripts/verify-approved-allow-waiver-reporting.py`
- Clean worktree install: `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --bin g3rs --force`
- Downstream check: `g3rs validate workspace --path packages/llm/ws-llm-client` in websmasher now prints `approved allow entry requires waiver` plus the waiver line, not `approved allow entry missing reason`.

## Next Steps

- The unrelated dirty `ValidateRepoRequest`/`execute_repo` mismatch in the guardrail3 main worktree still blocks installing from that dirty worktree. It was not part of this fix.
