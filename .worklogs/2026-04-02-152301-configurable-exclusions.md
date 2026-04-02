# Configurable path exclusions via guardrail3.toml

**Date:** 2026-04-02 15:23

## Summary
Replaced hardcoded project-specific exclusion patterns with configurable
`excluded_paths` in guardrail3.toml. Built-in exclusions (target,
.claude/worktrees) stay hardcoded. Project-specific patterns (tests/fixtures,
tests/snapshots) now come from config.

## Changes
- domain/config/types.rs: added `excluded_paths: Option<Vec<String>>` to RustConfig
- structure/src/lib.rs: collect() takes excluded_paths, stores in StructureFacts.
  filter_to_roots() uses is_excluded_by_builtin (hardcoded) + is_excluded_by_config
  (from config). Removed is_excluded_live_root_dir import.
- runtime/src/lib.rs: reads excluded_paths from config, passes to structure::collect
- guardrail3.toml: added `excluded_paths = ["tests/fixtures", "tests/snapshots"]`

## How it works
- Built-in (always): `target`, `.claude/worktrees`
- Configurable: `[rust] excluded_paths = ["tests/fixtures", ...]`
- Pattern matching: path segment windows — "tests/fixtures" matches any path
  containing those two consecutive segments
- Applied during legality filtering — excluded dirs/files stripped from legality output
