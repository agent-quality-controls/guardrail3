# Restructure into apps/guardrail3/ with workspace crates

**Date:** 2026-03-16 17:59
**Task:** Move from single crate to workspace with apps/ + packages/ structure

## Current structure (single crate)
```
Cargo.toml          ← single [package]
src/
  main.rs
  lib.rs
  cli.rs
  help_gen.rs
  fs.rs
  domain/           ← modules in one crate
  ports/
  app/
  adapters/
  commands/
  report/
tests/
fuzz/
```

## Target structure (workspace)
```
Cargo.toml          ← [workspace] members = ["apps/guardrail3"]
apps/
  guardrail3/
    Cargo.toml      ← [package] with all deps
    src/
      main.rs
      lib.rs
      cli.rs
      help_gen.rs
      fs.rs
      domain/
      ports/
      app/
      adapters/
      commands/
      report/
tests/              ← stays at repo root (shared test infrastructure)
fuzz/               ← stays at repo root
packages/           ← empty for now (future shared libraries)
```

## Approach
Since we're keeping it as ONE crate (just moving it into apps/), this is simpler than splitting into multiple crates. The key changes:
1. Create workspace Cargo.toml at root
2. Move current Cargo.toml into apps/guardrail3/
3. Move src/ into apps/guardrail3/src/
4. Update test paths
5. Update guardrail3.toml with per-crate config
6. Update all file references in tests and configs

## Why not split into multiple crates?
The hex arch is already enforced at the module level via R-ARCH-02 (dependency flow). Splitting into separate crates would enforce it at compile time (stronger) but requires duplicating dependencies across Cargo.toml files and adds significant build complexity. The module-level enforcement is sufficient for now.
