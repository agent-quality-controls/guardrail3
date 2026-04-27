Goal

Build the apparch family as real packages with the stable package contract: family-level types, real ingestion, config checks for dependency direction, and source checks for public trait ownership.

Approach

1. Create `packages/rs/apparch/g3rs-apparch-types` with the minimal public contract:
   - layer enum
   - crate identity
   - dependency edge
   - public trait fact
   - one shared input-failure type
   - config/source lane inputs only
2. Create `packages/rs/apparch/g3rs-apparch-config-checks` with lane-local re-export types and runtime rules for `g3rs-apparch/types-dependency-direction..03`.
3. Create `packages/rs/apparch/g3rs-apparch-source-checks` with lane-local re-export types and runtime rule for `g3rs-apparch/io-traits-in-types`.
4. Create `packages/rs/apparch/g3rs-apparch-ingestion` that:
   - discovers the pointed workspace root
   - resolves workspace members
   - determines layer from path segments
   - parses member `Cargo.toml` once
   - builds workspace-internal dependency edges
   - walks reachable Rust modules and extracts public trait facts
   - emits typed input failures instead of exposing raw parser/runtime internals
5. Add unit and ingestion tests first for the intended rule behavior and the minimal contract:
   - dependency direction violations
   - allowed dependency directions
   - public trait violations only in `io/*`
   - layer detection
   - dependency edge extraction
   - source fact extraction
6. Verify with family package tests and `git diff --check`.

Key decisions

- Do not create a custom `dep` lane. Dependency rules live in `config` because they are driven by parsed `Cargo.toml` data.
- Do not create a `filetree` lane. The current apparch rule inventory does not justify one.
- Keep the public family contract minimal. Richer parse and traversal detail stays ingestion-private.
- Keep unsupported lanes absent from the public API. No fake stubs.
- Start with the simple family package shape, not old app integration.

Files to modify

- `packages/rs/apparch/g3rs-apparch-types/**`
- `packages/rs/apparch/g3rs-apparch-config-checks/**`
- `packages/rs/apparch/g3rs-apparch-source-checks/**`
- `packages/rs/apparch/g3rs-apparch-ingestion/**`
