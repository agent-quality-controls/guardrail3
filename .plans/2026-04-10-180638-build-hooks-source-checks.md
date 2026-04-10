# Goal

Extract the hook source lanes from app code into `packages/` with the same family split used elsewhere:

- `hooks-shared` owns generic shell-hook source semantics
- `hooks-rs` owns Rust-specific hook command semantics

The extracted lanes must run end to end through family ingestion packages:

- `g3rs-hooks-shared-ingestion::ingest_for_source_checks`
- `g3rs-hooks-shared-source-checks::check`
- `g3rs-hooks-rs-ingestion::ingest_for_source_checks`
- `g3rs-hooks-rs-source-checks::check`

# Approach

1. Add one reusable shell parser package under `packages/parsers/`.
   - Reuse the app `hook_shell` parser and command-query behavior.
   - Keep parser tests so hook-family semantics do not regress.

2. Build `hooks-shared` source packages.
   - `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks`
   - `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion`
   - One source input per governed hook script.
   - Source rules only:
     - `HOOK-SHARED-04`
     - `HOOK-SHARED-10`
     - `HOOK-SHARED-11`
     - `HOOK-SHARED-13`
     - `HOOK-SHARED-14`
     - `HOOK-SHARED-15`
     - `HOOK-SHARED-16`
     - `HOOK-SHARED-18`
     - `HOOK-SHARED-19`
     - `HOOK-SHARED-20`
     - `HOOK-SHARED-21`
   - Leave structural rules out of this lane.

3. Build `hooks-rs` source packages.
   - `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks`
   - `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion`
   - One source input for the effective pre-commit script.
   - Source rules only:
     - `HOOK-RS-01`
     - `HOOK-RS-02`
     - `HOOK-RS-03`
     - `HOOK-RS-04`
     - `HOOK-RS-05`
     - `HOOK-RS-07`
     - `HOOK-RS-08`
     - `HOOK-RS-09`
     - `HOOK-RS-10`
     - `HOOK-RS-11`
     - `HOOK-RS-12`
     - `HOOK-RS-13`
     - `HOOK-RS-16`
   - Leave tool-availability rules out of this lane.

4. Keep ingestion family-wide.
   - Each hook family gets one ingestion package.
   - Real `ingest_for_source_checks`.
   - Stub `ingest_for_config_checks` and `ingest_for_file_tree_checks` for now.

5. Add end-to-end tests.
   - `crawl -> ingest_for_source_checks -> check`
   - Prove shared parser behavior.
   - Prove cross-family dependency:
     - `hooks-rs` uses the shared shell parser package
     - no duplicated shell parsing logic

# Key decisions

## One shared parser package

Do not duplicate shell parsing in `hooks-shared` and `hooks-rs`.

Why:
- both families already depend on the same executable-line contract
- parser drift would create false passes and false failures

Alternative rejected:
- local parser copies in each family

## Source, not config

Hook script body checks are source checks.

Why:
- they inspect executable command semantics
- order and execution context matter
- comments vs executable lines matter

Alternative rejected:
- force hook scripts into config semantics

## Minimal source input shape

`hooks-shared` source input should be one script at a time with:
- relative path
- script kind
- content
- `has_modular_dir`

`hooks-rs` source input should be the effective pre-commit script with:
- relative path
- content

The source runtime owns parsing and rule fan-out.

# Files to modify

- new parser package under `packages/parsers/hook-shell-parser`
- new family packages under:
  - `packages/rs/hooks-shared/`
  - `packages/rs/hooks-rs/`
- existing docs/worklogs/plans only as needed for migration records
