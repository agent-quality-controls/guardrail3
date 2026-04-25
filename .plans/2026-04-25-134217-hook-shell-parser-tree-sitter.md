# Goal

Replace the remaining hand-written hook shell token/segment parsing with parser-backed Bash facts.

The target is `packages/parsers/hook-shell-parser`. The package should use `tree-sitter-bash` for shell syntax, command boundaries, arguments, command substitutions, and wrapper command strings. Guardrail-specific semantics such as "does this hook line fail open" may stay in hook code, but quote/operator/token parsing should not be owned by guardrails.

# Approach

1. Add `tree-sitter` and `tree-sitter-bash` to `hook-shell-parser-runtime`, using the same versions already used by `package-script-command-parser`.
2. Add a small AST adapter module in `hook-shell-parser-runtime`.
   - It loads the Bash parser once per call.
   - It exposes command records with source ranges, executable text, arguments, and separator metadata.
   - It exposes command substitutions using AST nodes rather than `$(` string scanning.
   - It exposes shell word normalization using parsed word nodes, not quote-state scanning.
3. Replace `command_query/lex.rs` internals with calls into the AST adapter while keeping its public functions stable for `engine.rs` and `wrappers.rs`.
4. Replace parser support helpers that currently split semicolons, strip comments, identify leading commands, and detect command substitution using character scanners.
5. Keep non-syntax orchestration in `parser.rs` where appropriate.
   - Function collection, live/dead branch state, and hook-specific executable-line normalization are parser consumers, not standalone shell lexers.
   - If a branch decision needs a command name, derive it from AST facts.
6. Add or update tests for representative bugs:
   - quoted `#` is not treated as a comment
   - escaped `#` is not treated as a comment
   - semicolons inside quotes do not split commands
   - command substitutions in assignments are found from AST
   - `bash -c` wrapper command strings are parsed by the same AST path
7. Verify:
   - `cargo test -q --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace`
   - `cargo clippy -q --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace --all-targets -- -D warnings`

# Key Decisions

- Use `tree-sitter-bash`, not `shell-words`, because the hook parser needs command boundaries, command substitutions, and function/control-flow syntax, not just argument splitting.
- Keep the existing `command_query::lex` function names as an internal compatibility layer. This limits the diff while changing the implementation root.
- Do not broaden TypeScript scope in this change. The TS package script parser was already moved to `tree-sitter-bash`; this change closes the equivalent Rust/shared hook parser gap.

# Files To Modify

- `packages/parsers/hook-shell-parser/crates/runtime/Cargo.toml`
- `packages/parsers/hook-shell-parser/crates/runtime/src/lib.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/shell_ast.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/lex.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`
