Summary

Tightened the TS family seams so the public family inputs no longer expose full parser documents. `ts/eslint` now converts parser output into a family-owned snapshot before checks see it, `ts/tsconfig` now carries only derived strict-baseline facts plus effective compiler options, and the live `tsconfig` external-extends bypass is gone.

Decisions made

- Fixed the seam at the family boundary instead of changing the parser crates.
  - Why: the parser crates already matched the repo `Document { raw, typed }` pattern. The leak was the family API carrying parser documents directly.
  - Rejected: pushing more policy into parser crates.
- Kept `ts/eslint` on a single explicit target-root config for the current wave.
  - Why: `g3ts validate --path <root>` currently validates one explicit target root at a time, and the important current targets are app/package roots like `web` and `landing`.
  - Rejected: forcing a repo-global multi-surface ESLint design into this bug-fix pass. The old plan language was broader than the current runner semantics.
- Scoped ESLint probes to the selected config root anyway.
  - Why: even in the single-root model, probes should not drift across sibling package trees inside the crawl.
- Replaced the external `tsconfig` baseline deferral with a hard error.
  - Why: allowing external `extends` to skip the strict-baseline check was a real bypass.
  - Rejected: keeping the old info-only deferral.
- Corrected the `tsconfig` resolver to use the active traversal stack.
  - Why: that is the correct resolver shape for inheritance graphs and removes accidental branch-to-branch coupling.

Key files for context

- `.plans/2026-04-21-112615-fix-ts-family-boundaries.md`
- `packages/ts/eslint/g3ts-eslint-types/src/types.rs`
- `packages/ts/eslint/g3ts-eslint-types/src/convert.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-types/src/types.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-types/src/flags.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-ingestion/crates/runtime/src/run.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-ingestion/crates/runtime/src/resolve.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/support.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/ts_tsconfig_config_05_strict_baseline.rs`

Verification

- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/tsconfig/g3ts-tsconfig-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/tsconfig/g3ts-tsconfig-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/tsconfig/g3ts-tsconfig-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-types`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`
- `g3rs validate --path packages/ts/tsconfig/g3ts-tsconfig-types`
- `g3rs validate --path packages/ts/tsconfig/g3ts-tsconfig-ingestion`
- `g3rs validate --path packages/ts/tsconfig/g3ts-tsconfig-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/web`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing`
- adversarial review:
  - parser-boundary attack: clean
  - `tsconfig` family attack: clean
  - remaining ESLint note is about an older broader plan, not a live blocker for the current explicit-target-root runner

Next steps

- Build `ts/package` next.
- If `g3ts` later needs repo-root multi-surface ESLint validation, add a separate lane for local-config discovery instead of overloading the current single-root config-check input.
