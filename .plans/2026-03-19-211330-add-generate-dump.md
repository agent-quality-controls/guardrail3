# Add --dump-dir to generate/diff for verification

**Date:** 2026-03-19 21:13
**Task:** Add ability to dump generated files to a directory for inspection

## Approach
Add `--dump-dir <path>` flag to `rs diff` and `ts diff` commands. When provided, writes all generated files to the specified directory instead of just showing the summary. This lets us inspect exactly what generate would produce and manually compare against existing files.
