# Move crate into apps/guardrail3/ and create workspace

**Date:** 2026-03-16 18:01
**Task:** Restructure repo from single-crate to workspace with crate in apps/guardrail3/

## Goal
The guardrail3 binary crate lives at `apps/guardrail3/` inside a Cargo workspace. R-ARCH-04 no longer fires on self-validation. All 414 tests pass. Binary name unchanged.

## Approach

### Step-by-step plan
1. Create `apps/guardrail3/` directory
2. `git mv src apps/guardrail3/src`
3. `git mv Cargo.toml apps/guardrail3/Cargo.toml`
4. `git mv clippy.toml apps/guardrail3/clippy.toml`
5. `git mv deny.toml apps/guardrail3/deny.toml`
6. `git mv local apps/guardrail3/local`
7. `git mv tests apps/guardrail3/tests` (integration tests belong to the crate)
8. `git mv golden-tests apps/guardrail3/golden-tests`
9. Keep at root: `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`
10. Create new root `Cargo.toml` as workspace (with `[workspace]` members and `[workspace.lints]`)
11. Update `apps/guardrail3/Cargo.toml`: remove `[workspace.lints.*]`, add `[lints] workspace = true` referencing workspace root
12. Update `guardrail3.toml`: add `[rust.crates.guardrail3]` section
13. Create `packages/.gitkeep`
14. Run `cargo test` — all tests must pass
15. Run `cargo run -p guardrail3 -- rs validate .` — R-ARCH-04 should not fire

### Key decisions
- **Tests move with the crate**: integration tests in `tests/` reference crate internals, they belong to the crate package
- **Lints stay in workspace root Cargo.toml**: `[workspace.lints]` defined at root, crate uses `[lints] workspace = true`
- **fuzz/ stays at root**: fuzz targets are workspace-level tooling
- **golden-tests move with crate**: they test the binary specifically

## Files to Modify
- Root `Cargo.toml` — new workspace file
- `apps/guardrail3/Cargo.toml` — moved, lints section adjusted
- `guardrail3.toml` — add crate config
