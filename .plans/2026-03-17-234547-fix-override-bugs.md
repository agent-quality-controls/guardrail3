# Fix override injection bugs in generate.rs

**Date:** 2026-03-17 23:45
**Task:** Fix bugs 4, 5, and 14 in override injection

## Goal
1. Bug 4: Deduplicate override entries that already exist in base content
2. Bug 5: Validate override content — skip malformed lines with a warning
3. Bug 14: Don't inject "Local overrides" header when override is only comments

## Approach

### Step-by-step plan
1. Add `validate_override_content(content, file_name) -> String` in generate.rs — strips invalid lines
2. Add `deduplicated_override(base, override_content) -> String` in generate.rs — strips lines already in base
3. Apply validation in `load_local_overrides` for each override file
4. Apply dedup in clippy.rs `build_clippy_toml` and deny.rs `build_deny_toml_with_entries` at injection points
5. Fix "Local overrides" header: check if content is empty after validation/dedup before emitting header

### Key decisions
- Validation happens at load time (generate.rs) since it only needs the content and filename
- Dedup happens at injection time (clippy.rs/deny.rs) since it needs the base content
- Both functions are `pub(crate)` so they can be called from modules

## Files to Modify
- `src/commands/generate.rs` — add helper functions, apply validation in load_local_overrides
- `src/domain/modules/clippy.rs` — apply dedup at injection points, conditional header
- `src/domain/modules/deny.rs` — apply dedup at injection points, conditional header
