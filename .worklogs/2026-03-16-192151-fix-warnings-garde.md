# Fix warnings + real garde validation

**Date:** 2026-03-16 19:21
**Scope:** hex_arch_checks, cli, main, init, README, mutants.toml, release-plz.toml

## Summary
Fixed R-ARCH-01 path resolution (uses workspace member dirs). Added real garde validation on CLI (format must be text|json|md|markdown, profile must be service|library). .validate() now actually called at runtime. Created README.md, mutants.toml. Fixed release-plz.toml package entry.
