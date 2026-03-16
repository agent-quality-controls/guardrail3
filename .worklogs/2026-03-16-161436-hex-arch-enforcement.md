# Hex arch enforcement — R-ARCH-01/02/03

**Date:** 2026-03-16 16:14
**Scope:** hex_arch_checks.rs, mod.rs, help_gen.rs

## Summary
Three new checks that validate hex arch structure and dependency flow using TOML parsing + filesystem checks (not grep, not AST — structural validation).

- R-ARCH-01: Service crate missing hex arch dirs (domain/, adapters/)
- R-ARCH-02: Layer dependency flow violation (domain→adapters, etc.)
- R-ARCH-03: Library in packages/ depends on service internal crate in apps/

12 unit tests with StubFs mock filesystem.
