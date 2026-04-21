## Summary

Built `ts/apparch` wave 2 purity checks in the config lane. This iteration adds TypeScript-specific purity rules for `src/types` and `src/logic`, keeping them free of Next and React runtime imports while leaving the rest of the apparch surface unchanged.

## Decisions made

- Mirrored Rust purity intent, not Rust dependency mechanics.
  - Rust `apparch` purity is allowlist-based over crate dependencies.
  - TS apparch does not yet have a comparable policy file or dependency-kind model.
  - Chosen adaptation: ban framework runtime imports in pure layers.

- Kept purity in the config lane.
  - Why: import/dependency policy belongs with the existing layer-direction rules.
  - Rejected: moving purity into source checks, because the issue is dependency coupling, not public-surface shape.

- Scoped purity to Next and React runtime modules for wave 2.
  - Banned in `types` and `logic`:
    - `next`
    - `next/*`
    - `react`
    - `react/*`
    - `react-dom`
    - `react-dom/*`
  - Why: these are the clearest framework-coupling imports for a Next app.
  - Rejected: a broad third-party allowlist or denylist, because that would be speculative without a dedicated TS policy surface.

- Closed the `react/*` subpath hole before commit.
  - Initial implementation only matched bare `react`.
  - Fixed at the helper boundary and proved with `react/jsx-runtime`.

## Key files for context

- `.plans/2026-04-21-192135-ts-apparch-purity-wave2.md`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_08_types_purity.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_09_logic_purity.rs`
- `packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/support.rs`
- `packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/ts_apparch_config_06_types_purity.rs`
- `packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/ts_apparch_config_07_logic_purity.rs`
- `packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/ts/apparch/g3ts-apparch-config-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/apparch/g3ts-apparch-config-checks/Cargo.toml`
- `g3rs validate --path packages/ts/apparch/g3ts-apparch-config-checks`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/web --family apparch`

## Adversarial review

- Local adversarial pass found one real gap:
  - `react/jsx-runtime` and other `react/*` imports were not matched by the initial purity helper.
- Fixed:
  - expanded the helper to cover `react/*`
  - added test proof for the subpath case
- No remaining concrete blocker found in this wave 2 scope.

## Next steps

- If `apparch` goes deeper, the next likely slice is same-layer cycles or a broader TS policy surface for external dependency purity.
- `ui` and `lib` should stay out unless we define a precise ownership contract for them.
