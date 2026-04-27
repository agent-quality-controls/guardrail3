## Goal

Complete the `clippy` family under the package model by:

- migrating the live app-owned `RS-CLIPPY-*` rule surface into package lanes
- building the missing `filetree` lane
- extending the `config` lane with the remaining app-owned config rules
- removing the false public implication that a `source` lane exists
- hardening package tests so ingestion and bridge behavior are exact, not smoke-only

## Approach

1. Read the exact live app `clippy` rule bodies that the audit classified into:
   - filetree: `01`, `12`, `13`
   - config: `04`, `05`, `06`, `07`, `08`, `14`, `15`, `16`, `18`, `19`, `20`, `23`, `24`, `25`
2. Read the current package code and choose the smallest package-owned typed inputs needed for:
   - config content rules
   - policy-context-sensitive config rules
   - filetree coverage / placement / local-baseline rules
3. Add failing tests first for:
   - current stub-lane leakage in `g3rs-clippy-types` / `g3rs-clippy-ingestion`
   - weak exactness in current ingestion tests
   - one migrated config rule
   - one filetree rule
4. Replace the fake public source lane with the real package boundary:
   - no source lane unless app code proves one exists
5. Implement `g3rs-clippy-filetree-checks` and real `ingest_for_file_tree_checks(...)`.
6. Extend `g3rs-clippy-config-checks` and real config ingestion for the remaining config rules.
7. Re-run package tests and adversarial agents until they converge to no live findings.

## Key decisions

- Keep lane semantics strict:
  - filetree = coverage, placement, shadowing, input-failure surfacing
  - config = `clippy.toml`, `guardrail3.toml`, and `.cargo/config*` semantics
  - source = none unless the app proves a real source rule
- Treat the current `g3rs-clippy/max-struct-bools..08` package rules as meaningful package checks, not dead code.
  - They stay and the migrated app-owned rules will be added around them.
- Fix public API leakage first.
  - A package must not publicly expose unimplemented lanes as if they are real.

## Alternatives considered

- Keep source/filetree placeholders public until later.
  - Rejected: this leaks a false package contract.
- Delete the existing package config rules because they do not map 1:1 to app IDs.
  - Rejected: the agent review found them meaningful, lane-correct package checks.
- Migrate only the filetree lane first.
  - Rejected: the config lane still has major app-owned semantics and stale public boundaries.

## Files to modify

- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/**`
- `packages/rs/clippy/g3rs-clippy-ingestion/**`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/**` (new)
- `apps/guardrail3/Cargo.lock` if workspace dependency wiring changes require it
