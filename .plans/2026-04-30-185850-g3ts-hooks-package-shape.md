# Goal

Make the active G3TS hooks implementation follow the same package standards as the rest of G3TS:

- every `packages/ts/hooks/g3ts-hooks-*` package is a valid G3RS workspace root
- `lib.rs` files are facades only
- hook ingestion and source checks are split into owned modules instead of large implementation facades
- hook DTOs do not expose large public named-field structs
- the G3TS hooks runner still works against the landing app after the package cleanup

# Current Findings

`packages/ts/hooks` is active:

- `apps/guardrail3-ts/crates/logic/family-runner-hooks/src/run.rs` calls `g3ts_hooks_ingestion`, `g3ts_hooks_config_checks`, `g3ts_hooks_file_tree_checks`, and `g3ts_hooks_source_checks`
- `SupportedFamily::Hooks` exists
- Astro and style hook contracts are aggregated by the hooks runner

But the packages are not shaped like current guardrail packages:

- `g3ts-hooks-config-checks`, `g3ts-hooks-file-tree-checks`, `g3ts-hooks-ingestion`, `g3ts-hooks-source-checks`, `g3ts-hooks-types`, and `g3ts-hooks-contract-types` are plain single crates without root `[workspace]` policy/config files
- several `src/lib.rs` files contain implementation functions
- `g3ts-hooks-source-checks/src/lib.rs` is over the effective-line threshold
- `g3ts-hooks-ingestion/src/lib.rs` performs direct `std::fs` calls in a non-`fs` module
- hook input structs expose too many public named fields
- `g3ts-hooks-source-checks` still owns `src/lib_tests` from the pre-facade shape

# Approach

## 1. Package root hygiene

For every `packages/ts/hooks/g3ts-hooks-*` root:

- add `guardrail3-rs.toml`
- add `clippy.toml`
- add `deny.toml`
- add `rustfmt.toml`
- add `rust-toolchain.toml`
- make the root `Cargo.toml` declare `[workspace]` and workspace lints, matching the existing G3TS style package shape

Allowed dependency lists must be explicit per package. No catch-all waiver.

## 2. Facade cleanup

Move implementation out of `src/lib.rs`:

- `g3ts-hooks-config-checks/src/lib.rs` becomes a facade that re-exports `run::check`
- `g3ts-hooks-file-tree-checks/src/lib.rs` becomes a facade that re-exports `run::check`
- `g3ts-hooks-ingestion/src/lib.rs` becomes a facade that re-exports functions from owned modules
- `g3ts-hooks-source-checks/src/lib.rs` becomes a facade that re-exports `run::{check, check_effective}`

Owned modules:

- config checks: `run.rs`
- file-tree checks: `run.rs`
- ingestion: `run.rs`, `fs.rs`, `selection.rs`, `scripts.rs`, `tools.rs`, `app_roots.rs`
- source checks: `run.rs`, `commands.rs`, `fail_open.rs`, `results.rs`

## 3. DTO encapsulation

Update `g3ts-hooks-types` so large structs use private fields plus constructors/getters:

- `G3TsHooksConfigChecksInput`
- `G3TsHooksSourceChecksInput`
- `G3TsHooksFileTreeChecksInput`

Small fact structs can also be encapsulated if call-site churn is small:

- `G3TsHooksSelectedHookConfigFact`
- `G3TsHooksScriptFileFact`

Do not use builder types unless constructors become unreadable. This package is internal and explicit constructors are enough.

## 4. Tests

Keep existing behavior tests but move them onto the owning implementation module:

- source-check tests move from `src/lib_tests` to `src/run_tests`
- `run.rs` attaches the sidecar with `#[path = "run_tests/mod.rs"]`
- no `lib.rs` test sidecars

If G3RS requires shared assertion crates for these tests, add the minimal assertion crate rather than weakening the runtime tests.

## 5. Verification

Run:

- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-contract-types/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-types/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-ingestion/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-config-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-file-tree-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-source-checks/Cargo.toml --workspace --offline`
- `g3rs validate --path` for every hooks package root
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- reinstall local `g3ts`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family hooks --inventory`

# Non-goals

- Do not change app-side landing hooks in this change.
- Do not weaken hook trigger requirements.
- Do not implement style app wiring here.
