# Build And Reshape Deps Content Checks

**Date:** 2026-04-05 19:14
**Scope:** `.plans/2026-04-04-142819-family-checks-packages.md`, `packages/g3rs-deps-config-checks`

## Summary
Added the new `g3rs-deps-config-checks` package for the `deps` family content rules and then reshaped its internals to match the extracted-package architecture more closely. The package now owns `RS-DEPS-CONFIG-01`, `06`, `07`, `08`, and `12`, uses full parsed files as input, and follows the rule-directory plus sidecar-test pattern instead of the first flat-file draft.

## Context & Problem
The next family in the extraction sequence was `deps`. The intended split was:
- app keeps environment, lockfile, gitignore, and parse-failure rules
- package owns pure dependency-policy content checks

The first package draft got the core boundary right at the data level:
- parsed workspace `Cargo.toml`
- parsed crate `Cargo.toml`
- parsed workspace `guardrail3-rs.toml`

But structurally it drifted from the extracted-package specimen. In particular:
- runtime rules were flat files instead of rule directories
- test sidecars were wired through `#[path = "..._tests/mod.rs"]`
- the assertions crate was effectively a stub
- `types` exported directly from `lib.rs` instead of through a facade module

The user explicitly asked for an architecture check against the existing extracted packages, especially `g3rs-toolchain-config-checks`, and requested that the package-specific structural damage be fixed while skipping two known issues for now:
- oversized public input
- runtime sibling-directory complexity threshold

## Decisions Made

### Keep the deps content input as full parsed files
- **Chose:** define `G3DepsContentChecksInput` as one struct containing:
  - workspace `Cargo.toml`
  - crate `Cargo.toml`
  - workspace `guardrail3-rs.toml`
- **Why:** `RS-DEPS-CONFIG-01..08` need both the crate manifest and workspace manifest to resolve dependency identity correctly, and the user required that content packages receive parsed files rather than scoped helper subsets.
- **Alternatives considered:**
  - One giant workspace bag with derived policy state — rejected because it smuggles orchestrator logic into the package.
  - One-file-only input — rejected because the dependency allowlist rules need workspace resolution for `workspace = true` and internal/external path classification.

### Limit the extracted rule set to pure content checks
- **Chose:** move `RS-DEPS-CONFIG-01`, `06`, `07`, `08`, and `12` into the package, and keep `RS-DEPS-01..04`, `09`, `10`, and `11` in the app.
- **Why:** tool-installation, lockfile presence, gitignore handling, and malformed-input reporting are not content checks.
- **Alternatives considered:**
  - Move all deps rules together — rejected because it would collapse structural/orchestrator concerns into the package.
  - Extract only `RS-DEPS-CONFIG-05` first — rejected because the workspace-level policy simplification now makes `05..08` viable without recreating the old crate-scoped legacy config model.

### Reshape the runtime crate to the rule-directory pattern
- **Chose:** convert each deps rule from a flat file to a directory with `mod.rs`, `rule.rs`, and `tests/mod.rs`.
- **Why:** this matches the extracted-family pattern better than the first draft and removes the `#[path = "..._tests/mod.rs"]` bypass that the local arch rules flagged.
- **Alternatives considered:**
  - Keep flat rule files with separate `*_tests` directories — rejected because it was the main package-specific structural drift versus the existing extracted packages.
  - Collapse multiple deps rules into grouped production files — rejected because this repo’s extracted-package pattern is still one rule per file/directory.

### Make the assertions and types crates real facades
- **Chose:** add a real assertions surface with `common.rs` plus per-rule assertion modules, and move the types export behind `types/src/input.rs`.
- **Why:** the initial assertions crate was just a stub, and the direct parser imports in `types/src/lib.rs` were being treated as non-facade content by the arch checker.
- **Alternatives considered:**
  - Delete the assertions crate entirely — rejected because the extracted packages are supposed to have a package-local assertion layer.
  - Leave `types/src/lib.rs` as the concrete type definition — rejected because it weakened the crate-facade pattern already used elsewhere.

