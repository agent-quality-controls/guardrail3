## Summary

Fixed the `TS-ASTRO-CONFIG-07` diagnostic so it describes the real Astro pipeline wiring contract instead of falsely talking only about route and endpoint code. Added a landing-shaped config-check test proving that route-only `astro-pipeline` wiring still fails because the shared pipeline rules must also run on the generic Astro, TS, and TSX source lanes.

## Decisions made

- Kept the `TS-ASTRO-CONFIG-07` enforcement logic strict.
  - Reason: the real landing app wires `astro-pipeline` only on `src/pages/**`, which leaves `src/components/mdx-content.tsx` outside the plugin surface even though that file contains runtime MDX evaluation.
- Fixed the message instead of weakening the checker.
  - Reason: the failure is valid, but the old message claimed the contract was only about route and endpoint code. That was misleading and sent the debugging effort in the wrong direction.
- Added a lane-specific test helper instead of changing the shared parser or Astro ingestion.
  - Reason: this bug was diagnostic and contract-communication drift inside the config checks package, not a discovery or parser bug.
- Did not add pipeline-option enforcement in this fix.
  - Reason: the real app also omits `astro-pipeline` rule options, but that is a separate missing contract. This fix stays scoped to the concrete `TS-ASTRO-CONFIG-07` bug.

## Key files for context

- `.plans/2026-04-24-141143-ts-astro-config-07-contract-fix.md`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/eslint.config.mjs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/src/components/mdx-content.tsx`

## Verification

- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
  - result:
    - `TS-ASTRO-CONFIG-07` still fires
    - message now states the Astro, TS, and TSX source-lane contract
- `pnpm exec eslint --print-config src/components/mdx-content.tsx`
  - result: no `astro-pipeline` rules active on the runtime MDX bridge
- `pnpm exec eslint --print-config src/pages/index.astro`
  - result: `astro-pipeline` rules active only on the route lane
- Adversarial review against the plan, changed files, and the real landing app:
  - final result: `No concrete findings.`

## Next steps

- If we want the Astro pipeline contract to be actually fail-closed, add a follow-up config check for required `astro-pipeline` rule options.
- The real landing app still needs a config change:
  - wire `astro-pipeline` on the generic Astro, TS, and TSX source lanes
  - pass the required plugin options so the route and MDX-runtime rules are not inert
