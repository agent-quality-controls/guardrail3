## Summary

Fixed a real `ts/eslint` ingestion bug in fallback probe selection. On repo-root validation, the TS source probe could select nested `*/scripts/*.ts` files, which then made the family misread intentional script carve-outs as baseline violations.

## Decisions made

- Fixed the bug in probe selection, not in the ESLint rules.
  - Why: the root cause was that fallback probe selection only excluded `scripts/` at the workspace root.
  - Rejected: weakening `TS-ESLINT-CONFIG-09`, because the rule itself was correct.

- Broadened the exclusion to all nested script trees.
  - Chosen logic:
    - `scripts/*`
    - `*/scripts/*`
  - Applied to fallback:
    - TS source
    - TSX source
    - JS source

- Proved the bug first with a unit test.
  - Added coverage showing that `apps/landing/scripts/extract-content-schema.ts` must not become the repo-root TS source probe when a real source file exists.

## Key files for context

- `.plans/2026-04-21-130931-ts-foundation-attack-and-tightening.md`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select_tests/cases.rs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/eslint.config.mjs`

## Verification

- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `timeout 120 cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher --family eslint`
- `timeout 180 cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher`

## Next steps

- `websmasher` repo root is clean under the current `g3ts` family set:
  - `eslint`
  - `tsconfig`
  - `package`
  - `npmrc`
- The target repo root ESLint config was also updated locally to use `projectService: true`.
