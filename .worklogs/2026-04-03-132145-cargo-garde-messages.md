# Rule message clarity — CARGO + GARDE families

## RS-CARGO (15 rules)
- **01**: Added fix actions to all variants; fixed redundant "invalid shape" message
- **02**: Added fix actions to "weakens policy" and "wrong priority"
- **03**: Added fix actions to "missing reason" and "weak reason"
- **04**: Unchanged (already good)
- **05**: Replaced jargon "owning Cargo package metadata"; added fix to "edition missing"
- **06**: Added fix action to "weakened override"
- **07**: Included lint name in description; added fix action
- **08**: Added "Prefer resolver 3 (edition 2024) if the project allows it" to all error variants
- **09**: Added fix actions to "unrecognized" and "older than workspace"
- **10**: Added fix action (remove from members or create Cargo.toml)
- **11**: Unchanged (already good)
- **12**: Replaced dead-end "documented but still forbidden" with actionable "Remove or get it approved"
- **13**: Same fix as 12 for member-local allows; added escape-hatch guidance
- **14**: Replaced jargon title "cargo-family input failure" → "failed to read Cargo configuration"
- **15**: Replaced "MSRV contract" jargon; replaced "inventoried for non-library profiles" jargon

## RS-GARDE (14 rules)
- **01**: Replaced jargon; added fix action (add garde to dependencies)
- **02**: Replaced "No covering clippy configuration" jargon; added fix action
- **03**: Same pattern as 02 (disallowed-types)
- **04**: Same pattern as 02 (reqwest ban)
- **05**: Replaced jargon; added fix action (derive Validate); noted it's garde's Validate
- **06**: Same pattern as 02 (additional method bans)
- **07**: Added fix action (derive or impl Validate)
- **08**: Same as 05 but for enums
- **09**: Added fix actions to "missing reason" and "weak reason"
- **10**: Unchanged (already good — pass-through)
- **11**: Unchanged (already good — has examples)
- **12**: Unchanged (already good — has fix action)
- **13**: Added fix action (add garde context attribute)
- **14**: Replaced "prove"/"site" jargon; added fix action (call .validate())

## Key files
- All rule.rs under cargo/crates/runtime/src/ and garde/crates/runtime/src/
