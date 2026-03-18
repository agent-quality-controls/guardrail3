# Per-language check category flags in guardrail3.toml

**Date:** 2026-03-17 14:07
**Task:** Add config-driven check categories so situational checks (architecture, garde, tests, release) can be toggled per language

## Goal
Users can enable/disable check categories per language in `guardrail3.toml`. Core checks always run. Situational checks only run when explicitly enabled. CLI flags still work as override/filter on top of config.

## Input Information
- Current `ValidateDomains` struct has 4 bool fields: `code`, `architecture`, `release`, `tests` — all CLI-driven
- Garde checks (R-GARDE-01/02/03/04/05, R34/R35) currently always run under `domains.architecture` and source_scan respectively
- Architecture checks (R-ARCH-01/02/03/04, T-ARCH-01/02, eslint boundary audit) run under `domains.architecture`
- Test checks (R-TEST-*, T-TEST-*) run under `domains.tests`
- Release checks (R-REL-*, R-PUB-*, R-BIN-*) run under `domains.release`
- R34/R35 (garde skip in source scan) currently run in `allow_checks.rs` called from `source_scan.rs` — needs to be gated on garde category

## Categories

| Category | RS checks | TS checks | Default |
|---|---|---|---|
| **core** | Everything not listed below | Everything not listed below | Always on |
| **architecture** | R-ARCH-01/02/03/04, R-DEPS-01/02 | T-ARCH-01/02, eslint boundary audit (T36-T39) | Off |
| **garde** | R-GARDE-01/02/03/04/05, R34, R35 | N/A | Off |
| **tests** | R-TEST-02/03/04/05/06/07/08/09 | T-TEST-01/02/03/04/05 | On |
| **release** | R-REL-01/02/03, R-PUB-02/04/05/06/07/08/09/10/11/12, R-BIN-01/02/03 | N/A | Off |

## Approach

### Step-by-step plan

1. **Add `ChecksConfig` types to `domain/config/types.rs`**
   - `RustChecksConfig { architecture: Option<bool>, garde: Option<bool>, tests: Option<bool>, release: Option<bool> }`
   - `TsChecksConfig { architecture: Option<bool>, tests: Option<bool> }`
   - Wire into existing `RustConfig.checks` and `TypeScriptConfig.checks`

2. **Add `CheckCategories` domain type to `domain/report.rs`**
   - `RustCheckCategories { architecture: bool, garde: bool, tests: bool, release: bool }`
   - `TsCheckCategories { architecture: bool, tests: bool }`
   - These are the resolved runtime values (config + CLI merge)

3. **Update RS `validate/mod.rs`** to accept `RustCheckCategories` instead of `ValidateDomains`
   - `run_architecture_checks` gated on `categories.architecture`
   - Garde section gated on `categories.garde`
   - Test section gated on `categories.tests`
   - Release section gated on `categories.release`
   - R34/R35 in source_scan gated on `categories.garde`

4. **Update TS `validate/mod.rs`** to accept `TsCheckCategories` instead of `ValidateDomains`
   - Architecture section gated on `categories.architecture`
   - Test section gated on `categories.tests`

5. **Update `main.rs` / `commands/validate.rs`** to merge config + CLI
   - Load `guardrail3.toml` → extract check categories with defaults
   - CLI `--architecture`, `--tests`, `--release` become filters (if any specified, only those run)
   - Add `--garde` CLI flag
   - Resolution: config sets the baseline, CLI narrows it

6. **Update `rs init`** to scaffold `[rust.checks]` section with comments
7. **Update `ts init`** to scaffold `[typescript.checks]` section
8. **Update `guardrail3.toml`** (self-dogfooding) to declare its own check categories
9. **Update help_gen.rs** to document check categories
10. **Update tests** — existing tests that use `ValidateDomains` need updating

### Key decisions

- **Config sets baseline, CLI filters**: If config says `architecture = true` and CLI says `--tests`, only tests run. If no CLI domain flags, config decides.
- **`ValidateDomains` stays for hooks**: Hooks don't have per-language categories, they keep using the existing domain system.
- **R34/R35 move under garde gate**: Currently in source_scan's allow_checks — need to make source_scan aware of garde category.
- **Defaults**: `tests = true`, everything else `false`. This means on first run without config, you get core + tests.

## Files to Modify

- `apps/guardrail3/src/domain/config/types.rs` — add `RustChecksConfig`, `TsChecksConfig`
- `apps/guardrail3/src/domain/report.rs` — add `RustCheckCategories`, `TsCheckCategories`
- `apps/guardrail3/src/app/rs/validate/mod.rs` — gate sections on categories
- `apps/guardrail3/src/app/rs/validate/source_scan.rs` — pass garde flag, gate R34/R35
- `apps/guardrail3/src/app/rs/validate/allow_checks.rs` — no change (called conditionally from source_scan)
- `apps/guardrail3/src/app/ts/validate/mod.rs` — gate sections on categories
- `apps/guardrail3/src/cli.rs` — add `--garde` flag to `ValidateArgs`
- `apps/guardrail3/src/main.rs` — merge config + CLI into categories
- `apps/guardrail3/src/commands/validate.rs` — merge config + CLI
- `apps/guardrail3/src/commands/init.rs` — scaffold check categories in init
- `apps/guardrail3/src/help_gen.rs` — document categories
- `guardrail3.toml` — add `[rust.checks]` for self-validation
