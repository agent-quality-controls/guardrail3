Goal

Restore the `rs/garde` source lane to the intended architecture:
- ingestion owns source-file reading, parsing, and cross-file garde analysis
- source checks consume ingestion-owned analyzed inputs
- source checks stop rebuilding AST/global resolution from raw file paths

Approach

1. Prove the current seam defect with a run-level test in `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run_tests/cases.rs`.
   - Add a red test that passes prebound analyzed inputs and fails while `run.rs` still ignores them.
2. Move garde source analysis out of `g3rs-garde-source-checks`.
   - Add ingestion-owned analysis module under `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src`.
   - Reuse the existing garde AST parsing logic by relocating it from source checks into ingestion instead of duplicating it.
3. Narrow `g3rs-garde-types` source input.
   - Replace raw `source_files: Vec<G3RsSourceFile>` with analyzed-file and derived-check inputs owned by the family.
   - Keep config-family surfaces intact; only change the source lane.
4. Rewrite `g3rs-garde-source-checks` to pure dispatch.
   - `run.rs` should iterate ingestion-owned derived inputs only.
   - Remove source-file IO and parser modules from source checks.
5. Repair tests and assertions.
   - Move any run-level result assertions into the assertions crate where needed.
   - Keep rule-local fixtures working without reintroducing check-local parsing.

Key decisions

- Do not slice config documents in ingestion. This repair is source-lane only.
- Do not keep a second garde parser in source checks. The parser/analysis should live on the ingestion side.
- Prefer moving the existing parse/analysis code into ingestion over inventing a new representation.

Files to modify

- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- new ingestion analysis/parse files under `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run_tests/cases.rs`
- source-check assertions crate files if run-level helpers are needed
