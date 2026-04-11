# Goal

Add the first extracted `topology` package boundary for the new workspace-local model:

- `g3rs-topology-types`
- `g3rs-topology-ingestion`

The real implemented lane is file-tree only for now. It should ingest one pointed workspace root into a typed input for the still-relevant workspace-legality checks:

- nested workspace under the pointed workspace is forbidden
- workspace members must exactly match real child crates
- workspace member paths must not escape the workspace root
- workspace-local family files must be legally placed

# Approach

1. Add `g3rs-topology-types`
   - define the public file-tree input shape
   - keep config/source inputs as explicit stubs
   - store the parsed root `Cargo.toml` as `CargoToml`
   - store descendant Cargo roots as summarized typed facts
   - store workspace-local family files as typed facts
   - store descendant parse/read failures as typed input failures

2. Add `g3rs-topology-ingestion`
   - follow the existing family-ingestion package layout
   - export `ingest_for_config_checks`, `ingest_for_source_checks`, `ingest_for_file_tree_checks`
   - leave config/source as not implemented
   - implement real file-tree ingestion against `G3RsWorkspaceCrawl`

3. Keep the ingestion boundary strict
   - require root `Cargo.toml`
   - parse the root manifest with `cargo-toml-parser`
   - scan only crawl entries under the pointed workspace
   - do not read anything outside the crawl
   - ignore crawl entries marked ignored
   - record descendant manifest failures in typed `input_failures` instead of aborting the whole lane

4. Prove the ingestion behavior directly
   - root manifest unreadable fails
   - root manifest parse failure fails
   - descendant manifest parse failure becomes an input failure
   - ignored descendant roots and ignored family files stay out
   - family file discovery covers the currently extracted workspace-local files

# Key decisions

- Use a family `types` crate.
  - Why: topology ingestion and future topology checks are separate packages, so the file-tree input needs one public home.
  - Rejected: hiding the input shape inside ingestion and re-deciding it again when checks are added.

- Keep descendant root facts summarized, not full parsed manifests.
  - Why: the four current topology rules only need root path identity, manifest kind, and fail-closed visibility.
  - Rejected: carrying every descendant `CargoToml` when the current rules do not need it.

- Keep descendant parse/read problems inside the file-tree input instead of returning lane-wide ingestion errors.
  - Why: malformed child manifests are part of workspace-legality reporting, not a reason to stop analyzing the rest of the pointed workspace.
  - Rejected: aborting the whole topology file-tree lane on one bad child manifest.

# Files to modify

- `.plans/2026-04-11-145332-topology-file-tree-ingestion.md`
- `packages/rs/topology/g3rs-topology-types/...`
- `packages/rs/topology/g3rs-topology-ingestion/...`
- existing topology plan notes already edited in `.plans/.../topology.md`
