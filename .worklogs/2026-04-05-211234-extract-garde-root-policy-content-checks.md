# Extract garde root-policy content checks

**Date:** 2026-04-05 21:12
**Scope:** `packages/g3-garde-content-checks`, `apps/guardrail3/crates/app/rs/families/garde`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Extracted the parsed-file root-policy slice of the `garde` family into a new `g3-garde-content-checks` package and wired the app family to call it for valid typed `Cargo.toml` and `clippy.toml` inputs. The app still owns garde applicability gating, malformed-input reporting, and the fallback warning path for missing or unparseable covering clippy configs.

## Context & Problem
After `fmt`, `toolchain`, `deny`, `clippy`, `cargo`, and `deps`, the next content-check candidate was `garde`. The family mixes two very different surfaces:

- root-policy checks over `Cargo.toml` and covering `clippy.toml`
- AST/source checks over Rust files

The package boundary rule in this repo is strict: content-check packages take parsed files only, not normalized AST fact bundles, `ProjectTree`, or orchestrator-derived helper subsets. That made a full-family move wrong. The correct extraction target was the root-policy slice only.

There was also a bridge-specific risk: the new package can only operate on typed parsed files, but the app family previously used raw TOML for the clippy-ban checks. If typed parsing failed after raw TOML parsing succeeded, the app could accidentally keep running raw checks and silently blur structural malformed-input ownership.

## Decisions Made

### Extract only the garde root-policy slice
- **Chose:** Move `RS-GARDE-01`, `RS-GARDE-02`, `RS-GARDE-03`, `RS-GARDE-04`, and `RS-GARDE-06` into `g3-garde-content-checks`.
- **Why:** Those rules operate on one root `Cargo.toml` and one covering `clippy.toml`, which fits the parsed-files-only package boundary cleanly.
- **Alternatives considered:**
  - Moving the whole family — rejected because `RS-GARDE-05`, `07`, `08`, `09`, `11`, `12`, `13`, and `14` currently depend on analyzed Rust source facts, not parsed config files.
  - Keeping even the root-policy rules app-side — rejected because they are genuine content checks over parsed files and fit the extraction architecture.

### Split package inputs by actual file surface
- **Chose:** Define two package inputs:
  - `G3GardeDependencyCheckInput { cargo_rel_path, cargo }`
  - `G3GardeClippyBanChecksInput { clippy_rel_path, clippy }`
- **Why:** `RS-GARDE-01` only needs parsed `Cargo.toml`; `RS-GARDE-02/03/04/06` only need parsed `clippy.toml`. This keeps the package contracts honest and avoids a mixed bag input that pretends every rule needs both files.
- **Alternatives considered:**
  - One giant garde package input — rejected because it couples unrelated rule surfaces and widens the public contract without need.
  - Subset helper types carrying derived booleans or ban lists — rejected because content packages must receive parsed files, not app-computed policy fragments.

### Keep missing and typed-invalid covering clippy handling in the app
- **Chose:** The app bridge only calls the package when `clippy_toml_parser::parse(...)` succeeds. Otherwise it falls back to the legacy app-side warning rules for `RS-GARDE-02/03/04/06`.
- **Why:** The package should never see malformed or absent inputs. The family still needs the pre-existing “cannot verify ...” warnings when the covering clippy config is missing or unparseable.
- **Alternatives considered:**
  - Making the package own malformed-input warnings — rejected because malformed-input ownership belongs in the app/orchestrator boundary.
  - Silently standing down on typed parse failure — rejected because that would hide the loss of verification entirely.

### Treat typed clippy parse failure as a fallback-warning path, not as raw-TOML success
- **Chose:** When raw TOML parsing succeeds but `clippy_toml_parser` rejects the file, `facts/clippy.rs` now drops `parsed` and keeps only a `parse_error`.
- **Why:** Without that, the old raw TOML rules would still run and could inventory or warn based on untyped content, which violates the extracted boundary and masks the real problem.
- **Alternatives considered:**
  - Keeping both raw and typed representations on typed parse failure — rejected because it lets the app accidentally continue content validation on a file the package considers invalid.
  - Emitting `RS-GARDE-10` for typed clippy parse failure — rejected for now because the family already uses rule-specific “cannot verify ...” warnings for the covering-clippy path, and that ownership was intentionally preserved.

