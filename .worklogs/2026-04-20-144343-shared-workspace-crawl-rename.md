## Summary

Moved the neutral workspace crawl package out of `packages/rs` into `packages/shared`, renamed it from `g3rs-workspace-crawl` to `g3-workspace-crawl`, and neutralized the exported type names. Updated both Rust and TypeScript consumers to depend on the shared package instead of the Rust-branded one.

## Decisions made

- Kept one shared crawl package instead of duplicating Rust and TypeScript variants.
  - Reason: the package owns only neutral filesystem crawl facts and simple queries.
- Renamed exported types from `G3RsWorkspace*` to `G3Workspace*`.
  - Reason: leaving Rust-branded types in a shared package would still leak the wrong boundary into TS consumers.
- Moved the package to `packages/shared/g3-workspace-crawl`.
  - Reason: package location should match its permanent ownership.

## Key files for context

- `.plans/2026-04-20-143854-shared-workspace-crawl-rename.md`
- `packages/shared/g3-workspace-crawl/Cargo.toml`
- `packages/shared/g3-workspace-crawl/src/lib.rs`
- `packages/shared/g3-workspace-crawl/crates/runtime/src/lib.rs`
- `packages/shared/g3-workspace-crawl/crates/runtime/src/run.rs`
- `packages/shared/g3-workspace-crawl/crates/types/src/crawl.rs`
- `packages/shared/g3-workspace-crawl/crates/types/src/entry.rs`
- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/crates/types/app-types/src/traits.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/Cargo.toml`

## Verification

- `cargo test -q --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/shared/g3-workspace-crawl`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `cargo fmt --all --check --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml`
- `cargo fmt --all --check --manifest-path apps/guardrail3-rs/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml`

## Next steps

- Finish the ESLint parser package before adding real `ts/eslint` rules.
- Keep `g3-workspace-crawl` narrow and do not let family semantics leak into it.
