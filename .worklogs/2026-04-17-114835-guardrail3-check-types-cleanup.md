Summary
- Collapsed `packages/shared/guardrail3-check-types` from a one-member workspace into a root crate.
- Added the missing package policy files, normalized `deny.toml`, made publish intent explicit, and rewrote every dependency path that still pointed at the old nested member crate.
- The package now validates clean at `packages/shared/guardrail3-check-types`.

Decisions made
- Chose a root crate instead of preserving the old one-member workspace. For a single shared types package, the workspace-with-one-member shape was unnecessary indirection and not the clean long-term form.
- Kept the crate unpublished. This package is internal shared contract surface for the repo, not a separate published artifact.
- Updated all path dependencies to the package root instead of leaving stale nested-member paths behind.

Key files for context
- `packages/shared/guardrail3-check-types/Cargo.toml`
- `packages/shared/guardrail3-check-types/guardrail3-rs.toml`
- `packages/shared/guardrail3-check-types/deny.toml`
- `packages/shared/guardrail3-check-types/clippy.toml`
- `packages/shared/guardrail3-check-types/src/lib.rs`
- `packages/shared/guardrail3-check-types/src/profile.rs`
- `packages/shared/guardrail3-check-types/src/result.rs`
- `packages/shared/guardrail3-check-types/src/severity.rs`
- `apps/guardrail3-rs/Cargo.toml`
- Cargo.toml files under `packages/rs/**` that depend on `guardrail3-check-types`

Next steps
- Commit this slice by itself.
- Move to `packages/shared/reason-policy` and clean it package by package.
- Keep watching for other one-member workspace packages that should collapse to a root crate instead.
