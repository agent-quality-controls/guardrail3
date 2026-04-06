# Rename Parser Packages And Add Cargo TOML Parser

**Date:** 2026-04-04 21:55
**Scope:** `packages/rustfmt-toml*`, `packages/rust-toolchain-toml*`, `packages/clippy-toml*`, `packages/deny-toml*`, `packages/nextest-toml*`, `packages/mutants-toml*`, `packages/cargo-config-toml*`, `packages/cargo-toml-parser/**`, `.plans/2026-04-03-210000-config-parsers.md`, `.plans/2026-04-04-142741-new-parsers.md`, `.plans/2026-04-04-142819-family-checks-packages.md`, `.plans/by_family/rs/fmt.md`

## Summary
Renamed the public parser package layer to `*-parser` so the published facade names match their purpose and match the already-existing parser-oriented internal structure. Added a first-party `cargo-toml-parser` package with a file-local typed `Cargo.toml` model instead of depending on `cargo_toml::Manifest`.

## Context & Problem
The parser work had drifted into an awkward naming split: top-level public packages were named like plain config files (`rustfmt-toml`, `deny-toml`, etc.), while the internal runtime crates already carried parser-oriented names. At the same time, extraction planning still pointed at `cargo_toml::Manifest`, which does more than file-local parsing and brings workspace-inheritance resolution behavior that does not fit the content-check package boundary.

The active family-extraction work needs a stable parser package layer:
- public package names should clearly say “parser”
- internal crate names should not collide with the facade crate names
- Cargo.toml should have a first-party typed parser package consistent with the other parser packages

## Decisions Made

### Rename Public Parser Facades To `*-parser`
- **Chose:** rename the top-level parser packages to `rustfmt-toml-parser`, `rust-toolchain-toml-parser`, `clippy-toml-parser`, `deny-toml-parser`, `nextest-toml-parser`, `mutants-toml-parser`, and `cargo-config-toml-parser`
- **Why:** the public package name should describe the artifact users consume. These are parser packages, not general-purpose config packages.
- **Alternatives considered:**
  - Keep unsuffixed public package names — rejected because it preserved the facade/runtime naming mismatch and kept extraction docs ambiguous.
  - Rename only directories but leave Cargo package names unchanged — rejected because that would keep the public API confusing and leave crates.io naming inconsistent with repo structure.

### Disambiguate Internal Crate Names With `-runtime`
- **Chose:** keep the facade crate as `*-parser`, rename the implementation crate to `*-parser-runtime`, keep typed models as `*-parser-types`, and rename assertion helpers to `*-parser-runtime-assertions`
- **Why:** once the facade crate uses the `*-parser` name, the old runtime crate names collide semantically. The `-runtime` suffix preserves the existing package pattern used elsewhere in the repo (`runtime`, `types`, `assertions`) while making Cargo package names unambiguous.
- **Alternatives considered:**
  - Reuse the same crate name for facade and runtime — rejected because Cargo package naming and re-export structure become harder to reason about and docs/readmes become misleading.
  - Rename the internal crate to `*-core` — rejected because the rest of the repo already uses `runtime` for implementation crates.

### Build A First-Party `cargo-toml-parser`
- **Chose:** add `packages/cargo-toml-parser` with a file-local typed `CargoToml` model that captures the manifest structure guardrail3 actually needs
- **Why:** family extraction needs a typed Cargo manifest parser that behaves like the other parser packages: parse one file, preserve unknown keys, do not crawl the filesystem, and do not resolve workspace inheritance behind the caller’s back.
- **Alternatives considered:**
  - Use `cargo_toml::Manifest` directly — rejected because it mixes file parsing with workspace-inheritance completion semantics and is not a clean fit for extracted content-check inputs.
  - Leave Cargo.toml untyped and pass `toml::Value` — rejected because it repeats the same boundary mistake already identified in earlier extraction work.

## Architectural Notes
The resulting parser package pattern is now:

- public facade crate: `*-parser`
- runtime crate: `*-parser-runtime`
- typed model crate: `*-parser-types`
- assertion helpers crate: `*-parser-runtime-assertions`

`cargo-toml-parser` intentionally models file-local `Cargo.toml` structure only. It does not attempt to resolve workspace inheritance or compute “effective” values from parent manifests. That resolution, if needed for a family, belongs in the app/orchestrator layer rather than inside the parser package or inside extracted content-check packages.

The active extraction planning was updated to refer to the renamed parser crates. The `fmt` family planning note also keeps the rule that the orchestrator decides which files are authoritative and content packages only consume typed parsed inputs.

## Information Sources
- `packages/rustfmt-toml-parser/**` — reference parser package shape after rename
- `packages/rust-toolchain-toml-parser/**` — reference parser package shape for nested typed sections
- `packages/cargo-config-toml-parser/**` — reference pattern for facade/runtime/types/assertions split
- `packages/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs` — new first-party Cargo.toml typed model
- `.plans/2026-04-03-210000-config-parsers.md` — original standalone parser design note
- `.plans/2026-04-04-142819-family-checks-packages.md` — live family extraction planning updated to the new parser names
- local inspection of `cargo_toml` crate docs/source (`cargo info cargo_toml`, local registry source) — used to confirm workspace-inheritance behavior and why it is a poor boundary fit here

## Open Questions / Future Considerations
- `cargo-toml-parser` currently models file-local manifest structure only. If a rule genuinely needs effective workspace-inherited values, the app will need an explicit resolution step above this parser.
- Older planning files still mention `cargo_toml::Manifest` in historical sections. They were not all normalized in this pass.
- The worktree still contains unrelated `fmt` runtime/test edits from an earlier overreach. Those are intentionally not part of this commit.

## Key Files for Context
- `packages/cargo-toml-parser/Cargo.toml` — public package manifest for the new Cargo.toml parser facade
- `packages/cargo-toml-parser/src/lib.rs` — public re-export surface for `cargo-toml-parser`
- `packages/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs` — typed file-local Cargo.toml model
- `packages/rustfmt-toml-parser/Cargo.toml` — canonical example of the renamed public parser package surface
- `packages/rustfmt-toml-parser/crates/parser/runtime/Cargo.toml` — canonical example of the `*-parser-runtime` internal package naming
- `.plans/2026-04-04-142819-family-checks-packages.md` — current extraction plan with renamed parser crate references
- `.plans/by_family/rs/fmt.md` — current note that the orchestrator chooses authoritative file inputs for `fmt`
- `.worklogs/2026-04-04-193852-toolchain-content-checks-rewire.md` — recent extraction work that depends on stable parser/content-check package boundaries

## Next Steps / Continuation Plan
1. Rewire family extraction plans and implementation stubs that still mention `cargo_toml::Manifest` so they refer to `cargo-toml-parser::CargoToml` or to orchestrator-derived typed facts where appropriate.
2. Revisit `g3rs-toolchain-config-checks` and replace its current raw `toml::Value` toolchain input with typed `rust-toolchain-toml-parser::RustToolchainToml` so the content-check boundary matches the corrected parser package standard.
3. Clean the unrelated `fmt` runtime/test edits from the worktree before the next extraction commit so the next changeset can stay focused on `g3rs-fmt-config-checks`.
