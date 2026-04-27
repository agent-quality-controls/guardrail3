## Goal

Close the `ts/arch` attack findings without widening scope.

End state:

- `g3ts-arch/structural-split` depth and directory counting match the intended Rust-mirrored semantics for wave 1.
- source tests prove unreadable facade, parse-error facade, and `.tsx` facade handling.
- `g3ts` CLI tests prove `--family arch` end to end.

## Approach

1. Add proof first.
   - Extend `g3ts-arch-ingestion` and `g3ts-arch-file-tree-checks` tests to prove:
     - direct child of `src` counts as depth `1`
     - deep tree over threshold fires
     - `src/tests`, `src/__tests__`, and `src/examples` do not count
   - Extend `g3ts-arch-source-checks` tests to prove:
     - unreadable facade surfaces `g3ts-arch/facade-parseable`
     - parse-error facade surfaces `g3ts-arch/facade-parseable`
     - `.tsx` parsed facade stays clean when valid
   - Extend `g3ts` CLI tests to prove:
     - CLI parses `--family arch`
     - real default wiring emits `arch` findings when expected
2. Fix file-tree ingestion at the architectural root.
   - Correct depth math in `file_tree.rs`.
   - Exclude non-source test/example subtree names from structure counts.
3. Keep changes local.
   - no rule inventory expansion
   - no new families
   - no target-repo changes

## Key Decisions

- Keep the existing wave-1 structure thresholds.
  - Only align measurement semantics with Rust-like intent.
- Treat `src/tests`, `src/test`, `src/__tests__`, `src/examples`, and `src/example` as ignored structural subtrees in wave 1.
  - This follows the same principle as Rust ignoring test/example structure noise.

## Files To Modify

- `packages/ts/arch/g3ts-arch-ingestion/**`
- `packages/ts/arch/g3ts-arch-source-checks/**`
- `packages/ts/arch/g3ts-arch-file-tree-checks/**`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `.worklogs/**`