## Architectural Notes
The `garde` family is now split this way:

- **Package `g3-garde-content-checks`:**
  - `RS-GARDE-01`
  - `RS-GARDE-02`
  - `RS-GARDE-03`
  - `RS-GARDE-04`
  - `RS-GARDE-06`
- **App family remains owner of:**
  - garde applicability gating from routed policy and source adoption
  - missing / unparseable covering clippy handling for `02/03/04/06`
  - `RS-GARDE-05`
  - `RS-GARDE-07`
  - `RS-GARDE-08`
  - `RS-GARDE-09`
  - `RS-GARDE-10`
  - `RS-GARDE-11`
  - `RS-GARDE-12`
  - `RS-GARDE-13`
  - `RS-GARDE-14`

This keeps the extracted package on the same architecture as the earlier content-check packages:

```text
app garde family
  -> discovers routed roots and covering configs
  -> parses files and owns malformed-input behavior
  -> calls package on typed parsed-file opportunities only
  -> keeps AST/source and structural rules app-side
```

## Information Sources
- `AGENTS.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`
- `apps/guardrail3/crates/app/rs/families/garde/README.md`
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/facts/clippy.rs`
- `packages/g3-garde-content-checks/README.md`
- `cargo test --workspace --manifest-path packages/g3-garde-content-checks/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
- manual validator attacks on:
  - valid baseline root
  - missing covering `clippy.toml`
  - typed-invalid covering `clippy.toml`
  - single missing core ban

## Open Questions / Future Considerations
- `RS-GARDE-10` currently remains the malformed-input sink for source and policy failures, while typed-invalid covering clippy still falls back to rule-specific warning ownership for `02/03/04/06`. If the family ever wants a single malformed-input owner for covering-clippy schema failures, that should be redesigned deliberately rather than smuggled through the package.
- The AST/source garde rules still need a clean package boundary if they are ever extracted. Right now their inputs are normalized analysis facts, not parser-file types.
- `packages/g3-garde-content-checks` inherits the same shared package-level dependency-rule debt as the older extracted packages, plus the tolerated runtime sibling-directory complexity finding.

## Key Files for Context
- `packages/g3-garde-content-checks/crates/types/src/lib.rs` — package input contracts for dependency and clippy-ban checks
- `packages/g3-garde-content-checks/crates/runtime/src/run.rs` — package entrypoints for the moved rule slice
- `packages/g3-garde-content-checks/crates/runtime/src/support.rs` — garde dependency detection and canonical ban sets
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs` — app/package bridge and fallback behavior
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/facts/clippy.rs` — covering-clippy discovery and typed parse fallback behavior
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/root_policy/rs_garde_02_core_method_bans/tests/typed_parse_fallback.rs` — family-layer regression test for typed-invalid covering clippy
- `.plans/2026-04-04-142819-family-checks-packages.md` — extraction ledger updated to the actual garde split
- `.worklogs/2026-04-05-203355-deps-local-path-parity.md` — previous extraction-pattern work that informed the same package/app boundary discipline

## Next Steps / Continuation Plan
1. Commit this garde root-policy extraction batch and keep `garde` recorded as a partial family extraction, not a full package migration.
2. If more `garde` hardening is needed, add direct app-family bridge coverage for the package path of `RS-GARDE-01` and one additional clippy-ban rule beyond `RS-GARDE-02`.
3. When returning to extraction sequencing, read `apps/guardrail3/crates/app/rs/families/test` and decide whether `test` or a deeper `garde` AST extraction design is the cleaner next family.
4. Do not move the AST/source garde rules until there is a parsed-file package boundary that does not require smuggling normalized analysis bundles into the package.
