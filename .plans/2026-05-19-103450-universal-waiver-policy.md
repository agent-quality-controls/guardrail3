# Universal Waiver Policy

## Goal

Make waiver parsing and matching a shared guardrails contract used by both G3RS and G3TS.

The end state is:

- `guardrail3-rs.toml` and `guardrail3-ts.toml` keep separate typed root schemas.
- Shared TOML concepts live in one shared parser-side package.
- `WaiverConfig` is not owned by `g3rs-toml-parser`.
- G3TS accepts the same `[[waivers]]` syntax as G3RS.
- RS families that already support waivers use the same shared waiver type and matcher.
- `g3rs-deps/direct-dependency-cap` supports a visible waiver warning instead of an unwaivable error.

## Approach

1. Add `packages/parsers/g3-guardrail-toml-types`.
   - This package owns shared config types and waiver matching.
   - It exports:
     - `WaiverConfig`
     - `WaiverMatch`
     - `WaiverReason`
     - `find_waiver_reason`
     - `has_waiver`
   - Matching is exact on `rule`, `file`, and `selector`.
   - Empty or whitespace-only `reason` does not match.

2. Update `g3rs-toml-parser`.
   - Remove its local `WaiverConfig`.
   - Import and re-export shared `WaiverConfig`.
   - Keep `Guardrail3RsToml` and `RustChecksConfig` local.

3. Update `g3ts-toml-parser`.
   - Add `waivers: Vec<WaiverConfig>` to `Guardrail3TsToml`.
   - Import and re-export shared `WaiverConfig`.
   - Keep `Guardrail3TsToml`, `TsChecksConfig`, and all TS policy structs local.

4. Replace RS family-local waiver structs.
   - Replace `G3RsCargoWaiver`, `G3RsClippyWaiver`, `G3RsCodeWaiver`, `G3RsFmtWaiver`, and `G3RsGardeWaiver` with shared `WaiverConfig`.
   - Keep `arch` and `apparch` on shared `WaiverConfig`, but update imports to the new shared package instead of `g3rs-toml-parser`.

5. Replace RS family-local matching.
   - Cargo uses shared `find_waiver_reason`.
   - Clippy uses shared `has_waiver`.
   - Code uses shared `has_waiver`.
   - Fmt uses shared `find_waiver_reason`.
   - Garde uses shared `find_waiver_reason`.
   - Arch uses shared `has_waiver`.
   - Apparch uses shared `find_waiver_reason`.

6. Wire deps waiver support.
   - `G3RsDepsConfigChecksInput` receives `waivers: Vec<WaiverConfig>`.
   - `g3rs-deps-ingestion` passes parsed root waivers to every crate-policy input.
   - `g3rs-deps/direct-dependency-cap` uses selector `unique-direct-dependency-count`.
   - If over cap and no waiver matches, emit `Error`.
   - If over cap and waiver matches, emit `Warn`, not silence.
   - The warning includes current count, cap, and waiver reason.

7. Add fixtures and goldens.
   - Add an RS deps fixture proving the missing waiver errors and matching waiver warns.
   - Add a TS TOML parser fixture path only if existing fixture structure has parser-level fixtures. If not, prove through `g3ts` rule fixture output that `guardrail3-ts.toml` accepts `[[waivers]]`.

8. Add deterministic verifier.
   - Add `scripts/verify-universal-waiver-policy.py`.
   - It reads this plan manifest and checks:
     - shared package exists
     - shared package exports required symbols
     - RS parser depends on shared package and no longer defines `WaiverConfig`
     - TS parser depends on shared package and defines `waivers`
     - family-local waiver structs are gone
     - local matcher functions are gone or no longer used where manifest forbids them
     - deps direct dependency cap selector exists
     - deps rule uses shared matcher and emits warning on waived violation

## Key Decisions

- Do not merge `guardrail3-rs.toml` and `guardrail3-ts.toml` root schemas.
  - The root policy fields have different meanings.
  - A single schema would create irrelevant fields and unclear ownership.

- Do not implement runner-side waiver suppression.
  - `G3CheckResult` does not carry a selector.
  - Silent suppression would hide escape hatches.
  - Rule-local warning preserves visibility.

- Do not create deps-specific waiver types.
  - The issue is universal waiver application, not deps-specific semantics.

- Do not wire TS rule waiver behavior yet.
  - This change makes TS syntax parseable and available.
  - Individual TS rules can opt into the shared matcher when a concrete waiver use case appears.

## Files To Modify

- `packages/parsers/g3-guardrail-toml-types/**`
- `packages/parsers/g3rs-toml-parser/**`
- `packages/parsers/g3ts-toml-parser/**`
- `packages/rs/arch/g3rs-arch-types/**`
- `packages/rs/arch/g3rs-arch-config-checks/**`
- `packages/rs/apparch/g3rs-apparch-types/**`
- `packages/rs/apparch/g3rs-apparch-config-checks/**`
- `packages/rs/cargo/g3rs-cargo-types/**`
- `packages/rs/cargo/g3rs-cargo-ingestion/**`
- `packages/rs/cargo/g3rs-cargo-config-checks/**`
- `packages/rs/clippy/g3rs-clippy-types/**`
- `packages/rs/clippy/g3rs-clippy-ingestion/**`
- `packages/rs/clippy/g3rs-clippy-config-checks/**`
- `packages/rs/code/g3rs-code-types/**`
- `packages/rs/code/g3rs-code-ingestion/**`
- `packages/rs/code/g3rs-code-source-checks/**`
- `packages/rs/deps/g3rs-deps-types/**`
- `packages/rs/deps/g3rs-deps-ingestion/**`
- `packages/rs/deps/g3rs-deps-config-checks/**`
- `packages/rs/fmt/g3rs-fmt-types/**`
- `packages/rs/fmt/g3rs-fmt-ingestion/**`
- `packages/rs/fmt/g3rs-fmt-config-checks/**`
- `packages/rs/garde/g3rs-garde-types/**`
- `packages/rs/garde/g3rs-garde-ingestion/**`
- `packages/rs/garde/g3rs-garde-source-checks/**`
- `behavior/fixtures/**`
- `behavior/golden/**`
- `scripts/verify-universal-waiver-policy.py`

## Verification

Required commands:

```sh
python3 scripts/verify-universal-waiver-policy.py
cargo fmt --all --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml -- --check
cargo test --workspace --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml
cargo clippy --workspace --all-targets --all-features --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml -- -D warnings
cargo test --workspace --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml
cargo test --workspace --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml
fixture3 check --all
g3rs validate repo --path .
g3ts validate repo --path .
```
