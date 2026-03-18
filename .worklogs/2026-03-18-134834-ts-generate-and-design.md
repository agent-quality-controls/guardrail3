# TS generate complete configs + per-app config design

**Date:** 2026-03-18 13:48
**Scope:** apps/guardrail3/src/ (generate, modules, validate), tests/, .plans/

## Summary

Two major efforts: (1) made `ts generate` produce complete config files with all plugins, and (2) deep design work on per-app config architecture.

## Context & Problem

`ts generate` only produced 3-4 files (.npmrc, tsconfig.base.json, .jscpd.json, optional eslint starter). It needed to produce complete ESLint config with all 260+ rules (unicorn, regexp, sonarjs, react, a11y, boundaries, etc.), plus cspell.json and stylelint config.

Running the tool on steady-parent revealed fundamental architecture gaps: generate overwrites entire files, destroying project-specific content (487-line eslint, custom stylelint, 283-line pre-commit hook). The tool has no concept of per-app configs, per-app overrides, or non-destructive editing.

## Decisions Made

### TS Generate
- **Chose:** Generate complete ESLint config with ALL plugins and rules
- **Why:** Validate checks for 260+ rules; generate should produce configs that pass validation
- **Alternatives:** Keep minimal starter (rejected — useless)

### Bug Fixes (from adversarial QA)
- **AV-2.1:** `ends_with` → `split('/').last()` for path resolution (was matching substrings)
- **AV-8.4:** `ts init --force` subsection skip (was corrupting config when typescript had subsections)
- **AV-5.3:** Override `[[` injection restricted to feature-bans only (was allowing section header injection)
- **AV-6.2:** devDependencies checked for content signals (velite in devDeps wasn't detected)
- **AV-9.1:** ESLint ignores use `**/` prefix for monorepo compatibility
- **T2:** max-lines 300→400

### RS Generate Path Resolution
- **Chose:** Discovery-based resolution — generate runs detect_project() and maps config app names to actual filesystem paths
- **Why:** Init strips `apps/` from directory names for clean config keys; generate needs to reverse this
- **Alternatives:** Store full paths in config (ugly TOML keys), manual path field per app

### Per-App Config Design
- **Chose:** 6 categories: Fully-owned, Merge-managed, Shadow-imported, Scaffold-once, Validate-only, Not-managed
- **Why:** Different files need fundamentally different treatment based on guardrail vs project content ratio
- Comprehensive per-file analysis in .plans/per-app-config-design/

### clippy.toml Plan
- **Chose:** One clippy.toml per workspace root, merge-managed via toml_edit
- **Why:** Per-crate clippy.toml shadows workspace one completely; Cargo.toml deps handle crate isolation
- Override mechanism: only removals needed (user's extra bans preserved naturally by merge)

## Architectural Notes

The fundamental shift: from "generate whole files" to "ensure specific guardrail entries exist while preserving everything else." This requires format-aware parsers:
- TOML: toml_edit (not yet in deps)
- JSON: serde_json merge
- JS: shadow-import pattern (generate engine file, user imports it)
- INI: line-based merge

## Information Sources

- steady-parent actual file contents (clippy.toml, deny.toml, eslint.config.mjs, etc.)
- clippy configuration docs: https://doc.rust-lang.org/clippy/configuration.html
- ESLint flat config docs: https://eslint.org/docs/latest/use/configure/configuration-files
- release-plz config docs: https://release-plz.dev/docs/config
- Adversarial QA report (50+ attack vectors across 9 categories)

## Key Files for Context

- `.plans/per-app-config-design/00-unified-design-v2.md` — master design doc (corrected multiple times)
- `.plans/by_file/rs/clippy-toml.md` — detailed clippy.toml per-file plan (adversarially reviewed)
- `.plans/per-app-config-design/01-06` — 6 deep design docs (Rust scoping, TS scoping, overrides, generate flow, gap analysis, non-destructive editing)
- `apps/guardrail3/src/domain/modules/eslint.rs` — new ESLint config builder (469 lines)
- `apps/guardrail3/src/domain/modules/stylelint.rs` — new stylelint config builder
- `apps/guardrail3/src/domain/modules/cspell.rs` — new cspell config builder
- `apps/guardrail3/src/commands/generate_helpers.rs` — extracted from generate.rs (path resolution, overrides)

## Next Steps / Continuation Plan

1. deny.toml per-file plan (same depth as clippy.toml)
2. Remaining per-file plans (rustfmt, eslint, tsconfig, stylelint, etc.)
3. Implementation: P0 fixes (stop overwriting eslint/stylelint/pre-commit)
4. Implementation: toml_edit merge for clippy.toml and deny.toml
5. Implementation: shadow-import for eslint engine
