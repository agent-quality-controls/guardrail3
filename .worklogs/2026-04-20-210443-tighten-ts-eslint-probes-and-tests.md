Summary

Tightened the `ts/eslint` family where the attack review found real gaps. Ingestion now skips ignored, unreadable, test, script, and config-like files when choosing representative source probes; config checks now prove JS carve-outs more honestly and validate TSX baseline parity when a real TSX source file exists; and the tests now assert exact result sets instead of subset-only membership.

Decisions made

- Fixed probe-selection bugs in ingestion, not in config checks.
  - Reason: source-probe correctness is an orchestrator responsibility. Rules should consume better facts, not compensate for bad probe choices.
- Kept the family on effective-config semantics rather than source-shape/preset-name matching.
  - Reason: the durable contract is what ESLint actually enforces, not whether the user wrote one exact preset spread.
- Added one grouped TSX parity rule instead of cloning every TS rule for TSX.
  - Reason: this tightens the real coverage gap without exploding the runtime crate into duplicate rule files.
- Tightened the JS carve-out against representative type-aware rules only.
  - Reason: `disableTypeChecked` does not turn off non-type-aware rules like `@typescript-eslint/no-explicit-any`, so that rule does not belong in the JS carve-out proof.
- Did not widen this pass into JSX/React policy ownership.
  - Reason: React/jsx-a11y remain outside the current `ts/eslint` family cut.

Key files for context

- `.plans/2026-04-20-201145-tighten-ts-eslint-probes-and-tests.md`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_15_js_carveout.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_17_tsx_source_parity.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`

Verification

- `cargo test --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/parsers/eslint-config-parser/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/parsers/eslint-config-parser`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-config-checks`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/web --family eslint`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family eslint`
- final adversarial review:
  - no material blockers remain in current scope
  - JSX-specific policy remains intentionally out of scope

Next steps

- Start `ts/tsconfig`.
- If we later bring React/jsx-a11y into current family ownership, add a dedicated JSX probe instead of overloading the current TS/TSX probes.
