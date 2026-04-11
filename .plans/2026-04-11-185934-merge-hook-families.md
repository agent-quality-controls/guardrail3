Goal
- Replace the public `hooks-shared` + `hooks-rs` package split with one public Rust hook family: `g3rs-hooks`.
- Keep shell parsing and hook-selection helpers reusable privately, but remove `hooks-shared` as a checker-family boundary.
- Update hook command semantics to use `g3rs` as the binary name.

Approach
- Move the public package roots from:
  - `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks`
  - `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion`
  into:
  - `packages/rs/hooks/g3rs-hooks-source-checks`
  - `packages/rs/hooks/g3rs-hooks-ingestion`
- Widen the source-check input type so one input can support both old families:
  - hook script kind
  - path
  - content
  - modular-dir presence
  - workspace-project flag
- Merge the old `hooks-shared` runtime modules into the new `g3rs-hooks-source-checks` runtime and run both old rule sets from one `check(...)` entrypoint.
- Merge ingestion behavior:
  - select effective pre-commit path
  - ingest direct modular scripts
  - compute workspace-project flag once from root `Cargo.toml`
- Rename package names, import paths, README/TODO text, and internal references from `hooks-rs` / `hooks-shared` to `hooks`.
- Update Rust hook command rules and tests from `guardrail3` to `g3rs` where they refer to the binary name.
- Remove the old public package directories after the new package builds and tests.

Key decisions
- Keep one public hook family only.
  - `hooks-shared` remains only as implementation detail, not as a public checker family.
- Do not invent a separate config or file-tree lane for hooks in this pass.
  - Merge only the current source/content packages and their ingestion.
- Keep rule IDs unchanged for now.
  - This is a package-boundary migration, not a rule-ledger renumbering pass.
- Keep `hook-shell-parser` as a separate parser crate.
  - It is a real reusable parser boundary, unlike the old public hook-family split.

Files to modify
- `packages/rs/hooks-rs/**`
- `packages/rs/hooks-shared/**`
- new `packages/rs/hooks/**`
- docs and plan references that point at the old public hook-family split
