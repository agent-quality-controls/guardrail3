## Goal

Capture what is still not done in `guardrail3-rs` after the zero-error validate push, with emphasis on the missing generation surface and the latest architectural ideas around generated policy files.

This is a planning snapshot, not a final decision document.

## Current state

- The active Rust app exists at `apps/guardrail3-rs`.
- The active CLI currently exposes only `Validate`.
- All current Rust families are wired into `validate` and the repo is at zero errors under `guardrail3-rs validate`.
- The missing product surface is the write path:
  - generate
  - check / dry-run / diff-style write planning
  - init/bootstrap if kept
  - generated artifact ownership and write policy

Grounding files:

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`

## What is still to be done

### 1. Add the active Rust generation surface

The current app can validate, but it cannot yet produce or reconcile the files it validates.

Missing deliverables:

- a `Generate` command in the active CLI
- a typed generation request / result model
- a Rust write-plan owner
- dry-run / write / check mode behavior
- rendered reporting for generated changes
- generated-file write policy:
  - overwrite
  - scaffold-once
  - preserve custom content where allowed

### 2. Define one canonical Rust write set

We need one explicit owner for the Rust write set used by:

- generate
- check / dry-run
- hooks-install if it remains part of Rust-owned output

This matches the older planning direction that called for:

- crate: `crates/app/rs/generate`
- owned artifact file: `src/owned_artifacts.rs`

Relevant prior plan:

- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`

### 3. Decide file ownership for generated policy files

This is the main unresolved architecture question.

The latest ideas are:

- each policy file should have one composer
- the composer should live with the family that naturally owns that file surface
- other families may contribute policy fragments to that file
- generation should be file-centric, not validator-family-centric

Examples:

- `clippy.toml`
  - likely owned by a composer under the `clippy` family
  - `garde` may contribute additional bans
- `deny.toml`
  - likely owned by a composer under `deny`
- `rustfmt.toml`
  - likely owned by a composer under `fmt`
- `rust-toolchain.toml`
  - likely owned by a composer under `toolchain`

This is a latest idea, not a final decision.

### 4. Decide how `guardrail3-rs.toml` should be owned

This file is cross-family by nature, so it does not fit the simple "family owns its own file" shape as cleanly.

Latest idea:

- introduce a dedicated file-owner surface for `guardrail3-rs.toml`
- possibly as a `guardrail3` / Rust-policy family
- that owner handles:
  - file parsing
  - file composition
  - generic file-level structure
- consumer families continue to own the semantics of their own fields inside that file

This is also a latest idea, not a final decision.

### 5. Decide config-file handoff boundaries for generation and validation

The latest architectural preference is:

- parse once
- type once
- pass the whole typed config file across family boundaries
- do not centrally slice config files into per-family mini-views
- let each family read its own fields from the typed file

This matters especially for:

- `guardrail3-rs.toml`
- any future generated config composer boundaries

This should remain the target unless a concrete contradiction appears.

### 6. Add app-level types and ports for generation

The current `app-types` surface is validate-only.

We still need typed boundaries for generation, likely including:

- `GenerateRequest`
- `GenerateMode`
  - `Check`
  - `DryRun`
  - `Write`
- `GeneratedArtifact`
- `GeneratedChange`
- `GenerateOutcome`

Likely ports:

- planner for selecting artifacts
- writer for applying or diffing artifacts
- renderer for CLI output

### 7. Build the orchestration layer inside the active app

The app should own command orchestration, not family-local file content.

Latest likely shape:

- logic package for generation command
- outbound package for generated-file writing / diffing / reporting

Candidate app packages:

- `apps/guardrail3-rs/crates/logic/generate-command`
- `apps/guardrail3-rs/crates/io/outbound/generated-files`

This is a likely next shape, not a final crate map.

### 8. Wire generated output to validators without drift

We need generator and validator parity so they do not silently diverge.

Required outcome:

- canonical generated baselines and validator expectations come from one source of truth
- family validators do not drift from generated file content
- file composers and validators share the same canonical managed key expectations

### 9. Restore coherent user-facing command surface

The active Rust app still needs a coherent write-facing command surface.

Open items:

- whether we expose:
  - `generate`
  - `check`
  - `init`
  - `hooks-install`
- whether `diff` exists as its own command or only as `generate --dry-run`
- where help / guide / command text should live so command docs do not drift from reality

The current older planning direction favored one explicit owner for user-facing command text.

## Latest ideas to preserve explicitly

These are current working ideas, not final decisions.

### A. File composer + policy contributors

Current best idea:

- one generated file -> one composer
- the composer owns:
  - parse / preserve / merge / render
  - stable ordering
  - conflict detection
- other families contribute typed policy fragments to the file

Example:

- `clippy.toml` composer under `clippy`
- `garde` contributes only the `clippy.toml` fragments it legitimately owns

### B. Composer belongs with the natural file family

Current best idea:

- put a file composer with the family that naturally owns that file surface
- do not put all generation into one giant app-local monolith
- do not force validator-family ownership to mean full-file ownership

### C. `guardrail3-rs.toml` may need a dedicated owner

Current best idea:

- `guardrail3-rs.toml` is special enough to deserve its own file-owner surface
- possibly a dedicated `guardrail3` / Rust-policy family
- not because every family section moves there semantically
- but because one surface should own the file as a file

### D. Whole typed files across family boundaries

Current best idea:

- shared config files should cross family boundaries as whole typed files
- not as centrally sliced per-family mini-structs
- each family still produces minimal local rule inputs internally

## Non-final decisions / open questions

These remain open and should be resolved before implementation starts.

1. Does `guardrail3-rs.toml` get its own family, or only its own composer package?
2. Which generated files are fully managed versus scaffold-once?
3. Do we preserve user-owned keys inside generated files, and if so by what merge rules?
4. Is `diff` a standalone CLI command or only a mode of `generate`?
5. Does Rust hook installation belong to the same Rust write set owner?
6. What is the smallest good first vertical slice:
   - `rustfmt.toml`
   - `clippy.toml`
   - `guardrail3-rs.toml`
   - or another file

## Recommended implementation order

This is an execution proposal, not a locked sequence.

1. Freeze the generation boundary types in `app-types`.
2. Add the active CLI `Generate` command.
3. Create one Rust write-set owner in the app.
4. Implement one file composer end-to-end as the specimen.
   - likely `rustfmt.toml` or `clippy.toml`
5. Add dry-run / write / check reporting around that one specimen.
6. Expand to the remaining policy files.
7. Decide and implement `guardrail3-rs.toml` ownership after the simpler file surfaces prove out.

## Files to modify later

Not changing these now, but they are the likely next anchors:

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/types/app-types/src/traits.rs`
- new generation command package under `apps/guardrail3-rs/crates/logic/`
- new generated-file outbound package under `apps/guardrail3-rs/crates/io/outbound/`
- future file composers under the relevant family package groups
