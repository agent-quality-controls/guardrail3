# Cargo.toml (workspace.lints + per-crate lints)

## Category: Validate-only

guardrail3 NEVER modifies Cargo.toml. Too much project content (deps, features, profiles, metadata), and adding lint rules could break builds (e.g., `missing_docs = "deny"` errors on every undocumented item).

## What validate checks

### [workspace.lints] (R26-R29)

**R26:** `[workspace.lints.rust]` section exists with required entries:
- `unsafe_code = "forbid"`
- `unused_results = "deny"`
- `dead_code = "deny"`
- `unused_crate_dependencies = "deny"`
- `missing_debug_implementations = "warn"` (or stricter)

**R27:** `[workspace.lints.clippy]` section exists with required lint group settings:
- `all = { level = "deny", priority = -1 }`
- `pedantic = { level = "deny", priority = -1 }`
- `nursery = { level = "deny", priority = -1 }`
- `cargo = { level = "deny", priority = -1 }`
- Plus specific overrides: unwrap_used, expect_used, panic, todo, dbg_macro, print_stdout, print_stderr, disallowed_methods, disallowed_types, etc.

**R29:** Per-crate `[lints] workspace = true` for workspace members.

### Current bug

R26-R29 check at `primary_workspace_root()` which returns the ROOT Cargo.toml. In steady-parent, the root workspace has NO lints (it's a virtual workspace for packages only). validator-rust's extensive lints go UNCHECKED because they're in `apps/validator-rust/Cargo.toml`.

**Fix needed:** Validate must iterate ALL discovered workspaces and check lints in each workspace's Cargo.toml. For standalone crates (substack-publisher), check `[lints]` directly (not `[workspace.lints]`).

## Per-crate Cargo.toml

For workspace members: check `[lints] workspace = true`.
For standalone crates: check `[lints.rust]` and `[lints.clippy]` have required entries directly.

In steady-parent: `packages/low-expectations` and `packages/seo-site-files` have NO `[lints] workspace = true` — correctly flagged by R29.

## No generation, no merge

Validation reports what's missing. User adds lints manually. guardrail3 provides the canonical lint config via `guardrail3 show-module canonical/cargo-lints` as reference.
