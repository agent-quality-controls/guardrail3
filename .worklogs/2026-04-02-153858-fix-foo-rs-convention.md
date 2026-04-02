# Fix RS-ARCH-03: eliminate foo.rs convention (23 renames)

**Date:** 2026-04-02 15:38

## Summary
Moved 23 foo.rs files to foo/mod.rs. All RS-ARCH-03 "foo.rs convention"
violations eliminated (21 initial + 2 nested discovered after first round).
Zero remaining.

## Files moved
21 initial renames across: cli, core, families (clippy, code, deps, garde,
hexarch, hooks-rs, hooks-shared, libarch, release, test), domain/modules.
2 nested renames: code/parse/visitors, release/release_support/workflows.
