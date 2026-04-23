# Goal
Bring the active `ts/astro` family and `eslint-plugin-astro-pipeline` error messages into the repo's required message format so they match the Rust standard: specific bad thing, specific fix, specific reason.

# Approach
1. Read the local message-format plan and representative Rust rule messages to extract the concrete rules to follow.
2. Rewrite the active Astro guardrail titles and messages in:
   - `packages/ts/astro/g3ts-astro-config-checks`
   - `packages/ts/astro/g3ts-astro-file-tree-checks`
   so each finding names the exact file/surface, the exact fix, and the architectural reason.
3. Rewrite the active ESLint plugin messages in:
   - `packages/ts/eslint-plugin-astro-pipeline/src/rules/*`
   so each lint message names the offending module, the forbidden operation, the exact allowed replacement, and why the boundary exists.
4. Update tests that currently pin message text.
5. Run package tests.
6. Run an adversarial review against the message-format plan and the touched code. Fix any gaps it finds.

# Key Decisions
- Keep titles short and concrete, matching the Rust pattern.
  - Reject vague policy-only titles like "not effective" or "bypasses collections" without the concrete subject.
- Put the fix and the reason in the message, not only in the title.
  - Reject one-sentence policy messages that only say "must not".
- Do not widen scope beyond the currently active Astro slice.
  - No new rules.
  - No render-validator work.

# Files To Modify
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_01_astro_package_present.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_02_astro_check_present.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_03_astro_eslint_plugin_package_present.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_05_astro_eslint_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_06_pipeline_plugin_package_present.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_01_astro_config_exists.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_02_content_config_exists.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_03_live_config_exists.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_04_no_route_markdown_pages.rs`
