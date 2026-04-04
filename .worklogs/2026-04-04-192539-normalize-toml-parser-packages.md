# Normalize TOML Parser Packages

**Date:** 2026-04-04 19:25
**Scope:** `packages/rustfmt-toml/`, `packages/nextest-toml/`, `packages/mutants-toml/`, `packages/clippy-toml/`, `packages/deny-toml/`

## Summary
Normalized the remaining TOML parser packages to the same facade/workspace structure already established for `rust-toolchain-toml`. The packages now share the same internal split (`crates/parser/{runtime,assertions,types}`), the same free-function parser API shape, and the same file-based top-level type naming (`*Toml`).

## Context & Problem
After `rust-toolchain-toml` was normalized, the other parser packages were still inconsistent in multiple ways:
- some still used the old single-inner-crate layout
- some exposed top-level parsed document types as `*Config`
- some had only partial normalization and still mixed old and new structure

The user wanted the parser packages treated as real released facade packages, with the public interface only at the package root and the implementation hidden behind internal crates. They also explicitly wanted the parsed document types named after the file, not generically as “config”.

## Decisions Made

### Make Every Parser Package A Facade Workspace
- **Chose:** Normalize each package to a publishable root facade crate plus internal crates under `crates/parser/{runtime,assertions,types}`.
- **Why:** This matches the package architecture the user wants: a single public dependency surface per parser package, with implementation factored internally but still publishable for Cargo resolution.
- **Alternatives considered:**
  - Keep virtual workspace roots with one inner crate — rejected because that bypasses the facade-package architecture.
  - Keep mixed shapes across packages — rejected because it leaves the parser family hard to reason about and hard to automate against.

### Standardize On Free Parser Functions
- **Chose:** Keep `parse(...)` and `from_path(...)` as free functions in the runtime crate, re-exported through the facade crate.
- **Why:** This preserves the clean dependency direction `runtime -> types` and keeps the `types` crate as pure data models.
- **Alternatives considered:**
  - Put `FromStr` / `from_path` on the top-level parsed type — rejected because it drags runtime IO/error concerns into the shared types layer.

### Rename Top-Level Parsed Document Types To `*Toml`
- **Chose:** Rename the top-level parsed document types to file-based names:
  - `RustfmtToml`
  - `NextestToml`
  - `MutantsToml`
  - `ClippyToml`
  - `DenyToml`
- **Why:** This matches the naming rule the user clarified during the session and makes the package APIs consistent with `RustToolchainToml`.
- **Alternatives considered:**
  - Keep `*Config` for flat files and only use `*Toml` for sectioned files — rejected because it creates unnecessary naming drift across the parser family.

### Fully Migrate `deny-toml`
- **Chose:** Complete `deny-toml` as the last remaining old-layout parser package and remove the obsolete `crates/deny-toml` inner crate.
- **Why:** `deny-toml` was the only unfinished parser package after the other migrations. Leaving both the new and old structures in the tree would be ambiguous and brittle.
- **Alternatives considered:**
  - Defer `deny-toml` because it has more section types — rejected because the section types were already plain data and split cleanly into `types`.
  - Preserve the old `DenyConfig` constructor shape — rejected because nothing in the repo depended on it outside that package’s own old tests.

## Architectural Notes
Each normalized parser package now follows the same layering:

- root facade crate: the only intended public dependency surface
- `crates/parser/runtime`: parse entrypoints, file IO boundary, runtime-local sidecar tests
- `crates/parser/assertions`: reusable semantic proof helpers for parser tests
- `crates/parser/types`: shared parsed document model, marked `shared = true`

This gives a uniform package shape for future extracted content-check crates to consume.

The known repo-level caveat still remains: the Rust `test` family is internally inconsistent about owned sidecar discovery versus allowed shape. These parser packages build and lint cleanly, but package-scoped `test` validation still reports the previously identified `RS-TEST-02/03/07` mismatch. That is a guardrail-family issue, not a package-local structural inconsistency unique to one parser.

## Information Sources
- `.worklogs/2026-04-04-184131-rust-toolchain-toml-parser-package.md` — specimen normalization and previous parser-package decisions.
- `packages/rust-toolchain-toml/` — reference package shape for facade/runtime/assertions/types split.
- `packages/clippy-toml/`, `packages/rustfmt-toml/`, `packages/nextest-toml/`, `packages/mutants-toml/`, `packages/deny-toml/` — migrated parser packages.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover/components.rs` — current sidecar discovery behavior.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_02_owned_sidecar_shape/rule.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_03_runtime_assertions_split/rule.rs`

## Open Questions / Future Considerations
- Reconcile the `test` family so parser package sidecar/assertions layout can be fully green without working around contradictory discovery/rule expectations.
- Decide whether the package-root `Cargo.lock` files should continue to be committed for these standalone parser workspaces or be reconsidered as a repo-wide policy.
- Apply the same naming and packaging expectations to any future parser packages (`cargo-config-toml`, `guardrail3-toml`) from the start instead of normalizing them later.

## Key Files for Context
- `packages/rust-toolchain-toml/Cargo.toml` — specimen facade/workspace root shape for parser packages
- `packages/rustfmt-toml/Cargo.toml` — normalized flat-file parser package facade root
- `packages/nextest-toml/Cargo.toml` — normalized sectioned parser package facade root
- `packages/mutants-toml/Cargo.toml` — normalized hidden-file parser package facade root
- `packages/clippy-toml/Cargo.toml` — normalized clippy parser package facade root
- `packages/deny-toml/Cargo.toml` — fully migrated final parser package facade root
- `packages/deny-toml/crates/parser/runtime/src/parser.rs` — final `deny-toml` free-function parser API
- `packages/deny-toml/crates/parser/types/src/deny_toml.rs` — final `DenyToml` top-level parsed document type
- `.worklogs/2026-04-04-184131-rust-toolchain-toml-parser-package.md` — backstory for the first normalized parser package

## Next Steps / Continuation Plan
1. Commit this parser-package normalization batch without unrelated repo changes (`.gitignore`, the handoff worklog edits, or the stray untracked old deny worklog).
2. Start the first extracted checks package on top of the now-unified parser package surfaces, beginning with toolchain content checks.
3. When parser-package work resumes later, fix the guardrail `test` family mismatch so these packages can validate cleanly under package-scoped `--family test`.
