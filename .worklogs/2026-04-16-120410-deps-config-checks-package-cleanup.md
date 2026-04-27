Summary
- Cleaned `packages/rs/deps/g3rs-deps-config-checks` until `validate` returned `No findings.`
- Removed the local `types` wrapper, made the workspace unpublished, added the missing root policy files, and moved the sidecar tests onto the shared assertions crate.

Decisions made
- Deleted `crates/types` and used `g3rs-deps-types` directly because the local crate was only a thin wrapper and created fake boundary errors.
- Marked the root and child crates `publish = false` because this is an internal checks workspace and the release burden was fake.
- Kept the runtime and assertions crates as one-rule-per-module crates and documented that with `g3rs-arch/structural-split` waivers.
- Upgraded the local assertions macro surface to match the cleaned cargo package so sidecar tests could call shared assertions helpers instead of poking at `CheckResult`.

Key files for context
- `packages/rs/deps/g3rs-deps-config-checks/Cargo.toml`
- `packages/rs/deps/g3rs-deps-config-checks/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-config-checks/src/lib.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/assertions/src/common.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/assertions/src/lib.rs`

Next steps
- Continue to the next `packages/rs/deps/*` package.
- Stop only when a rule is genuinely contradictory or clearly outdated.
