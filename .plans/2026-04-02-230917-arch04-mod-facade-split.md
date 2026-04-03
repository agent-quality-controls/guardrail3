# ARCH-04: Make all mod.rs facade-only

## Current state
2344 ARCH-04 violations across 414 directories.

## Population breakdown

| Population | Dirs | Violations | Description |
|-----------|------|-----------|-------------|
| Unsplit rules | ~100 | ~646 | Rule mod.rs with production + test code, no rule.rs yet |
| Split rule facades | ~121 | ~289+357=~350* | Have rule.rs but mod.rs still has #[cfg(test)] helpers |
| Test modules | ~126 | ~721 | tests/mod.rs with helper functions + imports |
| Infrastructure | ~65 | ~845 | discover, project_walker, hooks, facts, parse, etc. |
| Legacy | 2 | ~few | legacy/ directory |

*Split rules: violations are all #[cfg(test)] — private use imports + helper functions.

## Key finding: #[cfg(test)] items

121 already-split rule facades are flagged ONLY because of `#[cfg(test)]` helper functions and imports. These don't exist in production builds. Similarly, 126 test module mod.rs files are flagged for having test orchestration code — which is their entire purpose.

## Decision needed: should #[cfg(test)] items be considered facade violations?

**Option A: Fix the rule** — skip items gated behind `#[cfg(test)]`. This is arguably correct: facade = production API surface. Test helpers aren't part of the production facade. This eliminates ~350 rule facade violations and all 721 test module violations instantly (1071 violations, 247 directories).

**Option B: Move all #[cfg(test)] items** out of mod.rs into separate files. More mechanical work but enforces "mod.rs is ONLY module declarations" even for tests.

## Approach

### Phase 1: Fix the rule to skip #[cfg(test)] items
The facade_surface.rs parser already extracts `#[cfg(...)]` attributes via `extract_feature_gate`. Add `#[cfg(test)]` detection — if an item has `#[cfg(test)]`, exclude it from `body_items`.

This is the right semantic: `body_items` represents "implementation that shouldn't be in a facade." Test-only code isn't production implementation.

### Phase 2: Split remaining unsplit rules (~100 dirs)
Extend the split script to handle rules with multiple pub fns (not just `pub fn check`). Pattern: any mod.rs with `const ID: &str = "RS-"` that doesn't have a rule.rs.

### Phase 3: Split infrastructure mod.rs (~65 dirs)
For each infrastructure mod.rs that has production implementation:
- Move implementation to a descriptive sibling file inside the module dir
- Leave mod.rs as facade: `mod impl_file; pub use impl_file::{...};` + child mod declarations

### Phase 4: Verify
Run arch validate, confirm ARCH-04 count drops to near zero.

## Files to modify

### Phase 1
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts/facade_surface.rs` — skip #[cfg(test)] items

### Phase 2
- ~100 rule mod.rs files → split into mod.rs + rule.rs

### Phase 3
- ~65 infrastructure mod.rs files → split into mod.rs + implementation.rs
