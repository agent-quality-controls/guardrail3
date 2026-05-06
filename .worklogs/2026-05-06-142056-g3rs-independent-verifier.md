Summary
- Added `scripts/g3rs/verify` as the Rust-only verifier for pre-commit and workspace modes.
- Updated Rust hook ingestion and source checks to validate `.githooks/pre-commit` plus `scripts/g3rs/verify`, without reading G3TS verifier files or shared verifier paths.

Decisions made
- Kept G3RS verification self-contained in `scripts/g3rs/verify`; no shared dispatcher, files mode, or worktree mode was added.
- Moved Rust command enforcement from direct pre-commit command checks to the verifier script contract, so `.githooks/pre-commit` only needs the G3RS verifier invocation.
- Used the existing shell parser facts for executable command checks, with narrow source-line checks only for script argument rejection branches that are static shell control flow.
- Replaced obsolete ingestion source-selection tests with verifier-specific selection tests because Rust ingestion must now read only `.githooks/pre-commit` and `scripts/g3rs/verify`.

Key files for context
- `scripts/g3rs/verify`
- `packages/rs/hooks/g3rs-hooks-types/src/types.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run_tests/selection.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run_tests/cases.rs`

Verification
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-types/Cargo.toml --workspace`
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-ingestion/Cargo.toml --workspace`
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace`
- `g3rs validate --path apps/guardrail3-rs`
- `scripts/g3rs/verify --mode pre-commit --scope apps/guardrail3-rs`
- `scripts/g3rs/verify --mode workspace --scope apps/guardrail3-rs`
- Script rejection checks for missing `--mode`, missing `--scope`, unknown flag, `--mode worktree`, and `--mode files`.

Next steps
- Parent hook integration should add the real `.githooks/pre-commit` line for `scripts/g3rs/verify --mode pre-commit --scope apps/guardrail3-rs`.
