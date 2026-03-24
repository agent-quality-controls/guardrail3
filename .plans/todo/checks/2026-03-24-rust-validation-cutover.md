# Rust Validation Cutover — New family runtime only

**Date:** 2026-03-24

## Purpose

`guardrail3 rs validate` must run the new Rust family checkers under `crates/app/rs/checks/**`.

The legacy validator under `crates/app/rs/validate/**` is not a compatibility layer, not a delegation layer, and not a fallback. It is removed from the runtime path.

The cutover is complete only when:
- `rs validate` emits only `RS-*` and `HOOK-*` findings
- CLI help and generated guide text describe only the new family model
- old `R*` / `R-ARCH-*` / `R-REL-*` rule inventory is gone from the user-facing validate path

## Runtime entrypoint

The public `guardrail3 rs validate` path is rooted in:
- `crates/main.rs`

That path currently reaches Rust validation through CLI validate helpers. Replace the Rust validate branch so it calls a new Rust validation runner built on:
- `crates/app/rs/checks/rs/*`
- `crates/app/rs/checks/hooks/*`

The runtime must not call:
- `crate::adapters::inbound::cli::validate::run(...)` for Rust validation
- `crate::app::rs::validate::run(...)`
- any old `app/rs/validate/*` family module

The old validator tree becomes dead code after cutover and should be deleted once the new runner is wired and tested.

The old Rust-specific hook validation path under:
- `crates/app/hooks/validate.rs`

is also removed from the Rust validation runtime contract.

## Family set

The Rust validation family set is:
- `fmt`
- `toolchain`
- `clippy`
- `deny`
- `cargo`
- `code`
- `hexarch`
- `deps`
- `garde`
- `test`
- `release`
- `hooks-shared`
- `hooks-rs`

`libarch` is not part of the runtime family set until there is a live checker module under `crates/app/rs/checks/rs/libarch/`.

## CLI contract

`guardrail3 rs validate` becomes a family-based command.

### Selection

Selection uses repeatable family flags:

```text
guardrail3 rs validate . --family fmt --family clippy
```

If no `--family` flags are provided, all implemented Rust families are selected.

The family names are:
- `fmt`
- `toolchain`
- `clippy`
- `deny`
- `cargo`
- `code`
- `hexarch`
- `deps`
- `garde`
- `test`
- `release`
- `hooks-shared`
- `hooks-rs`

CLI uses kebab-case family names.

The old grouped flags are removed:
- `--code`
- `--architecture`
- `--garde`
- `--tests`
- `--release`

If `hooks-rs` is selected, the runner must also execute `hooks-shared`.
`hooks-rs` is not a complete hook-validation mode by itself.

### Scope flags

The existing scope flags remain:
- `--staged`
- `--dirty`
- `--commits N`
- `--files ...`

Their meaning is exact:

- source-file inputs may be narrowed by scope
- root/config/policy/architecture inputs are not narrowed by file scope

The runner must not classify whole families as “scoped” or “unscoped” and then skip root-owned rules by accident.

Concretely:

- `code`, `garde`, and `test` apply scope only to their source-file analysis surfaces
- their root-owned / tool-owned / config-owned rules still run in full
- non-source families ignore file scope and still run in full when selected:
  - `fmt`
  - `toolchain`
  - `clippy`
  - `deny`
  - `cargo`
  - `hexarch`
  - `deps`
  - `release`
  - `hooks-shared`
  - `hooks-rs`

This is intentional. File scope narrows source scanning only. It does not soften root/policy/architecture guardrails.

The cutover is not complete while `code`, `garde`, and `test` expose only unscoped whole-tree source analysis.
They must either:
- accept scoped source inputs from the runner
- or accept scope data and apply source filtering internally while still running root-owned rules in full

### Thorough flag

`--thorough` remains.

Its effect is family-local:
- only `release` consumes it
- all other families ignore it

### Hooks commands

`rs hooks-install` remains a generation/install command.

`rs hooks-validate` is removed from the public validation contract.

Hook validation lives under:
- `guardrail3 rs validate --family hooks-shared --family hooks-rs`

There is one Rust validation entrypoint, not a separate hook-validator stack.

## Config contract

`guardrail3.toml` Rust check toggles become family-based.

### Global Rust family config

Use:

```toml
[rust.checks]
fmt = true
toolchain = true
clippy = true
deny = true
cargo = true
code = true
hexarch = true
deps = true
garde = true
test = true
release = true
hooks_shared = true
hooks_rs = true
```

Config uses snake_case family keys.

### Per-root overrides

Per-app / per-package Rust check overrides use the same family names:

```toml
[rust.apps.guardrail3.checks]
hexarch = true
garde = false

[rust.packages.checks]
code = true
test = true
```

### Removed config keys

The old grouped keys are removed:
- `architecture`
- `tests`
- `hooks`

Their replacements are:
- `architecture` -> `hexarch`
- `tests` -> `test`
- `hooks` -> `hooks_shared` and `hooks_rs`

`garde` and `release` keep their names.
`code` is a new explicit family key.

There is no compatibility parsing for the removed grouped keys.

