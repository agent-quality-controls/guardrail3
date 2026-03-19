# Rust-toolchain.toml resolution empirically verified

**Date:** 2026-03-19 15:59
**Scope:** coverage/rust_toolchain.rs

## Verified facts
1. Walk-up from CWD, nearest wins
2. Crosses workspace boundaries (project root override found from substack-publisher)
3. Intermediate files shadow per-directory (adapters/ override found, domain unaffected)
4. All four Rust config files (clippy, deny, rustfmt, rust-toolchain) behave identically
