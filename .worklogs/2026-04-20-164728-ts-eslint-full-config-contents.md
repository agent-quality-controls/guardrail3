Summary

Expanded `g3ts-eslint-config-checks` from the first narrow rule wave into a fuller config-content family. The package now enforces grouped TS baseline rule sets, test and JS carve-outs, and keeps the parser/types/ingestion/config-checks roots clean under `g3rs validate`.

Decisions made

- Kept the existing parser and ingestion boundary intact. `config-checks` still consumes one parsed ESLint root document and does not reintroduce discovery logic.
- Added grouped config-content checks instead of scattering more one-off booleans through ingestion. This keeps actual policy enforcement in the check lane.
- Split the heavier rule slice under `crates/runtime/src/full_config/` instead of suppressing the Rust family structural finding. The runtime root had exceeded sibling-file limits, so the correct fix was a module seam, not a waiver.
- Duplicated the grouped baseline constants inside `run_tests/helpers.rs` instead of importing sibling production modules from the run sidecar. That preserves the owned-sidecar contract required by Rust guardrails.

Key files for context

- `.plans/2026-04-20-163455-ts-eslint-full-config-contents.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/full_config/mod.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/full_config/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/full_config/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`

Next steps

- Tighten the actual ESLint baseline contents against the local TS ledgers and the latest ESLint/typescript-eslint docs. The current grouped rule sets are real, but they are still a first pass.
- Add more probe-specific checks for JS, tests, and config-file targets where the baseline should differ.
- Decide which plugin-stack and package-presence assertions belong in `ts/eslint` versus the future `ts/package` family.
