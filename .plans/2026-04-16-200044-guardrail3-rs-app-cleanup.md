## Goal

Make `apps/guardrail3-rs` conform to the active Rust package rules without changing the rules. The end state is a clean app workspace with root policy files, explicit publish intent, correct waiver metadata, and crate boundaries that match the current app architecture rules.

## Approach

1. Fix workspace-root package debt first.
   - Add missing root policy files at `apps/guardrail3-rs/`.
   - Add `guardrail3-rs.toml` with the exact waiver reasons needed by current manifest allows and any real shared-crate selectors.
   - Bring `Cargo.toml` up to the current workspace lint baseline instead of patching rule by rule.
   - Add explicit `publish = false` to all member crates unless the app truly publishes.
2. Fix crate metadata and dependency intent.
   - Mark genuinely shared crates with `[package.metadata.guardrail3] shared = true`.
   - Remove apparch-invalid deps from `types` and `logic` by moving contracts and behavior to the right crate.
3. Reshape each crate so facades stay facades.
   - Split `lib.rs` and `main.rs` bodies into real submodules.
   - Move inline tests to owned `*_tests/mod.rs` sidecars.
   - Keep file-owned sidecar `#[path]` only in the exact approved shape.
4. Move behavior out of `crates/types/app-types`.
   - Keep `types` as passive data and traits only if still justified.
   - Move CLI parsing helpers, severity reduction helpers, and selection logic into `logic` or `io`.
   - Replace public `Result<_, String>` APIs with typed errors where required.
5. Re-check architecture pressure.
   - Re-read `crates/io/outbound/packages` after the above moves.
   - If it still exceeds the dependency cap, split it at a real boundary instead of waiving it.

## Key decisions

- Fix root policy and manifest debt before code moves.
  - Why: it removes a large amount of noise and makes the architectural problems easier to see.
- Do not mark crates shared unless the dependency shape clearly requires it.
  - Why: `shared = true` is an architectural contract, not a quick way to silence edge checks.
- Do not keep behavior in `types` just because the app currently uses it there.
  - Why: that would directly violate the current app architecture rules and spread the bad shape.
- Treat the app as unpublished unless the manifests prove otherwise.
  - Why: this is an internal app workspace, not a crate release workspace.

## Files to modify

- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/guardrail3-rs.toml`
- `apps/guardrail3-rs/rust-toolchain.toml`
- `apps/guardrail3-rs/rustfmt.toml`
- `apps/guardrail3-rs/clippy.toml`
- `apps/guardrail3-rs/deny.toml`
- `apps/guardrail3-rs/crates/types/app-types/**`
- `apps/guardrail3-rs/crates/logic/validate-command/**`
- `apps/guardrail3-rs/crates/io/inbound/cli/**`
- `apps/guardrail3-rs/crates/io/outbound/packages/**`
- `apps/guardrail3-rs/crates/io/outbound/report/**`
