# Rename Extracted Rust Checks To `g3rs-*` Config/Ast Surfaces

**Date:** 2026-04-06 11:27
**Scope:** extracted Rust check packages under `packages/`, direct app family bridge consumers under `apps/guardrail3/crates/app/rs/families/{fmt,toolchain,clippy,deny,cargo,deps,garde}`, related plans/readmes/todos/worklogs, and `apps/guardrail3/Cargo.lock`

## Summary
Renamed the extracted Rust check packages from `g3-*-(content|ast)-checks` to the new `g3rs-*-(config|ast)-checks` scheme and renamed the extracted rule surface IDs from plain family numbering to explicit `CONFIG` / `AST` rule IDs. Then repaired the direct app-family bridge fallout, especially stale assertion module filenames, until the package workspaces, the wired family tests, and the full `guardrail3` binary all compiled cleanly again.

## Context & Problem
The repo had already split a meaningful chunk of Rust checks into extracted packages, but the naming still reflected the older `g3-*` / `content checks` vocabulary. The new direction is a dedicated Guardrails RS split, with package names that clearly belong to the Rust-only line and rule IDs that show which surface they validate. The user explicitly wanted:

- package prefix `g3rs-`, not `g3rs-rs-`
- `content checks` renamed to `config checks` where the package is validating config files
- AST packages kept separate as AST packages
- the app split deferred until after the package/config/AST renames were stabilized and manually tested

The initial rename pass got the tree into an inconsistent middle state: package directories moved and many identifiers changed, but some rule IDs, app bridge references, and app assertion module filenames were still half-renamed. That state needed to be normalized and then tested through both the extracted packages and the old app families that still bridge into them.

## Decisions Made

### Use `g3rs-<family>-<surface>-checks` as the package naming scheme
- **Chose:** rename extracted packages to `g3rs-fmt-config-checks`, `g3rs-toolchain-config-checks`, `g3rs-clippy-config-checks`, `g3rs-deny-config-checks`, `g3rs-cargo-config-checks`, `g3rs-deps-config-checks`, `g3rs-garde-config-checks`, and `g3rs-garde-ast-checks`
- **Why:** `g3rs` already implies Guardrail3 Rust; adding another `rs` was redundant. The explicit surface suffixes make the package role legible without opening the crate.
- **Alternatives considered:**
  - `g3rs-rs-*` — rejected because it duplicates the Rust namespace in the package name
  - keeping `content checks` everywhere — rejected because it hides the config-vs-ast distinction that now matters architecturally

### Rename only the extracted surfaces and the direct old-app bridge points in this batch
- **Chose:** rename the already extracted package surfaces and the app-family bridge/assertion code that directly consumes them, instead of trying to complete the entire future `filetree` / `ast` / `config` rename for every remaining app-owned rule in one pass
- **Why:** this was the smallest batch that could be made internally consistent and tested end-to-end right now. The old app still exists as the integration host, so the bridge layer had to compile. The extracted packages were the current source of truth for the new naming.
- **Alternatives considered:**
  - rename every remaining app-owned rule ID immediately — rejected for this batch because it would greatly expand the blast radius before the package-level rename was stabilized
  - rename only the package directories and leave rule IDs untouched — rejected because the user explicitly wanted the rule surface to advertise `config` / `ast`

### Keep AST separate from config rather than flattening everything into config
- **Chose:** keep `g3rs-garde-ast-checks` and `RS-GARDE-AST-*` distinct from config packages and IDs
- **Why:** the user explicitly corrected the earlier bad idea of calling AST checks “config checks”. AST packages read and analyze source files; their input contract is materially different.
- **Alternatives considered:**
  - rename AST packages into config packages for uniformity — rejected because it is semantically wrong and would blur the actual boundary

### Repair bridge fallout mechanically via file/module alignment before changing semantics
- **Chose:** fix compile failures by aligning renamed module declarations with on-disk filenames in the old app assertions crates, rather than changing the bridge logic itself
- **Why:** the failures exposed by the test matrix were mostly stale filenames after bulk textual replacement. Those are mechanical consistency fixes, not semantic changes.
- **Alternatives considered:**
  - revert the module declarations back to old names — rejected because that would undo the surface rename instead of completing it
  - keep the compile breaks and rename later — rejected because the whole point of this batch was to get to a tested, working renamed state

