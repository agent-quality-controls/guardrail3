# Summary

Replaced the hook shell parser's command-query lexing with `tree-sitter-bash` facts and moved several hook support helpers onto the same AST adapter. The parser now uses AST nodes for command segments, shell words, command substitutions, comments, heredoc starts, assignments, and fail-open command detection instead of quote/operator character scanners in the guardrail code.

# Decisions Made

- Added a local `shell_ast` adapter inside `hook-shell-parser-runtime` instead of spreading `tree-sitter` calls through the parser. This keeps parser-library use centralized and preserves the existing `command_query::lex` API for callers.
- Kept hook semantics in `support.rs`: branch liveness, function body tracking, dispatcher detection, and fail-open classification remain guardrail logic, but shell tokenization is delegated to the AST adapter.
- Added regression tests for semicolons inside quoted shell arguments. This was the specific class of bug the hand-rolled splitter was most likely to mishandle.
- Did not turn `cargo clippy -D warnings` into a completion gate for this package. It currently fails on broad preexisting lint debt across `command_query`, `parser`, and `support`; fixing that would be a separate cleanup, not part of the parser replacement.

# Key Files For Context

- `packages/parsers/hook-shell-parser/crates/runtime/src/shell_ast.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/lex.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`
- `.plans/2026-04-25-134217-hook-shell-parser-tree-sitter.md`

# Verification

- `cargo test -q --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace`
- `cargo clippy -q --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace --all-targets -- -D warnings` was attempted and failed on preexisting package-wide lint debt unrelated to this change.

# Next Steps

- If hook-shell-parser should become a strict clippy package, handle that as a dedicated lint cleanup with its own plan. The current lint failures include missing private docs, excessive nesting, too-many-arguments, too-many-lines, string slicing, and arithmetic-side-effect lints in existing parser/control-flow code.
