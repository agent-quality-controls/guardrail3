## Goal

Reach an authoritative `guardrail3-rs validate` state with zero errors across the `apps/guardrail3-rs` app and every package root under `packages/`, while allowing warnings. Correct the two adversarial findings before reporting results.

## Approach

1. Verify the topology ingestion boundary fix.
   - Confirm `g3rs-topology-ingestion` no longer re-exports topology contract types from runtime.
   - Ensure assertions consume ingestion types through the local types crate.
   - Run package tests and `validate` on the package root.

2. Verify the parser warning restoration.
   - Restore comment-style `// reason:` escape hatches in the five parser packages.
   - Run package tests and `validate` on each parser root.
   - Expect warnings, not errors.

3. Regenerate the full audit.
   - Sweep `apps/guardrail3-rs`.
   - Sweep every package root under `packages/`, including both `packages/rs/*` and `packages/rs/*/*` roots plus `packages/parsers/*` and `packages/shared/*`.
   - Save the report and derive per-package warning counts.

4. Run adversarial review on the corrective commits.
   - Ask subagents to attack the new fixes for relaxed rules, hidden warnings, or boundary regressions.
   - Fix any real findings before reporting completion.

## Key decisions

- Do not change rules unless a contradiction appears. The current issues are package-state and reporting integrity problems, not rule contradictions.
- Treat parser `RS-CODE-SOURCE-04` findings as visible warnings, not something to suppress.
- Treat the stored prior full report as invalid proof because it omitted `packages/rs/*` roots.

## Files to modify

- `packages/rs/topology/g3rs-topology-ingestion/crates/types/Cargo.toml`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/topology/g3rs-topology-ingestion/crates/assertions/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/lib.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/parser.rs`
