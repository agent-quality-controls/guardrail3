# deny-toml-parser Full Schema Handoff

**Date:** 2026-04-05 12:59
**Scope:** `packages/deny-toml-parser/`

## Summary
Expanded `deny-toml-parser` from a useful subset parser into a current-spec parser for the documented cargo-deny config surface. The package now models the broader `graph`, `advisories`, `bans`, `licenses`, `sources`, and `output` shapes, plus the auxiliary top-level `exceptions = [...]` file form used by `deny.exceptions.toml`.

## Context & Problem
The deny parser had already been normalized into the new parser-package structure, but it was still only modeling a subset of cargo-deny’s actual config schema. The immediate trigger was feedback on missed `licenses.exceptions` and `bans` fields, but a current-doc audit showed the issue was wider:

- several standard cargo-deny fields were only landing in `extra`
- some newer package-spec-based shapes were missing
- the parser still had helper-method API in `types`, which is not aligned with the current “file-faithful primary representation only” direction

The user explicitly clarified that the parser should not be consumer-shaped. It should be a robust representation of the file itself.

## Decisions Made

### Model The Current Documented cargo-deny Surface
- **Chose:** Add typed support for the currently documented config fields instead of leaving them in `extra`.
- **Why:** The parser’s job is to represent the file honestly and usefully, not just cover the narrow fields currently used by downstream checks.
- **Alternatives considered:**
  - Leave unknown standard fields in `extra` — rejected because that turns current schema into “future unknowns,” which is inaccurate.
  - Model only the fields already used by guardrail3 — rejected because that makes the parser downstream-shaped.

### Keep File Shape Primary
- **Chose:** Remove the remaining helper-method layer in deny parser `types` and expose the parsed fields directly.
- **Why:** The user explicitly rejected secondary/convenience API. The parser package should expose the file model, not a normalized consumer view.
- **Alternatives considered:**
  - Keep getters for convenience — rejected because they make the public API more consumer-shaped than file-shaped.

### Preserve Deprecated Forms When They Are Still Part Of Real Config Input
- **Chose:** Continue parsing old `name` / `version` table-style package spec fields alongside the newer `crate = "package-spec"` form.
- **Why:** cargo-deny docs still describe the old table form as deprecated rather than impossible, and a robust parser should preserve that input shape.
- **Alternatives considered:**
  - Parse only the modern `crate` form — rejected because it would reject or partially-model still-valid config.

### Model The Auxiliary Exceptions File In The Root Type
- **Chose:** Add root-level `exceptions: Vec<LicenseException>` on `DenyToml`.
- **Why:** cargo-deny also supports standalone exceptions files (`deny.exceptions.toml`, `.deny.exceptions.toml`, `.cargo/deny.exceptions.toml`). Leaving top-level `exceptions` untyped would miss a real standard file shape.
- **Alternatives considered:**
  - Keep root-level `exceptions` in `extra` — rejected because that would hide a documented file mode behind unknown-key preservation.

## Architectural Notes
The deny parser now follows the same parser-package contract as the other normalized parsers:

- root facade crate
- `crates/parser/runtime`
- `crates/parser/assertions`
- `crates/parser/types`

Within `types`, the updated modeling direction is:

- public fields for the actual file shape
- `#[serde(flatten)]` `extra` for unknown keys
- no helper/getter layer
- enums only when the file genuinely supports multiple representations, such as bare strings vs detailed tables

Notable schema expansions in this pass:

- `graph`
  - `features`
  - `exclude-dev`
  - `exclude-unpublished`
  - typed `targets` entries (`String` or `{ triple, features, ... }`)
- `advisories`
  - `db-path`
  - `db-urls`
  - `unused-ignored-advisory`
  - deprecated `version`
  - richer `ignore` detail shape for package-spec-based entries
- `bans`
  - `multiple-versions-include-dev`
  - `workspace-default-features`
  - `external-default-features`
  - `allow-workspace`
  - `skip-tree`
  - `[bans.workspace-dependencies]`
  - `[bans.build]`
  - `[[bans.build.bypass]]`
  - additional detailed-entry fields like `reason`, `deny-multiple-versions`, `use-instead`, `version`
- `licenses`
  - `include-dev`
  - `include-build`
  - deprecated `version`
  - `unused-allowed-license`
  - `unused-license-exception`
  - `clarify`
  - `ignore-sources`
  - root-level `exceptions`
- `sources`
  - `required-git-spec`
  - `private`
  - `unused-allowed-source`
  - `[sources.allow-org]`

## Information Sources
- Official cargo-deny docs:
  - `https://embarkstudios.github.io/cargo-deny/checks/cfg.html`
  - `https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html`
  - `https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html`
  - `https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html`
  - `https://embarkstudios.github.io/cargo-deny/checks/sources/cfg.html`
- Existing package files in `packages/deny-toml-parser/`
- Prior parser normalization worklogs in `.worklogs/`

## Open Questions / Future Considerations
- The parser now models the documented cargo-deny config surface, but cargo-deny may evolve again. Future audits should compare against the current docs rather than assuming today’s structs stay complete.
- `hash` in `licenses.clarify.license-files` is represented as `toml::Value` so the raw TOML numeric shape is preserved without overcommitting to one integer encoding path.
- Deprecated fields like section-local `version` and older `name` / `version` table forms are intentionally preserved in the parser because they are still part of accepted file input.

## Key Files for Context
- `packages/deny-toml-parser/crates/parser/types/src/deny_toml.rs` — root parsed type, including root-level `exceptions`
- `packages/deny-toml-parser/crates/parser/types/src/graph.rs` — graph config and typed target entries
- `packages/deny-toml-parser/crates/parser/types/src/advisories.rs` — advisories config and richer ignore entries
- `packages/deny-toml-parser/crates/parser/types/src/bans.rs` — expanded bans/build/workspace-dependencies schema
- `packages/deny-toml-parser/crates/parser/types/src/licenses.rs` — exceptions, clarify, private, and license-file modeling
- `packages/deny-toml-parser/crates/parser/types/src/sources.rs` — sources config, private sources, allow-org, unused handling
- `packages/deny-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — coverage for the expanded schema surface
- `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md` — this handoff

## Next Steps / Continuation Plan
1. Audit the remaining parser packages against their current upstream specs using primary docs only.
2. Patch any parser package that is still modeling a subset while treating the file shape, not current callers, as the source of truth.
3. Write a separate broader handoff that records which parser packages were fully covered, which were expanded, and which still have intentionally local/non-upstream schema (notably `guardrail3-rs.toml`).
