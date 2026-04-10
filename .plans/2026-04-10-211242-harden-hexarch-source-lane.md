# Goal

Bring the extracted `hexarch` source lane up to old app-family behavior for `RS-HEXARCH-22` and `RS-HEXARCH-23`.

The end state is:

- source ingestion selects only real hexarch member crates under app workspaces
- one malformed member does not fail the whole lane
- crates with no source entrypoint are skipped the same way as the app family
- source tests cover the old reachable-module, visibility, and fail-closed attack surface
- package pipeline tests assert exact outputs instead of only rule IDs

# Approach

1. Fix ingestion scope in `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`.
   - Discover app workspace roots under `apps/*`.
   - Parse workspace `members` and resolve them to member crate dirs.
   - Stop treating every `Cargo.toml` under `apps/` as a source crate.

2. Fix per-crate fail-closed behavior.
   - Parse member manifests inside per-crate summarization.
   - Turn member manifest parse/read problems into `source_error_*` on that crate instead of returning a lane-wide ingestion error.
   - Skip crates with no explicit entrypoint and no `src/`.

3. Harden source discovery behavior with tests first.
   - Add tests for:
     - app root workspace manifest exclusion
     - mixed good + bad crates in one lane
     - `[[bin]].path` and `src/main.rs`
     - `#[path = "..."]`
     - `foo/mod.rs`
     - `cfg(test)` and visibility edges
     - unreadable/invalid member manifest and unreadable source

4. Strengthen rule-local tests and pipeline assertions.
   - Assert exact file, severity, inventory, and titles for touched cases.
   - Restore the old non-adapter and non-ports negatives explicitly.

5. Re-run adversarial review on the hardened slice.

# Key decisions

- Match the old app-family boundary before expanding behavior.
  - Rejected: keeping the broader `all Cargo.toml under apps/` selection.

- Keep manifest failures inside source-rule fail-closed output for this lane.
  - Rejected: lane-wide ingestion errors for a single bad member.

- Port the old test matrix into package tests where the behavior now lives.
  - Rejected: relying on a thin package smoke test.

# Files to modify

- `.plans/2026-04-10-211242-harden-hexarch-source-lane.md`
- `.worklogs/...-harden-hexarch-source-lane.md`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/**`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/mod.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/mod.rs`
