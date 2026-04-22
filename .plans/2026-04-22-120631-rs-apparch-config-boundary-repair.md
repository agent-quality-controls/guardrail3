Goal
- Repair `rs/apparch` config checks so rule dispatch consumes ingestion-owned atomic inputs instead of rebuilding indexes and rebinding whole-family bags inside `g3rs-apparch-config-checks`.

Approach
- Read the live `apparch` config rule signatures and group them by owned input shape.
- Add a proving test in `g3rs-apparch-config-checks` that fails while `run.rs` still depends on bag inputs and local rebinding.
- Narrow `g3rs-apparch-types` config input into explicit prebound lanes:
  - per-crate dependency checks
  - per-crate purity checks
  - per-patch bypass checks
  - same-layer cycle check input
- Move the fan-out and prebinding logic into `g3rs-apparch-ingestion`.
- Reduce `g3rs-apparch-config-checks::run` to pure dispatch over those precomputed inputs.
- Keep individual rule files pure and focused. Do not expand source-check scope in this change.
- Run package tests and `g3rs validate` for the touched `apparch` slice.

Key decisions
- Keep the repair at the config lane only.
  - Alternative rejected: refactor config and source together. That would widen scope and slow convergence.
- Use prebound atomic inputs instead of keeping `crates_by_path` reconstruction in `run.rs`.
  - Alternative rejected: hide the rebinding behind a helper in config-checks. That would preserve the seam defect.
- Keep same-layer cycles as an ingestion-owned set input.
  - Alternative rejected: force per-edge cycle rule inputs. The cycle rule is inherently graph-shaped.

Files to modify
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/*`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/*` if the new proving test needs assertion helpers
