# Goal

Make every Rust package under `packages/ts`, `packages/shared`, and `packages/parsers` pass `g3rs validate`.

# Approach

- Collect current `g3rs` findings package-by-package.
- Fix parser/shared packages first because TS packages consume parser/shared contracts.
- Fix TS package findings by root cause:
  - dependency allowlists in `guardrail3-rs.toml`
  - test assertion placement into sibling assertions crates
  - package shape/layout findings
  - code-size/import findings only where the fix is local and architectural
- Avoid weakening rules to make findings disappear.
- Keep generated inventory warnings only if they are inventory warnings, not errors.

# Files To Modify

Expected categories:

- package-local `guardrail3-rs.toml`
- Rust source under `packages/ts/**`
- Rust source under `packages/shared/**`
- Rust source under `packages/parsers/**`
- assertions crates where tests need shared proof helpers

# Verification

- `g3rs validate --path <package>` for every package under `packages/ts`, `packages/shared`, and `packages/parsers`.
- Targeted `cargo test` for every modified package.
- `cargo fmt --check` for every modified package.
- `git diff --check`.

# Risks

- This is a broad cleanup pass. Some findings may expose pre-existing architectural debt outside the last Astro slice.
- If a package is not a valid `g3rs` workspace root, record it and validate the nearest package root instead.
