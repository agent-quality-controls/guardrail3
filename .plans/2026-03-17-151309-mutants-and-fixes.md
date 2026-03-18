# Add mutants config, fix remaining warnings

**Date:** 2026-03-17 15:13
**Task:** Add cargo-mutants config, fix R-BIN-03 false positive on publish=false crates, fix R41 main.rs imports, fix R-REL-03 release-plz config, change defaults to all-on

## Files
- `.cargo/mutants.toml` — already created
- `Cargo.toml` — already has [profile.mutants]
- `apps/guardrail3/src/app/rs/validate/release_bin_checks.rs` — fix R-BIN-03 to skip publish=false
- `apps/guardrail3/src/main.rs` — consolidate imports to fix R41
- `release-plz.toml` — add [[package]] for guardrail3
- `apps/guardrail3/src/domain/report.rs` — defaults already changed to all-on
- `apps/guardrail3/src/commands/init.rs` — update scaffold comments for all-on defaults
- `apps/guardrail3/src/help_gen.rs` — update docs for all-on defaults
