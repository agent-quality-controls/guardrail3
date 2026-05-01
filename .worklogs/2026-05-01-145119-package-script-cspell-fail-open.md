Summary
- Fixed package-script parsing so cspell spellcheck scripts are treated as guardrail-related shell commands.
- This preserves `||` fallback context for cspell scripts instead of letting spelling checks see an apparently safe invocation.

Decisions made
- Added `cspell` and `spellcheck` to the package-script parser's guardrail-related tool/script vocabulary.
- Kept shell parsing delegated to `tree-sitter-bash`; no ad hoc script parsing was added in the spelling family.
- Added parser coverage for `cspell . || true` and unsupported spellcheck shell syntax.

Key files for context
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser_tests/cases.rs`

Next steps
- Use the parser facts from the spelling family instead of inspecting package scripts directly.