## Architectural Notes
- The extracted package line is now named as the future Guardrails RS package surface:
  - `g3rs-*-config-checks`
  - `g3rs-*-ast-checks`
- The old `guardrail3` app remains the integration host for now. It still owns:
  - discovery
  - parse gating
  - structural/filetree ownership
  - app-side rules not yet extracted
- The rename batch deliberately did **not** finish the full future surface taxonomy for every app-owned rule. There are still many old app rule IDs that remain unsplit until the later app/filetree work.
- Direct app bridge consumers were updated enough for the renamed extracted packages to compile and run through the old app families:
  - `fmt`
  - `toolchain`
  - `clippy`
  - `deny`
  - `cargo`
  - `deps`
  - `garde`

## Information Sources
- `AGENTS.md` — repo instructions, worklog requirement, current Rust-only direction
- `.plans/2026-04-04-142819-family-checks-packages.md` — extracted package ledger and current package/app split
- `.worklogs/2026-04-05-221655-extract-garde-ast-checks.md` — latest extracted AST-package context
- `.worklogs/2026-04-05-211234-extract-garde-root-policy-content-checks.md` — latest extracted garde config-package context
- `.worklogs/2026-04-05-204032-package-todo-cleanup.md` — prior package follow-up inventory
- direct compile/test feedback from:
  - `cargo test --workspace --manifest-path packages/g3rs-fmt-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-toolchain-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-clippy-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-deny-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-cargo-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-deps-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-garde-config-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3rs-garde-ast-checks/Cargo.toml`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-{fmt,toolchain,clippy,deny,cargo,deps,garde} --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-run`
- subagent ledger from `Anscombe` for proposed family-wide `CONFIG` / `FILETREE` / `AST` rule renumbering; used as planning input, not as fully executed source of truth in this batch

## Open Questions / Future Considerations
- The repo is still in a mixed-ID state outside the extracted surfaces. Many app-owned rules still use old IDs like `RS-CLIPPY-24`, `RS-CARGO-14`, `RS-DEPS-01`, etc.
- `deps` tooling checks (`RS-DEPS-01..04`) do not fit cleanly into `CONFIG / FILETREE / AST`; they likely need an explicit `TOOLING` surface if the new naming scheme is extended across the remaining app rules.
- Some structural malformed-input sinks such as `RS-GARDE-10`, `RS-CARGO-14`, and `RS-DEPS-11` may eventually want a cleaner naming treatment than simply staying in the old numbering.
- A huge amount of planning/worklog prose changed mechanically in this batch. The text now reflects the new package names, but not every document has been reviewed for whether the new rule-ID vocabulary is semantically final.

## Key Files for Context
- `packages/g3rs-fmt-config-checks/Cargo.toml` — representative renamed config package manifest
- `packages/g3rs-garde-ast-checks/Cargo.toml` — representative renamed AST package manifest
- `packages/g3rs-clippy-config-checks/crates/types/src/lib.rs` — current renamed config package type surface
- `packages/g3rs-garde-ast-checks/crates/types/src/lib.rs` — current renamed AST package type surface
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/run.rs` — old app family bridge into renamed config package
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs` — old app family bridge into both renamed config and AST packages
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/lib.rs` — example of the app-assertions rename fallout that had to be repaired
- `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/advisories/mod.rs` — another example of stale assertion-module names after the rename
- `.plans/2026-04-04-142819-family-checks-packages.md` — package/app extraction ledger that now needs to stay aligned with the new names
- `.worklogs/2026-04-05-221655-extract-garde-ast-checks.md` — backstory for the garde AST package
- `.worklogs/2026-04-05-211234-extract-garde-root-policy-content-checks.md` — backstory for the garde config package

## Next Steps / Continuation Plan
1. Decide whether the next rename batch should extend the new surface IDs across the remaining old app-owned rules, or whether that should wait for the new minimal `g3rs` app. Read the subagent mapping and compare it against the actual current family surfaces before touching code.
2. If continuing the rename, handle one family at a time and separate `config`, `filetree`, `ast`, and possible `tooling` surfaces deliberately instead of repeating a global blind replacement. Start with `fmt`/`toolchain`/`cargo`, then `clippy`, `deny`, `deps`, and finally the rest.
3. Once the naming scheme is frozen, start the new minimal Rust-only app:
   - read only the extracted `g3rs-*` packages
   - own minimal discovery / parse gating
   - avoid reintroducing legality/topology machinery unless a package boundary strictly requires it
