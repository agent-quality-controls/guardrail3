# Rule 01 adversarial audit fixes — 45 tests final

**Date:** 2026-03-20 16:43

## Summary
Fixed all 8 gaps from adversarial audit. Removed 3 redundant tests. Added 2 single-app isolation tests. Every test now has: exact error count, title content assertion, file field assertion.

## Changes
- Title content checks added to: crates_dir_empty, crates_is_file_not_dir, inner_hex_crates_is_file, three_apps_three_different_failures
- Circular symlink test: replaced non-deterministic bound with tighter assertion
- hex_in_hex_inner_has_wrong_dirs: specific per-error title checks for each missing/unexpected dir
- Removed redundant: leaf_crate_not_mistaken_for_hex_in_hex, error_message_includes_app_name_and_file_field, inner_hex_error_distinguishable_from_outer
- Added: missing_crates_dir_devctl_only, missing_crates_dir_worker_only

## Next steps
- Clean up old tasks. Move to rule_02 adversarial testing.
