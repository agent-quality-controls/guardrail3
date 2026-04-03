# Rule message clarity — HEXARCH family

Audited all 25 RS-HEXARCH rules.

## Changes
- **10**: Added fix action (remove member or move crate inside boundary)
- **11**: Added fix action (remove member from root workspace)
- **13**: Added fix action + direction explanation to dependency direction violation
- **16**: Fixed severity bug — documented escape hatch was Error, now Warn (matches pattern in CARGO-03, FMT-07, GARDE-09). Added fix actions to missing/weak reason.
- **17**: Added fix action + direction explanation (workspace inherited)
- **18**: Added fix action + direction explanation (renamed deps)
- **19**: Added fix action (break cycle by extracting shared code)
- **20**: Added fix action (restructure test dependencies)
- **21**: Added fix actions to domain purity violations (non-pure layer + disallowed external)
- **25**: Added fix action + direction explanation (target deps)
- **27**: Added fix action (remove nested workspace)

## Unchanged (already good)
01, 02, 03, 04, 05, 06, 08, 12, 14, 15, 22, 23, 24, 26

## Bug fixed
- **16**: Documented escape-hatch patch/replace was Error instead of Warn — escape-hatch mechanism was useless (all 3 paths were Error)

## Key files
- All rule.rs under hexarch/crates/runtime/src/