## Selection resolution

The runner resolves the active family set in this order:

1. start from all implemented families
2. apply config toggles at the relevant ownership scope for each family
3. if CLI `--family` is present, intersect with those requested families
4. close the selected set over family dependencies

Current dependency closure:
- `hooks-rs` implies `hooks-shared`

CLI narrows. Config enables/disables. There is no grouped-domain expansion step.

## Ownership model

The runtime contract follows the family plans.

### Validation-root families
- `fmt`
- `toolchain`
- `hooks-shared`

### Policy-root families
- `clippy`
- `deny`
- `cargo`

### Mixed source + root families
- `code`
- `garde`
- `test`

### Mixed validation-root + package + Rust-root family
- `deps`

### Mixed validation-root + publishable-crate + release-edge family
- `release`

### Service-architecture family
- `hexarch`

### Hook dependency
- `hooks-rs` is layered on `hooks-shared`
- selecting `hooks-rs` alone is not a complete validation mode

Each family owns its own applicability rules. The top-level runner selects families; it does not re-implement family semantics.

## Orchestration pipeline

The new Rust runner must:

1. build one `ProjectTree`
2. construct one `FileSystem`
3. construct one `ToolChecker`
4. resolve the selected family set
5. call each family orchestrator directly with the inputs its public contract requires
6. collect family-local results into one `Report`

The runner must not:
- synthesize old grouped sections such as “Config files” or “Source code scan”
- merge multiple Rust families into one section
- map new family findings back to old `R*` IDs

## Report contract

Report sections are one-to-one with families.

The section names are:
- `Rust / fmt`
- `Rust / toolchain`
- `Rust / clippy`
- `Rust / deny`
- `Rust / cargo`
- `Rust / code`
- `Rust / hexarch`
- `Rust / deps`
- `Rust / garde`
- `Rust / test`
- `Rust / release`
- `Rust / hooks-shared`
- `Rust / hooks-rs`

Every finding in those sections must already have its family rule ID.

The validate path must not emit:
- `R1..R58`
- `R-ARCH-*`
- `R-REL-*`
- other old validator IDs

## Domain model changes

The coarse Rust runtime category model is removed.

Remove:
- `RustCheckCategories`

Replace it with a family-selection type, for example:
- `RustValidationFamily`
- `RustValidationSelection`

`ValidateDomains` remains only if still needed outside Rust validate. It must not drive Rust family routing.

The concrete grouped-schema surfaces that must be rewritten are:
- `crates/adapters/inbound/cli/cli.rs`
- `crates/adapters/inbound/cli/validate.rs`
- `crates/main.rs`
- `crates/domain/config/types.rs`
- `crates/domain/report/mod.rs`

`ValidateArgs` is currently shared by Rust and TypeScript validate/hook commands.
The cutover therefore requires separate validate-argument types so the Rust-only `--family` surface does not leak onto TypeScript commands.

## Config failure behavior

Rust validate must fail closed on invalid Rust family-selection config.

That includes:
- malformed `guardrail3.toml` when Rust family selection depends on it
- unknown keys under `[rust.checks]`
- removed grouped Rust keys under `[rust.checks]`

The runtime must not silently parse-fail to defaults and run an unintended family set.

## Help and generated docs

The following user-facing surfaces must switch to the new family inventory:
- `crates/adapters/inbound/cli/help_gen.rs`
- `crates/adapters/inbound/cli/init.rs`
- `crates/domain/modules/guide.rs`
- `apps/guardrail3/tests/**` Rust validate UX/config snapshots and CLI tests
- CLI help text for `rs validate`
- generated guide/help output that currently lists old `R*` inventories

They must describe:
- the family list
- `--family`
- family-level config keys
- the absence of grouped Rust validation flags

## Acceptance criteria

The cutover is correct only when all of the following are true:

1. `guardrail3 rs validate` runs only the new family modules.
2. The runtime does not call `app::rs::validate::run`.
3. `guardrail3 rs validate . --family hexarch` on this repo reports the workspace-boundary issue through `RS-HEXARCH-*`, not `R-ARCH-*`.
4. Report output contains only family sections and new rule IDs.
5. CLI help and generated guide text no longer advertise old grouped Rust validate flags or old rule IDs.
6. `rs init` scaffolds family-based `[rust.checks]` keys instead of grouped domain keys.
7. `rs hooks-validate` is gone as a separate validation path.
8. Rust validate tests and golden snapshots no longer lock in old grouped flags, grouped config keys, or old rule IDs.
9. The public `guardrail3 rs validate` path in `main.rs` no longer routes Rust validation through the old CLI validate helper / legacy Rust validator stack.

## Implementation order inside the cutover

1. Add the new Rust family-selection type and `--family` CLI contract.
2. Add a new Rust validation runner over `app/rs/checks/**`.
3. Switch CLI Rust validate to that runner.
4. Switch report sections to family-based names.
5. Update config parsing and docs to family-level keys.
6. Remove old grouped Rust validate flags.
7. Remove `rs hooks-validate`.
8. Delete the legacy runtime path under `app/rs/validate/**`.
