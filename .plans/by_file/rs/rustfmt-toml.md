# rustfmt.toml

## Location

**Where rustfmt looks:** Walks UP from the file being formatted. Checks each directory for `rustfmt.toml` or `.rustfmt.toml`. Nearest wins — no merging. Falls back to `~/.config/rustfmt/` if none found.

**In steady-parent:**
- `apps/validator-rust/rustfmt.toml` (11 lines — 7 settings + commented nightly hints)
- `apps/substack-publisher/rustfmt.toml` (7 lines — 7 settings only)
- NO root rustfmt.toml (root packages have no formatting config)

**Valid locations:** Same as clippy.toml — one per workspace/standalone crate. Per-crate would shadow, same danger.

**Rule: one rustfmt.toml per Rust workspace/standalone crate.**

## Contents

~60 stable configuration options exist. guardrail3 sets 7:

| Key | guardrail3 value | rustfmt default | Why we set it |
|---|---|---|---|
| edition | "2024" | "2015" | Must match project edition |
| max_width | 100 | 100 | Same as default, explicit |
| tab_spaces | 4 | 4 | Same as default, explicit |
| use_field_init_shorthand | true | false | Cleaner struct init |
| use_try_shorthand | true | false | `?` over `.unwrap()` |
| reorder_imports | true | true | Deterministic import order |
| reorder_modules | true | true | Deterministic module order |

**User might also have:** `hard_tabs`, `newline_style`, `fn_params_layout`, `ignore`, `format_code_in_doc_comments`, `normalize_comments`, `wrap_comments`, `imports_indent`, `imports_layout`, `merge_imports`, etc.

## Category: Fully-owned vs Merge-managed?

In steady-parent, both files contain ONLY the guardrail3 settings — zero user customization. The nightly hints in validator-rust are commented out (inactive).

But a different project COULD have extra formatting preferences. If we overwrite, those are lost.

However: formatting is uniform within a project. You don't want different formatting in different files. A rustfmt.toml should be the same everywhere. If the user has `fn_params_layout = "Compressed"`, that's a project-wide choice — and guardrail3 has no opinion about it (we don't enforce a specific function param layout).

**Decision: Merge-managed.**
- guardrail3 ensures its 7 keys are present with correct values
- User's extra keys are preserved
- On new file: generate all 7 keys (current behavior)
- On existing file: add missing keys, leave user keys alone

**Why not fully-owned:** A project with `ignore = ["generated/**"]` or `newline_style = "Unix"` would lose those settings on overwrite. These are legitimate project configs.

**edition special case:** `edition` MUST match the project's Rust edition. On merge: if `edition` exists, LEAVE it. If missing, read the workspace `Cargo.toml`'s `edition` field (or `[workspace.package] edition`) and use that. Fall back to "2024" only if undetectable. NEVER write "2024" for a 2021 project — rustfmt's 2024 edition formatting is materially different (import grouping, expression formatting) and would cause a massive diff on first `cargo fmt`.

## Algorithm

### On `generate` (existing file):
```
1. Parse with toml_edit
2. For each guardrail key (max_width, tab_spaces, use_field_init_shorthand, use_try_shorthand, reorder_imports, reorder_modules):
   - If missing: ADD with guardrail value
   - If present: LEAVE (validate warns if different from guardrail value for non-edition keys)
3. For edition:
   - If missing: ADD "2024"
   - If present: LEAVE (project-specific)
4. All other keys: LEAVE
5. Write back with toml_edit
```

### On `generate` (new file):
```
1. Write guardrail3's 7 keys (current behavior)
```

## Override mechanism

None needed. User adds their keys directly to rustfmt.toml. guardrail3 won't touch them.

No removal mechanism needed either — the guardrail keys are non-controversial formatting standards. Nobody needs to remove `reorder_imports = true`.

## Edge cases

1. **Per-crate rustfmt.toml:** Same as clippy.toml — WARN if found in crate subdirectory. It shadows the workspace config. But less dangerous than clippy since it's formatting, not security.

2. **Root packages with no rustfmt.toml:** Generate one if `[rust.packages]` in guardrail3.toml. Root packages should have consistent formatting.

3. **edition mismatch:** If guardrail3 sets "2024" but the project uses Rust 2021, rustfmt might format differently (edition affects import grouping). The merge algorithm leaves existing edition alone, which is correct.

4. **`ignore` patterns:** User might have `ignore = ["generated/**", "proto/**"]`. Must be preserved — these are project-specific paths that shouldn't be formatted.

## Parser

`toml_edit` — same as clippy.toml. Simple key-value at root level, no arrays of tables. Straightforward merge.

## Summary

| What | Action |
|---|---|
| guardrail3's 7 keys | Ensure present |
| edition | Ensure present, don't override existing |
| User's extra keys | Preserve |
| Comments | Preserve |
| Per-crate files | Warn (shadows workspace) |
