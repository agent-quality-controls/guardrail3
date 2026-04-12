# Fmt Family Migration

## Goal

Finish the `fmt` package migration based on the live app rule bodies:

- move `RS-FMT-01`, `RS-FMT-05`, and `RS-FMT-08` into a new filetree lane
- move `RS-FMT-07` into the config lane
- fix config ingestion so `RS-FMT-CONFIG-03` and `RS-FMT-CONFIG-04` keep their
  blocker behavior instead of failing too early in ingestion

The result should be:

- `g3rs-fmt-config-checks` owning all config rules
- `g3rs-fmt-filetree-checks` owning all filetree rules
- no `fmt` source lane

## Approach

1. Add failing tests first.
   - Filetree rule and pipeline tests for:
     - root config missing
     - nested override files
     - dual-file conflicts
     - root `.rustfmt.toml` acceptance
   - Config tests for:
     - ignore escape-hatch policy
     - nightly-key blocker behavior when toolchain is missing/invalid
     - edition blocker behavior when Cargo is missing/invalid
2. Expand shared `fmt` types.
   - Add lane-pure filetree input.
   - Replace config lane's hard-required parsed files with explicit root-file
     state so config rules can decide whether to emit blockers or stay quiet.
   - Add a minimal typed escape-hatch input for `RS-FMT-CONFIG-07`.
3. Implement `g3rs-fmt-filetree-checks`.
   - `RS-FMT-FILETREE-01`
   - `RS-FMT-FILETREE-05`
   - `RS-FMT-FILETREE-08`
4. Rewire `g3rs-fmt-ingestion`.
   - config ingestion should accept `rustfmt.toml` or `.rustfmt.toml` at root
   - filetree ingestion should collect root variants, nested config files, and
     same-directory dual-file conflicts
   - config ingestion should preserve missing/invalid Cargo and toolchain state
     instead of always erroring
5. Extend `g3rs-fmt-config-checks`.
   - add `RS-FMT-CONFIG-07`
   - move the old app blocker behavior for `03` and `04` into the package lane
6. Verify mechanically.
   - `cargo test --workspace -q` in `g3rs-fmt-config-checks`
   - `cargo test --workspace -q` in `g3rs-fmt-filetree-checks`
   - `cargo test --workspace -q` in `g3rs-fmt-ingestion`
   - `git diff --check`

## Key decisions

- `RS-FMT-07` is config, not filetree.
  - Why: it checks parsed `ignore` semantics plus typed escape-hatch metadata.
- Accept root `.rustfmt.toml` as a valid root config for migration parity.
  - Why: the live app facts treat it as the active root config when
    `rustfmt.toml` is absent.
- Keep filetree and config lanes separate.
  - Why: missing root config and nested override structure are filetree checks,
    while parsed rustfmt policy and escape-hatch reasoning are config checks.

## Alternatives considered

- Keep `RS-FMT-07` app-side.
  - Rejected because the package model should own the whole config rule surface.
- Keep config ingestion hard-failing on missing Cargo/toolchain.
  - Rejected because that suppresses rule-owned blocker findings for `03` and
    `04` and does not match the old rule semantics.

## Files to modify

- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/**`
- `packages/rs/fmt/g3rs-fmt-ingestion/**`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/**` (new)
