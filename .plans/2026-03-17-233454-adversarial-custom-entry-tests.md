# Adversarial tests for custom entry detection in generate dry-run

**Date:** 2026-03-17 23:34
**Task:** Write destructive adversarial tests that break `extract_custom_entries` / `collect_toml_entries` in diff.rs

## Goal
Append integration tests to `adversarial_generate.rs` that expose parser limitations in the line-based `{ path = ... }` / `{ name = ... }` entry detection.

## Input Information
The parser (`collect_toml_entries` in diff.rs) works by:
1. Iterating lines
2. Checking if trimmed line starts with `{ path =`, `{path =`, `{ name =`, or `{name =`
3. Stripping trailing comma, inserting into BTreeSet
4. Comparing actual vs generated sets to find custom entries

Known weaknesses:
- Multiline entries: only first line matched, continuation lines ignored
- Comments: `# { path = ...}` does NOT start with `{ path =` after trim (the `#` prefix prevents matching) -- so this is actually handled correctly
- Whitespace: `{path=` (no space before `=`) is NOT matched -- only `{path =` with space before `=`
- No section awareness: entries in disallowed-methods vs disallowed-types are treated identically
- Override duplication: if override file contains an entry already in the generated base, it gets included twice in the generated output

## Approach
Write 10 focused integration tests, each targeting a specific parser weakness. Tests that expose bugs are marked with `// BUG:` comments. Tests use the same helper pattern as existing tests (temp dir, minimal config, Command invocation).

## Files to Modify
- `apps/guardrail3/tests/adversarial_generate.rs` -- append new tests
