# Package Syncpack And Shell Parser Cleanup

## Summary

Moved package-family banned dependency enforcement out of direct manifest scanning and into Syncpack setup/configuration enforcement. Rebuilt `package-script-command-parser` on `tree-sitter-bash` so package script facts come from a real shell parser instead of local command splitting.

## Decisions Made

- `TS-PACKAGE-CONFIG-08` now enforces that package-manager roots list `syncpack`, run `syncpack lint` fail-closed, and provide `.syncpackrc` with exact `source` coverage and canonical banned `versionGroups`.
- Package-family banned dependency names now live in package ingestion as Syncpack policy facts, not in config checks as direct local manifest scans.
- `TS-PACKAGE-CONFIG-06` now requires parsed `only-allow pnpm` execution in `scripts.preinstall`; echoed text and fail-open shell forms do not satisfy it.
- `package-script-command-parser` uses `tree-sitter-bash` with the existing `tree-sitter` 0.25 line to avoid native linker conflicts with the rest of G3TS.
- Syncpack wildcard dependency policy uses `embla-carousel*`; Syncpack docs confirm `versionGroups.dependencies` accepts exact names and glob patterns.
- Adversarial self-review found an initial `only-allow pnpm | tee log` bypass because unsupported syntax was only fail-closed for `eslint`, `astro`, and `syncpack`; fixed by treating `only-allow` as a guarded tool too.

## Key Files For Context

- `.plans/2026-04-25-124853-package-syncpack-shell-parser.md`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/ts_package_config_06_root_scripts.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/ts_package_config_08_local_banned_dependencies.rs`
- `packages/ts/package/g3ts-package-types/src/types.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace`
- `cargo clippy -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --all-targets -- -D warnings`
- `cargo test -q --manifest-path packages/ts/package/g3ts-package-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/package/g3ts-package-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`

## Next Steps

- If package-family crates are later made clippy-clean as standalone workspaces, remove their pre-existing clippy debt separately. This change does not make package checks parse shell or dependency policy directly.
