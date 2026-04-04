# Build guardrail3-rs-toml-parser

**Date:** 2026-04-04 23:19
**Scope:** `packages/guardrail3-rs-toml-parser/`, `.plans/2026-04-04-142741-new-parsers.md`

## Summary
Built a new `guardrail3-rs-toml-parser` package in the same facade/workspace shape as the existing `*-toml-parser` packages. The parser models the current agreed workspace-local Rust config shape: one workspace profile, root-level check toggles, dependency allowlist, excluded paths, and generic per-rule waivers.

## Context & Problem
The repo already had normalized parser packages for Rust-facing TOML files, but the new workspace-local Rust guardrail config format did not yet have a parser package. We first narrowed the schema in discussion: the new file should configure exactly one workspace, not recreate the old repo-global `apps`/`packages` split, and the old generic `escape_hatches` tuple was replaced with a cleaner `[[waivers]]` envelope keyed by `rule`, `file`, and `selector`.

The user also wanted the package to follow the exact normalized parser workspace structure already used in the repo, rather than inventing a new one-off layout. That meant matching the current `*-parser` package shape, public facade API, internal `crates/parser/{runtime,assertions,types}` split, and sidecar parser test pattern.

## Decisions Made

### Use The Current `*-parser` Workspace Shape
- **Chose:** Build `packages/guardrail3-rs-toml-parser/` in the same layout as `rustfmt-toml-parser` and `cargo-config-toml-parser`.
- **Why:** The repo has already converged on that package shape. Matching it keeps future parser package automation and family integration consistent.
- **Alternatives considered:**
  - Reuse an older `*-toml` facade naming/layout — rejected because the current tree has already moved to `*-parser`.
  - Build only a single crate without internal split — rejected because it would drift from the normalized parser package standard.

### Model One Workspace Per File
- **Chose:** Encode one root `profile`, one root `allowed_deps`, root `excluded_paths`, root `checks`, and root `[[waivers]]`.
- **Why:** In the new direction, one `guardrail3-rs.toml` configures one workspace. Carrying forward repo-global `[apps.*]` / `[packages]` subtrees would preserve complexity that no longer matches the ownership model.
- **Alternatives considered:**
  - Keep old app/package subtrees — rejected because they belong to the old repo-global config shape.
  - Keep `workspace_root` — rejected because the file itself now lives at the workspace root and no longer needs to point at one.

### Replace Generic Escape Hatches With Generic Waivers
- **Chose:** Put `[[waivers]]` into the proposed schema and parser model instead of `[[escape_hatches]]`.
- **Why:** The old `family/file/kind/selector/reason` shape was too loose and forced several unrelated waiver systems through one generic tuple. The new envelope is narrower and aligns with the agreed rule-driven selector design.
- **Alternatives considered:**
  - Keep `escape_hatches` unchanged — rejected because it preserves an already-messy abstraction.
  - Remove waivers entirely from the parser — rejected for now because current Rust families do already rely on documented exception inventory, and the agreed schema still needs to carry that concept.

### Test With A Realistic On-Disk Fixture
- **Chose:** Add a parser fixture file under `crates/parser/runtime/src/parser_tests/fixtures/workspace_service.toml` and parse it via `from_path`.
- **Why:** Inline string fixtures prove parser behavior, but the user explicitly asked whether the parser had been tested against an actual guardrail config file. The on-disk fixture closes that gap without needing a real migrated workspace in the repo yet.
- **Alternatives considered:**
  - Rely only on inline test strings — rejected because it does not answer the “actual file” concern cleanly.

## Architectural Notes
The package now follows the same layers as the other parser packages:

- root facade crate: public dependency surface
- `crates/parser/runtime`: parse entrypoints, file IO boundary, sidecar parser tests
- `crates/parser/assertions`: reusable parser-test assertions
- `crates/parser/types`: shared typed config model marked `shared = true`

The parser model is intentionally narrow:

- known root workspace fields are typed
- unknown top-level keys are preserved in `extra`
- unknown `[checks]` keys are preserved in `checks.extra`
- unknown waiver keys are preserved in `waiver.extra`

That preserves forward compatibility while still making the currently agreed schema concrete.

The waiver design documented in the plan remains a runtime/orchestrator concern, not a parser concern. The parser only reads `[[waivers]]` data; rule-owned selector computation and waiver indexing happen later in app/runtime code.

## Information Sources
- `packages/rustfmt-toml-parser/` — specimen parser package workspace shape
- `packages/cargo-config-toml-parser/` — specimen parser/runtime/assertions/types API and test pattern
- `.plans/2026-04-04-142741-new-parsers.md` — evolving config/parser direction during this session
- `apps/guardrail3/crates/domain/config/types.rs` — previous shared config model and field inventory
- `apps/guardrail3/crates/app/rs/families/cargo/...` — current waiver/allowance usage
- `apps/guardrail3/crates/app/rs/families/garde/...` — current `sqlx_query_as` waiver usage
- `apps/guardrail3/crates/app/rs/families/hexarch/...` — current `patch_replace` waiver usage
- `apps/guardrail3/crates/app/rs/families/fmt/...` — current rustfmt ignore waiver usage

## Open Questions / Future Considerations
- Integrate the new parser into app/runtime code once `guardrail3-rs.toml` replaces the legacy shared config path.
- Decide whether root `allowed_deps` remains enough, or whether future workspace-local schema needs narrower override points.
- Decide whether all current Rust families should support waivers, or only the small subset with clean exact selectors.
- Replace legacy `escape_hatches` consumption with the new `waivers` model when runtime migration begins.

## Key Files for Context
- `packages/guardrail3-rs-toml-parser/Cargo.toml` — root facade/workspace definition for the new parser package
- `packages/guardrail3-rs-toml-parser/crates/parser/types/src/guardrail3_rs_toml.rs` — typed model for the new workspace-local config file
- `packages/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser.rs` — public parse/from_path entrypoints
- `packages/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — parser behavior coverage
- `packages/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser_tests/fixtures/workspace_service.toml` — realistic on-disk config fixture
- `.plans/2026-04-04-142741-new-parsers.md` — current research draft for workspace-local schema and waiver design
- `packages/cargo-config-toml-parser/` — closest parser package specimen used during implementation

## Next Steps / Continuation Plan
1. Wire `guardrail3-rs-toml-parser` into the Rust runtime/config-loading path after the workspace-local config contract is frozen.
2. Add a runtime-side waiver index keyed by `(rule, file, selector)` and migrate one family as the first proof of the agreed rule-owned selector design.
3. Decide which current Rust families remain waivable under the new model and remove generic legacy `escape_hatches` plumbing as those families migrate.
