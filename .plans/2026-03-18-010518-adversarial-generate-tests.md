# Adversarial test suite for generate/diff/init — based on QA attack vector report

**Date:** 2026-03-18 01:05
**Task:** Build comprehensive adversarial test fixtures for all generate features

## Confirmed Bugs to Fix (from attack vectors)

### Critical
1. **AV-2.1**: `ends_with` suffix matching → `"apps/my-validator".ends_with("validator")` = true. Fix: match on path segment boundary, not substring.
2. **AV-8.4**: `ts init --force` corrupts config when `[typescript]` has subsections like `[typescript.apps.web]`. The skip loop stops at subsection headers.
3. **AV-1.3**: Per-crate config keys (e.g., `shedul3r-core`) don't resolve to their actual nested path.

### High
4. **AV-6.2**: `auto_detect_app_type` only checks `dependencies`, not `devDependencies`. Velite in devDeps not detected.
5. **AV-8.5**: `rs generate --dry-run` uses `generate_expected` which includes TS files. Should only show RS files.
6. **AV-9.1**: ESLint ignore patterns use `"node_modules/**"` instead of `"**/node_modules/**"` — misses app-level dirs in monorepos.
7. **AV-3.5**: cspell.json `words` array lost on regenerate. Need override mechanism or preserve strategy.
8. **AV-5.3**: `[[bans.features]]` header passes validate_override_content in deny-bans context.

## Test Categories

### T1: Workspace Discovery (adversarial_ws_discovery.rs)
- Empty workspace members array
- Glob members matching nothing (crates/* with empty dir)
- Nested workspace inside app (apps/X has own [workspace])
- Workspace exclude matching a configured app
- Multiple workspaces at different levels
- Single-crate project (no workspace)
- Virtual workspace (workspace-only, no package)

### T2: Path Resolution (adversarial_path_resolution.rs)
- App name is suffix of another app dir (validator vs my-validator) — MUST FAIL before fix
- App name with dots (quoted TOML key)
- App at project root (name ".")
- Two apps, same suffix different prefix
- App not found in any workspace → fallback behavior
- Deeply nested crate (apps/X/crates/Y/crates/Z)
- App in packages/ not apps/

### T3: TS Generate Complete Config (adversarial_ts_generate.rs)
- Content + service apps → both plugin sections present
- Content only → no boundaries, yes a11y/tailwind-ban
- Service only → no a11y, yes boundaries
- Library only → defaults to service ESLint
- No apps configured → defaults to service
- ESLint ignores use `**/` prefix
- Generated config is syntactically valid JS (basic parse check)
- cspell.json is valid JSON
- stylelint only generated for content apps
- tsconfig has all strict flags

### T4: RS Generate Per-App (adversarial_rs_generate.rs)
- Single app, single workspace → configs at correct path
- Multiple apps, each own workspace → per-app configs
- App + packages → app configs at app path, package configs at root
- No apps, only packages → configs at root with library profile
- App with type override (service app with library type) → library clippy
- Unknown type string → falls back to service with no crash

### T5: Custom Entry Detection (adversarial_diff_detection.rs)
- Entry with } in quoted string
- Mixed CRLF/LF line endings
- Empty file on disk
- File with only comments
- Section header in a quoted value
- Multiline entries spanning 3+ lines

### T6: Init Round-Trip (adversarial_init_roundtrip.rs)
- ts init → ts generate → ts validate (zero-error round-trip)
- rs init → rs generate → rs validate (no config-file errors)
- ts init --force with existing subsections — MUST NOT corrupt
- Generate idempotency (run twice, second shows no changes)
- rs generate --dry-run only shows RS files (not TS)

### T7: Override Edge Cases (adversarial_override_advanced.rs)
- Section header injection via deny-bans override
- Override for unrecognized filename → should warn
- Override with entries that duplicate generated base

## Approach

Each test file creates a temp directory with the fixture, runs the generate/diff/init function, and asserts the output. Tests use `tempdir` for isolation.

The test fixtures need:
- Minimal Cargo.toml files (with [workspace] sections)
- Minimal package.json files (with dependencies)
- guardrail3.toml configs with various app/package combos
- Some pre-existing config files (clippy.toml, eslint.config.mjs) with custom content

## Files to Create
- `tests/adversarial_ws_discovery.rs` — 7+ tests
- `tests/adversarial_path_resolution.rs` — 7+ tests
- `tests/adversarial_ts_generate.rs` — 10+ tests
- `tests/adversarial_rs_generate.rs` — 6+ tests
- `tests/adversarial_diff_detection.rs` — 6+ tests
- `tests/adversarial_init_roundtrip.rs` — 5+ tests
- `tests/adversarial_override_advanced.rs` — 3+ tests

Total: 44+ adversarial tests across 7 files
