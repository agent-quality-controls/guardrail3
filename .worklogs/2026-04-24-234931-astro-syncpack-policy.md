# Astro Syncpack Policy

## Summary

Replaced the rejected hand-rolled Astro dependency floor validation with Syncpack contract enforcement. G3TS now enforces that Astro apps install Syncpack, run `syncpack lint` fail-closed, and provide a canonical `.syncpackrc` for required stack pins and forbidden dependencies.

## Decisions Made

- Removed npm SemVer/range parsing from `package-json-parser`; package manifests now expose structural dependency names and metadata only.
- Added `syncpack-config-parser` as a structural JSON parser only; Astro-specific package policy matching stays in Astro ingestion.
- Moved required Astro stack pins and forbidden dependency policy into Astro ingestion typed facts.
- Kept Astro config checks limited to rendering typed facts and reporting setup/config failures.
- Removed the old direct Astro `velite` dependency scan; Syncpack bans own that dependency policy now.
- Tightened package script safety so one safe `syncpack lint` script cannot hide another fail-open `syncpack lint || true` script.
- Made Syncpack `source` matching exact and literal relative to the selected `.syncpackrc`; aliases, globs, and app-local repo-relative entries do not satisfy the Astro contract.

## Key Files For Context

- `.plans/2026-04-24-211626-astro-syncpack-stack-policy.md`
- `packages/parsers/syncpack-config-parser`
- `packages/parsers/package-json-parser/crates/runtime/src/parser.rs`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_08_syncpack_package_and_script.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_09_syncpack_stack_pins.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_10_syncpack_forbidden_deps.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/package-json-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/parsers/syncpack-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo clippy -q --manifest-path packages/parsers/syncpack-config-parser/Cargo.toml --workspace --all-targets -- -D warnings`
- `cargo clippy -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --all-targets -- -D warnings`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

## Adversarial Review

- First attack pass found source alias acceptance, parser policy leakage, fail-open script bypasses, missing `specifierTypes` parsing, and stale messages.
- Second attack pass found missing ingestion regressions, noncanonical ban gaps, incomplete parser structural tests, duplicate unsafe script surface bypass, and diagnostics that forced iterative fixes.
- Final attack pass found production config-check messages duplicating exact package policy; fixed by rendering ingestion-owned typed facts.
- Final convergence result: no blocking findings.

## Next Steps

- Move pre-existing `g3ts-package/local-banned-dependencies` banned dependency policy to Syncpack.
- Replace pre-existing `g3ts-package/root-scripts` substring script detection with package-script command facts.
- Rebuild package script/shell parsing on a real shell parser or consolidate it with `hook-shell-parser`.
