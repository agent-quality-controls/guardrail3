# Build code config lane

## Goal

Implement the missing `code` config lane:

- add `g3rs-code-config-checks`
- implement `g3rs-code-ingestion::ingest_for_config_checks`
- preserve current live behavior for:
  - `RS-CODE-07` EXCEPTION comment inventory
  - `RS-CODE-12` workspace `unsafe_code` lint level
- prove the full `crawl -> ingest_for_config_checks -> check` path with tests

## Approach

1. Create `packages/rs/code/g3rs-code-config-checks`
   - standard package scaffold:
     - root facade
     - `crates/types`
     - `crates/assertions`
     - `crates/runtime`
   - runtime owns fan-out from one config input into the two rules

2. Define the public config input in `g3rs-code-config-checks/crates/types`
   - one family-level input object with:
     - collected exception comments
     - collected unsafe-code lint facts
   - keep it lane-specific and small

3. Replace the placeholder `G3RsCodeConfigChecksInput` in
   `g3rs-code-ingestion-types`
   - re-export the real type from `g3rs-code-config-checks-types`
   - keep the family-ingestion facade signatures honest

4. Implement `g3rs-code-ingestion::ingest_for_config_checks`
   - select owned config files from the crawl
   - read raw content for exception-comment discovery
   - parse root/workspace `Cargo.toml` with structured parsing for
     `workspace.lints.rust.unsafe_code`
   - build one `G3RsCodeConfigChecksInput`

5. Keep ownership narrow
   - `ingestion` owns:
     - crawl selection
     - file reads
     - cargo parsing
     - comment extraction
     - fact collection
   - `checks runtime` owns:
     - fan-out into tiny rule calls
   - `rules` stay tiny and pure

6. Add tests before and around implementation
   - rule-local tests in `g3rs-code-config-checks`
   - ingestion tests for selection/parsing
   - end-to-end pipeline tests in `g3rs-code-ingestion`

## Key decisions

- Use one family-level config input object, not one input per comment/file.
  - Why: the two live rules already come from repo-wide config discovery, and
    the checks runtime can still fan out into one local assertion per rule hit.

- Use the cargo parser package for `RS-CODE-12`.
  - Why: this is config semantics and should stay on a structured parser.

- Keep raw line scanning for `RS-CODE-07`.
  - Why: the rule is about comment inventory across text config files, not TOML
    semantics alone.

- Return ingestion errors for unreadable/unparseable required config inputs.
  - Why: this lane should not silently drop owned config files.

## Files to modify

- `packages/rs/code/g3rs-code-config-checks/**` (new)
- `packages/rs/code/g3rs-code-ingestion/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/error.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/select.rs`
- new helpers under `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/**`
- `.plans/todo/checks/rs/code.md` if wording or package status needs update

## Done means

- `g3rs-code-config-checks` exists and tests pass
- `g3rs-code-ingestion::ingest_for_config_checks` is real
- pipeline tests prove:
  - `RS-CODE-07`
  - `RS-CODE-12`
- `cargo test --workspace -q` passes in:
  - `packages/rs/code/g3rs-code-config-checks`
  - `packages/rs/code/g3rs-code-ingestion`
