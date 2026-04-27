Goal

Repair the remaining `rs/arch` source-lane boundary defect. `g3rs-arch-source-checks` should consume prebound crate-plus-lib-facade inputs instead of rebuilding a `facade_map` and pairing crates to lib surfaces locally in `run.rs`.

Approach

- Add a prebound source input type that carries one source crate plus its optional lib facade surface.
- Add a red run-boundary test proving `run.rs` still depends on local crate-to-facade pairing instead of the new prebound input.
- Update `g3rs-arch-ingestion` to bind those crate/lib facade pairs once.
- Rewrite `g3rs-arch-source-checks` `run.rs` to dispatch over the new pair inputs and keep the `mod.rs` and `path_attr` lanes separate.
- Add run-level assertions support if needed for the new source run test.

Key decisions

- Keep `mod.rs` facade checks on the plain facade-surface lane.
  - Reason: `g3rs-arch/mod-facade-only` is already naturally one-surface-at-a-time and does not need crate pairing.
- Keep `path_attr_sites` unchanged.
  - Reason: that lane was already repaired and does not share this defect.
- Prebind only `crate + optional lib facade`, not a larger crate-plus-all-surfaces bag.
  - Reason: `g3rs-arch/lib-facade-only` and `g3rs-arch/feature-gated-exports` each need exactly that local assertion unit.

Files to modify

- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/run.rs`
