Goal

Make `packages/rs/arch/g3rs-arch-ingestion` validate cleanly under the current rules.

Approach

- Normalize the workspace root:
  - add `rust-toolchain.toml`
  - add `rustfmt.toml`
  - add `clippy.toml`
  - add `deny.toml`
  - add `guardrail3-rs.toml`
  - make publish intent explicit
- Remove the local `crates/types` wrapper if it is only forwarding shared arch types plus an error.
  - move the ingestion error into runtime, like the already-clean `g3rs-apparch-ingestion` package
  - let the facade crate export runtime functions and the runtime-owned error directly
- Reshape runtime:
  - split the large `run.rs` into `run/mod.rs` plus focused submodules
  - add an `fs.rs` shim so `view.rs` does not call `std::fs` directly
  - move `ingest_tests` off the orphaned sidecar and onto owned `x_tests` sidecars for real files
- Reshape tests:
  - add `crates/assertions -> crates/runtime`
  - move final result proof into shared assertions files
  - replace weak `expect(...)` messages with specific failure messages

Key decisions

- Follow `packages/rs/apparch/g3rs-apparch-ingestion` as the package pattern instead of inventing a new ingestion layout.
- Fix the package, not the rules, unless a real contradiction appears during the cleanup.

Files to modify

- `packages/rs/arch/g3rs-arch-ingestion/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-ingestion/src/lib.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/**`
- `packages/rs/arch/g3rs-arch-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/assertions/src/**`
- delete `packages/rs/arch/g3rs-arch-ingestion/crates/types` if it is only a wrapper
