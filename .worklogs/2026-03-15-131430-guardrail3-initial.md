# guardrail3 — Initial implementation

**Date:** 2026-03-15 13:14
**Scope:** Entire codebase (40 source files, 8329 lines)

## Summary
Built guardrail3 from scratch — a unified Rust CLI tool that validates and generates code guardrails for Rust and TypeScript projects. Went through 4 rounds of adversarial audits with fixes between each round.

## Context & Problem
The ts-rust-railway template has 150+ guardrail rules scattered across clippy.toml, deny.toml, Cargo.toml lints, eslint.config.mjs, .npmrc, pre-commit hooks, etc. These files are manually copied to derived projects and drift out of sync. Need a single tool that can: (1) validate any project against the canonical rule set, (2) generate config files from composable modules, (3) work without config for validation (config-free auto-detection).

## Decisions Made

### Single binary vs multiple packages
- **Chose:** Single Rust binary with `rs`/`ts`/`hooks` subcommand namespaces
- **Why:** One install, one config file, one validate command. User manages one tool, not three.
- **Alternatives considered:**
  - Separate crates.io + npm packages — rejected: requires Node.js for TS side, three tools to manage
  - Git subtree with shell generator — rejected: merge conflicts, no versioning

### Validate is config-free
- **Chose:** `validate` auto-detects stacks and checks against universal rule set, no guardrail3.toml needed
- **Why:** "Point at any project and get a report" — zero setup friction
- **Alternatives considered:**
  - Require config for all commands — rejected: validation should work on any project

### Score removed, counts only
- **Chose:** Report shows error/warning/info counts, no score percentage
- **Why:** Score formula was misleading (info items inflated it). Simple counts are clearer.

### unused_crate_dependencies universally exempted
- **Chose:** Always Info, matching pre-commit hook behavior
- **Why:** False positives in bin crates, integration tests, lib crates with proc macros

### missing_docs not enforced
- **Chose:** Not in expected lint set
- **Why:** User preference — documentation level is a style choice, not a safety guardrail

## Architectural Notes
- Modules are embedded via const strings (not include_dir!) for simplicity
- Config parsing uses toml crate with serde Deserialize
- Source scanning uses line-by-line iteration with block comment tracking
- ESLint config auditing uses pattern grep (not JS parsing)
- Report system is format-agnostic: text/json/md formatters consume the same Report struct

## Information Sources
- Template guardrails: /Users/tartakovsky/Projects/ts-rust-railway (clippy.toml, deny.toml, Cargo.toml, eslint.config.mjs, .npmrc, .githooks/pre-commit)
- Plan: /Users/tartakovsky/Projects/ts-rust-railway/.plans/2026-03-15-111415-guardrail3-unified.md
- 4 rounds of adversarial audits identified and fixed: ID mapping errors, dead code, false positives, score formula, block comment handling, skip entry parsing, multi-line allow handling

## Key Files for Context
- `src/cli.rs` — all CLI command definitions
- `src/commands/validate.rs` — top-level validate orchestrator with git scope resolution
- `src/rs/validate/` — 57 Rust checks across 7 modules
- `src/ts/validate/` — 61 TypeScript checks across 3 modules
- `src/hooks/validate.rs` — 11 hook + 5 deployment checks
- `src/modules/` — embedded module content from template
- `src/commands/generate.rs` — config file generation from modules
- `src/report/` — text/json/markdown formatters

## Next Steps / Continuation Plan
1. Run adversarial audit against the full plan to find any remaining gaps
2. Fix compiler warnings (unused config struct fields)
3. Test generate/init/check/diff on real projects
4. Consider publishing to crates.io
