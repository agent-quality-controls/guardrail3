# CLI help generation + per-crate dependency allowlists

**Date:** 2026-03-16 14:33
**Scope:** help_gen.rs, dependency_allowlist.rs, config types, main.rs

## Summary
Two features added in parallel:
1. Graf-style CLI help injection — every --help now shows full command tree, profiles, workflow, check IDs
2. Per-crate profiles and dependency allowlists (R-DEPS-01, R-DEPS-02)

## CLI Help
- help_gen.rs (407 lines) injected via after_help() into clap command tree
- Top-level: getting started, profiles, workflow, commands, output formats, scope flags
- rs validate: all Rust check IDs grouped by category
- ts validate: all TypeScript check IDs grouped by category
- rs init: profiles explained, files created, examples

## Per-Crate Allowlists
- CrateConfig gains `profile` and `allowed_deps` fields
- R-DEPS-01: flags unauthorized dependencies not in allowlist
- R-DEPS-02: warns library crates without an allowlist
- Per-crate profile used for clippy generation (library crates get I/O bans)
