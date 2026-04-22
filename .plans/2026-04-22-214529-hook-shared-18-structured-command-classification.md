# Goal
Fix the remaining production-path boundary defect in `hook_shared_18_executable_command_context_only` so executable-step classification uses structured command context instead of raw `command_text().contains(...)` substring matching.

# Approach
1. Add a regression test in the rule's sidecar tests that proves a substring false positive from the current matcher.
2. Update `rule.rs` to classify each supported family from `ResolvedCommand` structure:
   - command name
   - argument positions
   - helper methods that stay inside the rule slice
3. Keep the scope limited to the rule file and its existing test/assertion surface.
4. Run the targeted package tests and `g3rs validate` to verify the fix.

# Key Decisions
- Use a false-positive regression instead of a synthetic helper test, because the bug is in live production-path classification.
- Fix the rule at the classifier boundary rather than adding another textual normalization layer.
- Keep the assertion crate untouched unless the test surface requires it, which it should not.

# Files to Modify
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule_tests/golden.rs`
