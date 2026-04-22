Summary
- Fixed hook shell parsing so escaped `#` stays executable text and inline commands after single-line function definitions are not lost when later comments or strings contain `}`.
- Updated the hook-side inline comment helper so hook rules agree with the parser on escaped-hash handling.

Decisions made
- Fixed both issues at the parser/support boundary instead of teaching individual hook rules special cases.
- Kept the parser and hook helper semantics aligned so comment parsing does not drift between packages.
- Combined the two parser-support bugs because both lived in the same shell-inline parsing surface and were verified together.

Key files for context
- packages/parsers/hook-shell-parser/crates/runtime/src/support.rs
- packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_14_no_bypass_instructions/rule_tests/golden.rs
- .plans/2026-04-22-232019-hook-shell-parser-function-tail-fix.md
- .plans/2026-04-22-232054-hook-shell-escaped-hash-parser-fix.md

Next steps
- Finish and commit the `RS-HOOKS-SOURCE-15` discarded-trigger gating fix.
- Run another attack pass on hooks/parser after that commit lands.
