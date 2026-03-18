# Create adversarial nightmare monorepo test file

**Date:** 2026-03-18 01:15
**Task:** Create comprehensive adversarial test file for nightmare monorepo scenario

## Goal
Single test file at `apps/guardrail3/tests/adversarial_nightmare_monorepo.rs` that builds a complex monorepo fixture and runs every command against it with 30 assertions.

## Approach
- Copy header pattern from adversarial_generate.rs
- Build setup_nightmare_monorepo function creating full fixture tree
- Write 30 test functions covering rs generate, ts generate, ts init, and idempotency
- Use exact canonical content from canonical.rs for rustfmt.toml and .npmrc matching
- Handle that `--dry-run` on generate routes to diff command (exits with code 1 when changes exist)

## Key decisions
- dry_run output: `path — would create`, `path — would update`, `path — no changes needed`
- Custom entries output: `Custom entries found` with entry details
- ts init dry_run: `Would update guardrail3.toml:` with `+`/`-` lines
- diff exits with code 1 when changes pending (non-zero exit is expected for "would change" scenarios)
