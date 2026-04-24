## Summary

Fixed the Astro source-pipeline effectiveness hole. `TS-ASTRO-CONFIG-07` now fails when `astro-pipeline` rules are enabled but the route-scoped rules lack usable `routeGlobs` or `endpointGlobs`, and Astro ingestion now carries a typed per-lane fact for which route-scoped pipeline rules are actually effective.

## Decisions made

- Fixed the bug in `ts/astro`, not in the plugin.
  - Reason: the bug was that guardrails called the plugin "wired and effective" without proving the config shape that makes the plugin effective.
- Normalized the ESLint option fact in Astro ingestion.
  - Added per-lane lists of effective route-scoped pipeline rules instead of a raw boolean.
  - Reason: config checks should consume typed family facts, not re-inspect raw ESLint option JSON.
- Kept the fix inside `TS-ASTRO-CONFIG-07`.
  - Reason: the existing rule already owns "wired and effective". Missing usable scope options means the plugin is not effective, not that a new rule surface is needed.
- Tightened scope-option validation to real string arrays.
  - Reason: non-empty non-string arrays such as `[1]` must not count as valid route or endpoint scope.

## Key files for context

- `.plans/2026-04-24-143024-ts-astro-pipeline-options-bug.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
  - result:
    - landing is now green
    - `TS-ASTRO-CONFIG-07` reports effective only when the lane has required rules plus usable route or endpoint scope options
- `pnpm --dir /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing exec eslint --print-config src/components/mdx-content.tsx | rg -n "astro-pipeline/no-authored-content-fs-read|astro-pipeline/no-side-loader-imports|routeGlobs|endpointGlobs"`
  - result:
    - live ESLint config for the runtime MDX module includes the `astro-pipeline` rules and scope options
- Adversarial review:
  - first pass found three real gaps:
    - malformed numeric arrays counted as valid
    - lane-level boolean was too ad hoc
    - endpoint-only scope branch was not explicitly tested
  - second pass result:
    - `No concrete findings.`

## Next steps

- Reinstall the local `g3ts` binary so the installed CLI matches this commit.
- The next Astro follow-up, if we stay on TS, is still the larger content-family work rather than more `ts/astro` scope unless another real bug appears.
