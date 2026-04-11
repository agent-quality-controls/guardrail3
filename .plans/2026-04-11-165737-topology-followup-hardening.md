Goal

Close the follow-up topology migration gaps found by the latest adversarial audit:

- valid `./` workspace members must not false-positive
- absolute workspace members must fail as escaping paths
- workspace-local topology must fail closed on descendant manifest input failures
- rule 16 must prove member cargo-sidecar placement end to end
- rule 11 must prove nested hybrid workspaces

Approach

- Add rule-local failing tests first for:
  - `./crates/api`
  - `./crates/*`
  - `/absolute/member`
  - nested hybrid workspace
- Add pipeline failing tests first for:
  - member `.cargo/config.toml` illegal placement
  - descendant manifest failure producing topology fail-closed output
- Fix member-pattern normalization in topology support at the checks layer.
- Add a small topology fail-closed rule in the file-tree checks package for `input_failures`.
- Update topology plan text so the extracted workspace-local subset includes fail-closed reporting.

Key decisions

- Keep fail-closed reporting in `topology`, not ingestion errors, when the root workspace is valid and only descendants fail. Otherwise topology legality gets silently weakened.
- Treat absolute member paths as escaping member paths under `RS-TOPOLOGY-13`.
- Treat leading `./` as syntactic noise and normalize it away before exact/glob member matching.

Files to modify

- `.plans/todo/checks/rs/topology.md`
- `.plans/by_family/rs/topology.md`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- new `RS-TOPOLOGY-07` rule file and tests
- existing `RS-TOPOLOGY-11/12/13/16` tests
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
