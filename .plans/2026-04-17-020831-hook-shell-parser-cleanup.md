# Goal
Clean `packages/parsers/hook-shell-parser` to the current parser package shape so it validates cleanly with no compatibility aliases and no package-local escape hatches.

# Approach
1. Normalize the package layout from `crates/parser/runtime` to sibling member crates under `crates/`, adding `crates/assertions` and `crates/types` if needed.
2. Move public data structs out of runtime `lib.rs` into a dedicated types crate, keep the root facade minimal, and update repo callers to import parser types from `hook_shell_parser::types::...`.
3. Split runtime logic so `lib.rs` becomes facade-only and `command_query.rs` drops below the line cap by moving related internals into submodules.
4. Replace inline and `src/tests/` test layout with owned sidecar tests and shared assertions.
5. Add missing package root files and README content, then run package tests and validation.

# Key decisions
- Prefer the same parser shape already used in `deny-toml-parser` and `guardrail3-rs-toml-parser` instead of inventing a hook-specific layout.
- Do not preserve old root type exports; update callers to the clean `hook_shell_parser::types::...` path.
- Keep command-query behavior in runtime, but move shared data types to `crates/types` because they are public transport values rather than runtime behavior.

# Files to modify
- `packages/parsers/hook-shell-parser/Cargo.toml`
- `packages/parsers/hook-shell-parser/src/lib.rs`
- `packages/parsers/hook-shell-parser/src/types.rs`
- `packages/parsers/hook-shell-parser/README.md`
- `packages/parsers/hook-shell-parser/clippy.toml`
- `packages/parsers/hook-shell-parser/deny.toml`
- `packages/parsers/hook-shell-parser/crates/**`
- repo callers under `packages/rs/hooks/**`
