# Per-language check category flags

**Date:** 2026-03-17 14:17
**Scope:** domain/config/types.rs, domain/report.rs, rs/validate/mod.rs, rs/validate/source_scan.rs, ts/validate/mod.rs, cli.rs, main.rs, commands/validate.rs, commands/init.rs, help_gen.rs, guardrail3.toml, tests/adversarial_fixtures.rs

## Summary
Added config-driven check categories so situational checks (architecture, garde, tests, release) can be toggled per language in guardrail3.toml. Core checks always run. CLI flags act as filters on top of config.

## Context & Problem
guardrail3 had checks that assume project properties that may not hold — hex arch enforcement on projects without hex arch, garde validation on projects without garde, release checks on unpublished crates. These fired errors on projects where they don't apply, creating noise. The existing `ValidateDomains` was CLI-only with no config persistence.

## Decisions Made

### 5 categories, not 12
- **Chose:** core (always on), architecture, garde, tests, release
- **Why:** The user correctly identified that splitting config/suppression-hygiene/source-health/hooks/deploy into separate categories makes zero sense — they're all "is your project properly configured" which is guardrail3's core job. Only the truly situational things need toggles.
- **Alternatives considered:**
  - 12 fine-grained categories — rejected because most can't meaningfully be used independently
  - Global `[checks]` with inheritance — rejected for unnecessary complexity

### Per-language, not global
- **Chose:** `[rust.checks]` and `[typescript.checks]` separately
- **Why:** garde and release are Rust-only. Architecture could differ between RS and TS sides of a polyglot project.

### Config baseline + CLI filter
- **Chose:** Config sets the baseline (what categories are enabled). CLI flags act as a filter (if any domain flag is set, only those categories run).
- **Why:** Config is the persistent "what does this project need". CLI is the ad-hoc "I only want to see X right now".

### Defaults: tests=true, everything else=false
- **Chose:** Only tests enabled by default
- **Why:** Tests are universally applicable. Architecture, garde, and release all require project-specific setup to be meaningful.

### R34/R35 moved under garde gate
- **Chose:** Garde skip checks in source_scan now gated on `garde_enabled` parameter
- **Why:** These checks are about garde field validation — meaningless if garde isn't in use

## Key Files for Context
- `apps/guardrail3/src/domain/config/types.rs` — RustChecksConfig, TsChecksConfig
- `apps/guardrail3/src/domain/report.rs` — RustCheckCategories, TsCheckCategories (resolved runtime types)
- `apps/guardrail3/src/app/rs/validate/mod.rs` — RS validation gating
- `apps/guardrail3/src/app/ts/validate/mod.rs` — TS validation gating
- `apps/guardrail3/src/main.rs` — config+CLI merge logic (build_rs_categories, build_ts_categories)
- `guardrail3.toml` — self-dogfooding with all categories enabled
