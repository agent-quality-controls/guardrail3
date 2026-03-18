# Wire domain filtering into all three orchestrators

**Date:** 2026-03-15 19:46
**Task:** Wire ValidateDomains into rs/ts/hooks validate orchestrators, gate checks by domain

## Goal
Each validation domain (code, architecture, release, tests) can be toggled independently via CLI flags. Checks only run when their domain is active.

## Approach

### Step-by-step plan

1. **src/rs/validate/mod.rs** — Change `run()` signature to accept `domains: &ValidateDomains` and `thorough: bool`. Gate sections by domain. Move architecture checks (dependency_direction, check_unsafe_code_forbid) out of source_scan::check() into mod.rs. Remove dead_code allow on ValidateDomains.

2. **src/rs/validate/source_scan.rs** — Remove the three architecture calls (dependency_direction::check_all_dependency_directions, dependency_direction::check_dependency_graph, structure_checks::check_unsafe_code_forbid) since they'll be called from mod.rs.

3. **src/ts/validate/mod.rs** — Add `domains: &ValidateDomains` param, import from crate::rs::validate, gate checks.

4. **src/hooks/validate.rs** — Add `domains: &ValidateDomains` param, import from crate::rs::validate, gate checks.

5. **src/commands/validate.rs** — Rename `_domains` to `domains`, pass to all three run() calls, pass `args.thorough` to rs.

6. **src/main.rs** — Update the three sub-command call sites to build ValidateDomains and pass it + thorough.

## Files to Modify
- `src/rs/validate/mod.rs`
- `src/rs/validate/source_scan.rs`
- `src/ts/validate/mod.rs`
- `src/hooks/validate.rs`
- `src/commands/validate.rs`
- `src/main.rs`
