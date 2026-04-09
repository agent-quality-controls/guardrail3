# Implement code AST profile resolution

**Date:** 2026-04-09 13:15
**Scope:** `packages/rs/code/g3rs-code-ast-checks/crates/types/src/lib.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/support.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_*/rule_tests/helpers.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_30_input_failures/rule_tests/direct.rs`, `packages/rs/code/g3rs-code-ast-ingestion/README.md`, `packages/rs/code/g3rs-code-ast-ingestion/TODO.md`, `packages/rs/code/g3rs-code-ast-ingestion/crates/assertions/src/common.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/Cargo.toml`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/classify.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/run.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/select.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/types/src/error.rs`, `packages/rs/code/g3rs-code-ast-ingestion/Cargo.lock`

## Summary
Implemented real profile classification for the `code` AST ingestion lane. The ingestion package now parses nearest owning `Cargo.toml` files, classifies selected Rust files as `library` or `binary` when that ownership is clear, marks the exact library root file, and fails ingestion on malformed owning manifests instead of silently leaving profile-sensitive rules blind.

## Context & Problem
The extracted `g3rs-code-ast-checks` package already covered most single-file rules, but the remaining API-shape rules need target context. The existing ingestion cut left `profile_name` as `None` for every file, which meant the lane could parse source but could not tell whether `lib.rs` or another module belonged to a library target. The next migration batch would have been forced either to guess from path strings in the checks runtime or to reintroduce workspace discovery there. The accepted plan in `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md` was to keep that classification in ingestion.

## Decisions Made

### Classified files from nearest owning Cargo manifest
- **Chose:** For each selected `.rs` file, find the nearest ancestor `Cargo.toml`, parse it, derive library and binary roots, and classify the file from that package context.
- **Why:** This keeps target ownership in the ingestion layer where workspace and Cargo mapping already belong.
- **Alternatives considered:**
  - Infer everything from `src/lib.rs` / `src/main.rs` string checks inside the checks runtime — rejected because it would smear Cargo logic into the AST package and break the lane split.
  - Build full module-graph ownership for every source file — rejected because it is heavier than needed for the next `code` rules and would slow down this migration step.

### Added one explicit `is_library_root` flag instead of overloading `profile_name`
- **Chose:** Extend `G3RsSourceFile` with `is_library_root: bool` while keeping `profile_name` as `Some("library")`, `Some("binary")`, or `None`.
- **Why:** The next rules need two separate answers: “is this file library-owned?” and “is this the actual library root file?” A separate boolean keeps those concerns clear.
- **Alternatives considered:**
  - Encode rootness inside profile strings like `library-root` — rejected because that mixes two axes into one field and makes later rule logic brittle.
  - Leave only `profile_name` and let rules special-case `rel_path == "src/lib.rs"` — rejected because target roots may be custom and the whole point was to avoid raw path guessing in rules.

### Failed ingestion on malformed owning manifests
- **Chose:** Add `ParseFailed` to the ingestion error type and use it when the nearest owning `Cargo.toml` cannot be parsed.
- **Why:** Unknown profile context is acceptable only when no owning manifest is present. Once a source file is clearly under a manifest that is unreadable or malformed, silently returning `None` would make profile-sensitive rules fail open.
- **Alternatives considered:**
  - Leave malformed manifests as `profile_name = None` — rejected because that would hide real classification failures.
  - Fail on every unrelated malformed `Cargo.toml` in the crawl — rejected because the implementation only needs manifests that actually own selected source files.

### Kept the first implementation intentionally narrow
- **Chose:** Resolve obvious library and binary ownership from Cargo targets and conventional source trees, without trying to model every exotic target/module case.
- **Why:** The immediate goal is to unblock the next `code` AST rule batch. A small correct-enough classifier plus focused tests is better than dragging in full target graph logic before the rules even exist.
- **Alternatives considered:**
  - Solve all custom target/module ownership in one pass — rejected because it would balloon this step and delay rule migration.

## Architectural Notes
The lane split remains:
- workspace crawl discovers files
- `g3rs-code-ast-ingestion` reads manifests and source content, and now also attaches `profile_name` and `is_library_root`
- `g3rs-code-ast-checks` still only parses bounded source content and slices rule-local facts

To support future rules without tripping `dead_code`, the checks runtime input now carries `profile_name` and `is_library_root`, but current migrated rules do not read them yet. That keeps the context available for the next migration step without forcing premature rule changes.

The classifier chooses the nearest ancestor `Cargo.toml` per selected source file, parses only those relevant manifests, then derives:
- one optional library root
- zero or more binary roots
- obvious library-owned files
- obvious binary-owned files

Test files still keep `profile_name = None`, because the profile-sensitive rules we are targeting are not supposed to treat test-owned code as library API.

## Information Sources
- `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md` — implementation plan for this step
- `packages/parsers/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs` — Cargo target model used for `[lib]`, `[[bin]]`, `autolib`, and `autobins`
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/*` — current ingestion scaffold and stub profile resolver
- `apps/guardrail3/crates/app/rs/families/code/README.md` — current family behavior notes, especially around which policy surfaces are still live
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/*.rs` and `inventory/rs_code_29_large_trait_inventory/rule.rs` — legacy rule behavior used to sanity-check which remaining rules really need target context
- `.worklogs/2026-04-09-124022-plan-code-ast-profile-resolution.md` — planning context immediately before implementation

## Open Questions / Future Considerations
- Some notes in the old `code` ledger are stale: not every remaining unmigrated rule still appears to require library-only gating. Re-check that boundary before porting `RS-CODE-29`, `31`, and `33`.
- The current classifier intentionally uses path-based target ownership heuristics inside a package once the manifest is known. If later rules need exact module graph ownership for mixed lib/bin packages, add that as a separate refinement rather than widening this first cut silently.
- The new classifier does not yet have broad real-fixture proof for custom `[lib] path` and explicit `[[bin]] path` shapes. The package TODO now calls that out.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/types/src/lib.rs` — source-file contract now includes `is_library_root`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/support.rs` — runtime-local rule input now carries the new profile context
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/classify.rs` — the new Cargo-target-based classifier
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/select.rs` — where selected source files get profile context attached
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs` — direct proof for library root, library module, binary root, unowned file, and malformed manifest behavior
- `packages/rs/code/g3rs-code-ast-ingestion/README.md` — package behavior after this change
- `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md` — the plan this implementation follows
- `.worklogs/2026-04-09-124022-plan-code-ast-profile-resolution.md` — planning worklog immediately before code changes

## Next Steps / Continuation Plan
1. Re-check the remaining unmigrated `code` rules against live legacy behavior and split them into:
   - actually library-root-only
   - library-owned but not root-only
   - not profile-sensitive after all
2. Add coverage for custom target paths in `g3rs-code-ast-ingestion`:
   - `[lib] path = "..."`,
   - `[[bin]] path = "..."`,
   - pure workspace root plus nested member package ownership
3. Migrate the next profile-aware `code` AST rules using the new fields instead of path guessing:
   - `RS-CODE-26` if it is still owned here,
   - `RS-CODE-27`,
   - `RS-CODE-29`,
   - `RS-CODE-31`,
   - `RS-CODE-33`
