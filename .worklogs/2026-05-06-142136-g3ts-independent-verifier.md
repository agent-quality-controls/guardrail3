Summary
- Added the independent G3TS verifier script at `scripts/g3ts/verify`.
- Updated TypeScript hook source ingestion and source checks to inspect `.githooks/pre-commit` and `scripts/g3ts/verify` only.
- Added unit coverage for verifier hook routing, verifier command categories, forbidden Rust calls, argument rejection, and staged-path behavior.

Decisions made
- Kept G3TS verification independent from G3RS: the verifier never calls `g3rs` or Cargo and does not know about the Rust verifier.
- Kept source ingestion narrow per the plan by excluding `scripts/g3rs/verify`, `scripts/guardrails/*`, and modular hook scripts from source-check inputs.
- Implemented workspace mode as unfiltered verification for the configured scope, while pre-commit mode exits early when staged paths are not TypeScript-relevant.
- Used package-manager script fallbacks where present, then delegated to the underlying tools (`tsc`, `eslint`, `prettier`, `cspell`, `stylelint`, `syncpack`, `type-coverage`).

Key files for context
- `scripts/g3ts/verify`
- `packages/ts/hooks/g3ts-hooks-types/src/types.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run_tests/cases.rs`

Verification
- `bash -n scripts/g3ts/verify`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-types/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-ingestion/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-source-checks/Cargo.toml --workspace`
- `g3rs validate --path packages/ts/hooks/g3ts-hooks-types`
- `g3rs validate --path packages/ts/hooks/g3ts-hooks-ingestion`
- `g3rs validate --path packages/ts/hooks/g3ts-hooks-source-checks`
- `scripts/g3ts/verify --mode pre-commit --scope /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing`

Blocked verification
- `g3ts validate --path packages/ts/hooks/g3ts-hooks-types` failed because the installed G3TS currently reports existing missing TS app/package surfaces and stale pre-commit hook policy failures for this Rust-backed hook package.
- `g3ts validate --path packages/ts/hooks/g3ts-hooks-ingestion` failed for the same existing G3TS policy reasons.
- `g3ts validate --path packages/ts/hooks/g3ts-hooks-source-checks` failed for the same existing G3TS policy reasons.
- `scripts/g3ts/verify --mode workspace --scope /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing` failed at `g3ts validate --path "$SCOPE"` with existing landing G3TS policy failures in style, fmt, spelling, typecov, and hooks.

Next steps
- Re-run the blocked G3TS validations after the parent integration updates `.githooks/pre-commit` and the selected TypeScript scope satisfies the currently enforced G3TS families.
