# Extract shared family types

**Date:** 2026-04-07 11:14
**Scope:** 8 new g3rs-{family}-types packages

## Summary
Extracted the shared input type from each checks/types crate into standalone `g3rs-{family}-types` packages. Ingestion packages now depend on the shared types package instead of the checks facade. The backwards dependency (ingestion → checks) is eliminated.

## New packages
- packages/rs/cargo/g3rs-cargo-types/
- packages/rs/clippy/g3rs-clippy-types/
- packages/rs/deny/g3rs-deny-types/
- packages/rs/deps/g3rs-deps-types/
- packages/rs/fmt/g3rs-fmt-types/
- packages/rs/garde/g3rs-garde-types/
- packages/rs/release/g3rs-release-types/
- packages/rs/toolchain/g3rs-toolchain-types/

## Dependency chain change
Before: ingestion → checks (facade) → types
After:  ingestion → g3rs-{family}-types ← checks

## What each checks/types crate now does
Re-exports from the shared types package. The struct definitions moved to the new package.
