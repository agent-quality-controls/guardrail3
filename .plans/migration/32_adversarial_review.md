# Step 32: Adversarial Review of Migration

## Goal
Launch adversarial agents to find issues the migration introduced or missed.

## Task (3 agents in parallel)

### Agent 1: Review syn implementations
Read ALL files in `src/rs/validate/ast_helpers.rs` and the migrated check functions. Find:
- Patterns that syn doesn't catch (e.g., macro-generated code)
- Edge cases in span-to-line-number mapping
- Fallback-to-grep paths that should have been migrated
- Any `unwrap()` or `panic!()` that could crash on malformed input

### Agent 2: Review tree-sitter implementations
Read ALL files in `src/ts/validate/ast_helpers.rs` and migrated TS checks. Find:
- Patterns tree-sitter doesn't catch
- Grammar version mismatches (TS syntax not supported)
- Memory issues with large files

### Agent 3: Write NEW adversarial fixtures
Now that the tool uses AST parsing, write fixtures that try to break the AST approach:
- Syntactically invalid Rust files (syn parse fails — does fallback work?)
- Rust files with proc macro attributes (syn can't expand macros)
- Very large files (does syn OOM?)
- Files with syntax errors midway (partial parse)
- TypeScript with JSX (does tree-sitter handle it?)
- TypeScript with decorators (does tree-sitter handle it?)

## Verification
Each agent produces a report. Any CRITICAL or HIGH findings → create fix tasks.

## On Failure
If agents find issues → go to step 33. If agents find nothing → proceed to step 99.