### Skip two known issues for now
- **Chose:** leave the six-field public input and the five-rule runtime sibling-directory count untouched in this pass.
- **Why:** the user explicitly said to skip the first two complaints after the architecture review.
- **Alternatives considered:**
  - Split the input immediately — rejected because the user told us not to solve that in this pass.
  - Split the runtime into sub-crates immediately — rejected because the user told us to skip the first two issues and fix the rest first.

## Architectural Notes
The resulting `deps` package follows the extracted-package boundary used across the repo:

```text
app deps family
  -> discovers workspace + crate + workspace policy files
  -> parses them
  -> emits structural / malformed-input findings
  -> calls g3rs-deps-config-checks with parsed files
  -> package runs pure dependency-policy content rules
```

Important current state:
- the package is built and tested
- the app family is not wired yet
- the package uses the current workspace policy model from `guardrail3-rs.toml`, not the old per-crate legacy `guardrail3.toml` shape

After the structural cleanup, the remaining validator findings are mostly either:
- the two intentionally skipped issues
- the same cross-package dependency/facade rule debt already present in older extracted packages like `g3rs-toolchain-config-checks`

## Information Sources
- `AGENTS.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`
- `packages/g3rs-toolchain-config-checks` — extracted-package specimen used for structural comparison
- `apps/guardrail3/crates/app/rs/families/deps/README.md`
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/dependency_entries.rs`
- `packages/g3rs-deps-config-checks`
- `cargo test --workspace --manifest-path packages/g3rs-deps-config-checks/Cargo.toml`
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate packages/g3rs-deps-config-checks --family arch --family code --format json`

## Open Questions / Future Considerations
- The package still trips the local complexity threshold because the runtime crate now has five rule directories.
- The public input still exposes six `pub` fields and trips `RS-CODE-31`.
- Cross-package dependency rules still flag the parser and shared-check-types dependencies here, just as they do in the older extracted packages. That broader package-boundary rule set likely needs a repo-wide decision instead of another one-off workaround.
- The app `deps` family still needs to migrate from legacy behavior and actually call this package.

## Key Files for Context
- `.plans/2026-04-04-142819-family-checks-packages.md` — source-of-truth rule split for the deps extraction
- `packages/g3rs-deps-config-checks/crates/types/src/input.rs` — current parsed-file contract for the deps package
- `packages/g3rs-deps-config-checks/crates/runtime/src/run.rs` — package entrypoint and moved rule wiring
- `packages/g3rs-deps-config-checks/crates/runtime/src/support.rs` — shared dependency resolution logic
- `packages/g3rs-deps-config-checks/crates/assertions/src/common.rs` — package-local assertion helpers
- `packages/g3rs-toolchain-config-checks/crates/runtime/src` — comparison specimen used during the architecture cleanup
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md` — previous extracted-package and parser-contract context
- `.worklogs/2026-04-05-165315-deny-content-package-tests.md` — recent package-local test-surface precedent

## Next Steps / Continuation Plan
1. Wire the app `deps` family to parse the authoritative workspace manifest, crate manifest, and workspace `guardrail3-rs.toml`, then call `g3rs-deps-config-checks` for `RS-DEPS-CONFIG-01`, `06`, `07`, `08`, and `12`.
2. Keep `RS-DEPS-01..04`, `09`, `10`, and `11` in the app and make the app own malformed-input reporting explicitly before package calls.
3. After the app wiring is in place, add bridge smoke tests at the family layer similar to the clippy and cargo migrations.
4. Revisit the skipped issues only after wiring:
   1. decide whether the six-field input should be split or accessorized
   2. decide whether the runtime crate should remain one crate despite the five-rule threshold
5. Treat the remaining cross-package dependency/facade complaints as a shared extracted-package problem, not a deps-only fixup.
