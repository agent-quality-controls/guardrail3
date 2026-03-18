# Expand garde enforcement to all input boundary derives

**Date:** 2026-03-16 17:20
**Task:** Change R-GARDE-01 to Error, expand R-GARDE-05 to check Deserialize/Parser/Args/FromRow, update messages and help, add tests.

## Goal
Every struct that accepts external input (via serde, clap, or sqlx) must also derive `Validate`. R-GARDE-01 missing garde = Error. R-GARDE-05 checks four derives.

## Approach

### Files to modify
1. `src/app/rs/validate/garde_checks.rs` — R-GARDE-01 severity, rename/expand count function, update R-GARDE-05 messages
2. `src/help_gen.rs` — update R-GARDE-05 description
3. `src/domain/modules/guide.rs` — update GARDE section

### Key decisions
- Rename `count_deserialize_structs_ast` to `count_unvalidated_input_structs` for clarity
- Check for: Deserialize, Parser, Args, FromRow (and path-qualified variants)
- R-GARDE-05 severity stays as-is (Info for inventory) but message updated
- R-GARDE-01 missing garde changes from Info to Error
