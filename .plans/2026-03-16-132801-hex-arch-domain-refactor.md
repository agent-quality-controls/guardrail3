# Hex Arch Refactor: Move domain logic into src/domain/

**Date:** 2026-03-16 13:28
**Task:** Move all pure domain logic (validation, modules, config, report types) into a domain/ directory

## Goal
Create `src/domain/` containing all pure business logic, separated from I/O adapters (fs.rs, report formatters, commands).

## Approach

### Step-by-step plan
1. Create directory structure: `src/domain/`, `src/domain/validate/`
2. Move directories: rs → domain/validate/rs, ts → domain/validate/ts, hooks → domain/validate/hooks, modules → domain/modules, config → domain/config
3. Move discover.rs → domain/discover.rs
4. Create domain/report.rs with types from report/types.rs, update report/types.rs to re-export
5. Create domain/mod.rs and domain/validate/mod.rs
6. Update lib.rs
7. Update ALL imports across ~50 files
8. Run cargo test and cargo clippy

### Key decisions
- **Report types split:** Move types to domain/report.rs, leave formatters in report/
- **Re-export strategy:** Update lib.rs to expose `pub mod domain` and remove direct `pub mod rs/ts/hooks/modules/config/discover`
- **Import update approach:** Systematic find-and-replace with manual verification

## Files to Modify
- `src/lib.rs` — replace module declarations
- `src/main.rs` — update all guardrail3:: imports
- `src/domain/mod.rs` — new file
- `src/domain/validate/mod.rs` — new file
- `src/domain/report.rs` — new file (types moved from report/types.rs)
- `src/report/mod.rs` — remove types, re-export from domain
- `src/report/types.rs` — gutted, re-exports from domain::report
- All files in rs/, ts/, hooks/, commands/, config/ — import path updates
- tests/*.rs — import path updates
- fuzz/*.rs — import path updates
