# Arch Family Rewrite Plan

**Date:** 2026-04-01 22:39
**Scope:** `.plans/2026-04-01 ISSUES.md`

## Summary

Designed complete arch family rewrite through extensive discussion. Identified all issues with current 4-rule implementation and planned 8 new rules across 3 groups. Current arch is fundamentally broken: path-gated to `packages/`, library-only filters, name-matching bypasses, missing universal enforcement.

## Decisions Made

### Arch operates on crate directory tree, not workspaces or paths
- **Chose:** Arch knows only Cargo.toml tree, lib.rs/mod.rs content, source mod declarations
- **Why:** Current implementation hardcodes `packages/` filter and workspace membership. Architecture rules must be universal.
- **Rejected:** Workspace-aware rules, path-based filtering, profile-name gating

### Facade enforcement at both crate and module level
- **Chose:** lib.rs AND mod.rs are facades. Both must be facade-only (no implementation). Module directories must use mod.rs (not foo.rs convention).
- **Why:** Same encapsulation principle at both levels. lib.rs gates crate access, mod.rs gates module access.
- **Rejected:** Only enforcing at crate level, allowing foo.rs convention

### Boundary crossing via filesystem containment, not name matching
- **Chose:** Walk up from dependency target to find intermediate Cargo.toml boundaries. If source is outside that boundary, error.
- **Why:** Name matching is bypassable via aliases. Path resolution is not.
- **Rejected:** Package name matching (current RS-ARCH-04), workspace membership checks

### Single `shared` flag for dependency access control
- **Chose:** `shared = true` in `[package.metadata.guardrail3]`. Non-child deps require it.
- **Why:** Covers both sibling access and ancestor access with one flag. Default closed prevents spaghetti.
- **Rejected:** Two separate flags (sibling_access + descendant_access), default-open model

### Feature-gated facade exports
- **Chose:** Every pub item in lib.rs behind `#[cfg(feature)]`. `all` meta-feature enables sub-features. `all` cannot directly gate items.
- **Why:** Allows consumers to depend on parts of a crate. Zero overhead for consumers who want everything (default = all).

### Rules renumbered by group
- **Chose:** facade/ (01-04), dependency/ (05-06), complexity/ (07-08). Consecutive within groups.
- **Why:** Previous numbering was non-consecutive across groups, confusing.

### Sibling .rs file threshold raised to 10
- **Chose:** From 6 to 10 for escalation rule
- **Why:** Infrastructure files (lib.rs, mod.rs, facts, inputs) eat into the budget. 6 was too tight.

## Key Files for Context
- `.plans/2026-04-01 ISSUES.md` — complete issue list and target rule set
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/` — current (broken) implementation
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_27_facade_only_lib.rs` — rule to migrate to arch

## Next Steps
1. Implement all 8 rules in new grouped structure (facade/, dependency/, complexity/)
2. Wire into runtime, family mapper, config, CLI
3. Remove `packages/` filter and library-only gates
4. Migrate RS-CODE-27 to arch as RS-ARCH-02
5. Delete old RS-ARCH-03
6. Verify project produces expected errors (400+ for mod rules alone)
7. Run test-attack skill for convergence
