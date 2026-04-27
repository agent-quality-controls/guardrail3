# Deps Config Ingestion Rewrite

**Date:** 2026-04-07 18:51
**Scope:** `.plans/by_family/rs/deps.md`, `.plans/todo/checks/rs/deps.md`, `packages/rs/deps/g3rs-deps-types`, `packages/rs/deps/g3rs-deps-config-checks`, `packages/rs/deps/g3rs-deps-config-ingestion`

## Summary
Rewrote the `deps` package contract around normalized config facts and removed the stale app-config dependency from the package layer. Added a new `g3rs-deps-config-ingestion` package that reads workspace `Cargo.toml`, required workspace `guardrail3-rs.toml`, and member `Cargo.toml` files from a workspace crawl and emits per-crate config-check inputs.

## Context & Problem
The existing `packages/rs/deps/*` crates still used app-owned `GuardrailConfig` and pushed raw local-path discovery state into config checks. That contradicted the target architecture discussed in this session:

- package-layer deps code must not depend on app types
- config checks should consume normalized dependency facts, not raw manifests
- local path target structure belongs to file-tree ownership, not config-check ownership
- `guardrail3-rs.toml` is the package-layer policy input

We also discovered one concrete contract gap while implementing: the current `guardrail3-rs.toml` parser normalizes missing and empty `allowed_deps` to the same `Vec<String>` shape, so config checks cannot tell whether an allowlist was explicitly present unless ingestion carries that as a separate boolean.

## Decisions Made

### Normalize deps config inputs before checks
- **Chose:** Replace the old deps config input with a per-crate normalized shape containing crate identity, profile, explicit `allowlist_present`, `allowed_deps`, and normalized dependency entries.
- **Why:** The config rules only need resolved dependency identity plus section ownership. They do not need raw manifests, app config types, or local path manifest bags.
- **Alternatives considered:**
  - Keep passing raw workspace/crate manifests into checks — rejected because that preserves the stale boundary and keeps normalization logic inside the checks package.
  - Keep local path manifest data in config inputs — rejected because those cases belong to file-tree ownership and were the main reason the input shape became incoherent.

### Add explicit `allowlist_present`
- **Chose:** Carry `allowlist_present: bool` in `G3RsDepsConfigChecksInput`.
- **Why:** `g3rs-deps/library-allowlist-present` needs to distinguish “no allowlist configured” from “allowlist configured but empty”, and the current parser does not preserve that distinction by itself.
- **Alternatives considered:**
  - Infer presence from `allowed_deps.is_empty()` — rejected because it is wrong for explicit empty lists.
  - Change the parser package first — rejected for this step because the ingestion package can recover presence from raw TOML without widening the current parser scope.

### Keep config ingestion real, keep AST/file-tree stubbed
- **Chose:** Build real `ingest_config` for deps now, while `ingest_ast` and `ingest_file_tree` stay explicit stubs.
- **Why:** This matches the repo-wide ingestion contract already established in the prior commit and unblocks the real missing family lane without pretending file-tree ownership is solved.
- **Alternatives considered:**
  - Wait until file-tree checks exist before building config ingestion — rejected because it keeps the whole deps family blocked on a later lane.
  - Smuggle local target validation back into config ingestion output — rejected because that recreates the boundary problem we were fixing.

### Treat in-workspace path dependencies as non-config output
- **Chose:** During config normalization, skip path dependencies that resolve inside the crawled workspace root, including real workspace members and non-member in-tree targets.
- **Why:** Those are file-tree classification/validity cases. Config checks should see only external dependency facts.
- **Alternatives considered:**
  - Keep parsing local target manifests to recover package names in config checks — rejected because it reintroduces raw file-tree behavior into config-only packages.
  - Emit in-tree non-member path targets as config failures — rejected because that mixes file-tree failure ownership back into config checks.

## Architectural Notes
The new package-layer split is now:

- `g3rs-deps-types` owns normalized shared input types
- `g3rs-deps-config-checks` owns pure config checks against those normalized facts
- `g3rs-deps-config-ingestion` owns crawl selection, parsing, workspace-member discovery, workspace dependency resolution, and normalization into per-crate config inputs

`g3rs-deps-config-ingestion` follows the same outer scaffold as the other ingestion packages in the repo:

- facade crate at the package root
- `crates/types` for the ingestion error surface
- `crates/assertions` as the standard test-helper anchor crate
- `crates/runtime` split into `fs`, `select`, `parse`, `ingest`, `run`
- runtime sidecar tests under `src/ingest_tests`

The current normalization intentionally stops short of file-tree semantics. It resolves:

- simple dependencies
- renamed dependencies via `package = "..."`
- `workspace = true` through `[workspace.dependencies]`
- target-specific dependency tables

It currently skips path dependencies that stay inside the crawled workspace root, because those belong to the future file-tree lane. Path dependencies that resolve outside the workspace root still normalize by declared package identity (`package` if present, otherwise alias), which keeps config checks useful for vendored/sibling-repo style cases without dragging local target manifests into the input contract.

## Information Sources
- `.plans/by_family/rs/deps.md` — updated family target plan
- `.plans/todo/checks/rs/deps.md` — updated historical ledger with target package architecture note
- `.worklogs/2026-04-07-150226-session-handoff.md` — prior session handoff for pipeline state
- `.worklogs/2026-04-07-161057-split-ingestion-entrypoints.md` — prior ingestion entrypoint standardization work
- `packages/rs/cargo/g3rs-cargo-config-ingestion` — scaffold specimen for the new ingestion package shape
- `packages/parsers/guardrail3-rs-toml-parser` — current typed parser shape and its `allowed_deps` behavior
- `packages/rs/g3rs-workspace-crawl` — crawl types and selection helpers

## Open Questions / Future Considerations
- Local path dependency classification is still incomplete by design. In-workspace path target validation needs its own file-tree package/lane.
- External path dependencies normalized only by alias/package may still miss true package identity when `package` is omitted and the target manifest name differs. That is another reason the future file-tree lane should produce richer normalized facts.
- The current `guardrail3-rs.toml` parser may eventually want to preserve presence for policy keys like `allowed_deps` directly, which could remove the need for the ingestion-layer `allowlist_present` recovery step.

## Key Files for Context
- `.plans/by_family/rs/deps.md` — target deps family architecture and next steps
- `.plans/todo/checks/rs/deps.md` — detailed rule ledger with updated package target note
- `packages/rs/deps/g3rs-deps-types/src/input.rs` — new normalized deps config input contract
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/support.rs` — simplified checks helpers over normalized facts
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/run.rs` — ingestion orchestration entrypoint
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest.rs` — dependency normalization logic
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/select.rs` — workspace member selection from crawl + workspace manifest
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/parse.rs` — config parsing and `allowlist_present` recovery
- `.worklogs/2026-04-07-150226-session-handoff.md` — earlier session background on why `deps` was still missing
- `.worklogs/2026-04-07-161057-split-ingestion-entrypoints.md` — prior standardized ingestion API work

## Next Steps / Continuation Plan
1. Build the deps file-tree lane for local path target validation, including missing target manifests, malformed local target `Cargo.toml`, missing `[package].name`, and in-workspace non-member path targets.
2. Decide whether the future file-tree lane should feed richer normalized external dependency facts back into config ingestion for outside-workspace path deps whose real package identity differs from the declared alias.
3. Once file-tree ownership exists, wire the RS-specific app runner to `g3rs-deps-config-ingestion` + `g3rs-deps-config-checks` and delete the remaining legacy deps package assumptions instead of adapting app-family mapper code.
