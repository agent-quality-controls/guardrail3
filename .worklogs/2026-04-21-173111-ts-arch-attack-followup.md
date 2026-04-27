## Summary

Closed the concrete `ts/arch` attack findings from the first adversarial pass. The follow-up fixed the file-tree depth math and ignored-subtree semantics at ingestion, strengthened source and file-tree proof coverage, and added direct `g3ts --family arch` CLI coverage.

## Decisions made

- Fixed the file-tree bug in ingestion, not in the rule.
  - Why: the off-by-one depth and ignored-subtree inflation were fact-generation bugs in `g3ts-arch-ingestion`.
  - Rejected: weakening `g3ts-arch/structural-split`, because the rule thresholds were already correct.

- Ignored only non-source structural subtrees under `src`.
  - Chosen exclusions:
    - `src/test`
    - `src/tests`
    - `src/__tests__`
    - `src/example`
    - `src/examples`
  - Why: these subtrees should not create structural-split pressure for the main source tree.

- Tightened proof instead of widening production behavior where the attack found test debt.
  - Added direct tests for:
    - unreadable facade
    - parse-error facade
    - clean `.tsx` facade
    - exact-threshold structural quiet path
    - `--family arch` CLI parsing
    - real `g3ts` arch output

- Strengthened source-lane assertions to pin payload, not only IDs.
  - Why: the second attack pass correctly found that ID-only assertions could hide title/message/file regressions.

## Key files for context

- `.plans/2026-04-21-172337-ts-arch-attack-followup.md`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/arch/g3ts-arch-source-checks/crates/assertions/src/run.rs`
- `packages/ts/arch/g3ts-arch-source-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/assertions/src/run.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/ts/arch/g3ts-arch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/arch/g3ts-arch-source-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/arch/g3ts-arch-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/arch/g3ts-arch-ingestion/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/arch/g3ts-arch-source-checks/Cargo.toml`
- `g3rs validate --path packages/ts/arch/g3ts-arch-ingestion`
- `g3rs validate --path packages/ts/arch/g3ts-arch-source-checks`
- `g3rs validate --path packages/ts/arch/g3ts-arch-file-tree-checks`
- `g3rs validate --path apps/guardrail3-ts`

## Adversarial review

- First attack pass found:
  - file-tree off-by-one depth
  - ignored test/example subtree inflation
  - missing unreadable/parse-error/`.tsx` proof
  - missing direct `--family arch` app-wiring proof
- Follow-up attack pass:
  - app wiring: clean
  - file-tree: clean
  - source lane: one remaining assertion-strength gap, then clean after payload assertions were tightened

## Next steps

- `ts/arch` wave 1 is converged for the current scope.
- If continuing TS structure work, the next family should be `ts/apparch`, derived from the live Rust `apparch` package split rather than stale TS planning.
