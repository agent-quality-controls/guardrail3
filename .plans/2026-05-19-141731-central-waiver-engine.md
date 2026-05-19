# Central Waiver Engine

## Goal

Make waivers automatic and universal for G3RS and G3TS.

End state:

- `[[waivers]]` uses `rule`, `subject`, `selector`, and `reason`.
- `file` is deleted from the waiver config contract.
- Every `G3CheckResult` has a deterministic waiver key.
- Rule packages do not parse waivers.
- Rule packages do not call waiver matching helpers.
- Runners apply waivers once after each family returns findings.
- A matched `Error` becomes a visible `Warn`.
- A matched `Warn` stays a visible `Warn`.
- A matched `Info` stays `Info`.
- A waiver never silently removes a finding.
- All RS and TS families get waiver support through the shared result pipeline.

## Approach

1. Create the shared waiver package.
   - Add `packages/shared/guardrail3-waivers`.
   - Own `WaiverConfig`, `WaiverKey`, `WaiverReason`, exact matching, and result application.
   - Use `subject`, not `file`.
   - Keep exact matching on `rule`, `subject`, `selector`.
   - Keep non-empty `reason` required.
   - The package may depend on `guardrail3-check-types`.
   - The package must not depend on G3RS or G3TS runners.
   - The package must not know any rule IDs or family names.

2. Update `G3CheckResult`.
   - Keep the existing `new(...)` constructor temporarily so every rule does not need to change in this refactor.
   - Internally normalize:
     - `subject = file.unwrap_or("-")`
     - `selector = line:<n>` when line exists
     - otherwise `selector = title:<stable-slug>`
   - Add getters:
     - `subject()`
     - `selector()`
     - `waiver_key()`
     - `waiver_reason()`
   - Add a method used only by the central engine:
     - `apply_waiver(reason)`
   - Add builder methods for rules that need stable selectors:
     - `with_subject(subject)`
     - `with_selector(selector)`
   - Keep `file()` as a display alias for existing renderers for now.

3. Add central waiver application.
   - Place it in `guardrail3-waivers`.
   - It must not depend on G3RS or G3TS runners.
   - It must not know any rule IDs.

4. Wire G3RS runner.
   - Parse `guardrail3-rs.toml` once in `execute`.
   - Apply waivers to each family result vector before pushing it into the report.
   - Parse repo-level waivers in `execute_repo` if root `guardrail3-rs.toml` exists.
   - Do not pass waivers through family-specific inputs for waiver behavior.

5. Wire G3TS runner.
   - Parse `guardrail3-ts.toml` once in `execute`.
   - Apply waivers to each family result vector before pushing it into the report.
   - Do not pass waivers through family-specific inputs for waiver behavior.

6. Remove rule-local waiver handling.
   - Delete direct calls to `find_waiver_reason`, `has_waiver`, and `WaiverMatch` from RS rule packages.
   - Remove `WaiverConfig` from family input types.
   - Remove `g3-guardrail-toml-types` from family check/type/ingestion package dependencies.
   - Remove rule-local downgraded warning branches.
   - Let the runner downgrade after findings are produced.

7. Update reporting.
   - Plain text output should show `subject`.
   - Waived findings should include the waiver reason in the rendered message.
   - Output should make the waiver key visible enough for a user to write a waiver.

8. Update fixtures.
   - Replace `file = ...` with `subject = ...`.
   - Update waived fixture output to show a warning produced by central waiver application.
   - Add one G3TS fixture proving `guardrail3-ts.toml` waivers are parsed and applied.

9. Add deterministic verifier.
   - Add `scripts/verify-central-waiver-engine.py`.
   - It reads the manifest and checks:
     - waiver config has `subject`, not `file`
     - result type has subject and selector accessors
     - RS runner calls the central waiver engine
     - TS runner calls the central waiver engine
     - no family rule package imports `WaiverMatch`, `find_waiver_reason`, or `has_waiver`
     - no `[[waivers]]` fixture uses `file`
     - deps fixture uses `subject`

## Key Decisions

- No backward compatibility.
  - `file` is removed from waiver config.
  - Existing fixtures and docs must move to `subject`.

- Do not edit every rule constructor in this refactor.
  - There are hundreds of `G3CheckResult::new(...)` call sites.
  - A central default key gives universal support now.
  - A later strict-constructor refactor can force explicit selectors rule by rule.

- Do not suppress findings.
  - Waived findings remain visible.
  - Waivers downgrade errors; they do not hide code quality inventory.

- Do not put waiver logic in families.
  - Families emit findings.
  - Runners apply shared policy once.

## Files To Modify

- `packages/parsers/g3-guardrail-toml-types/**`
- `packages/shared/guardrail3-waivers/**`
- `packages/shared/guardrail3-check-types/**`
- `apps/guardrail3-rs/crates/logic/validate-command/**`
- `apps/guardrail3-ts/crates/logic/validate-command/**`
- RS family files currently importing waiver matching helpers
- `apps/guardrail3-rs/crates/io/outbound/report/**`
- `apps/guardrail3-ts/crates/io/outbound/report/**`
- `behavior/fixtures/**`
- `behavior/golden/**`
- `scripts/verify-central-waiver-engine.py`

## Verification

Required commands:

```sh
python3 scripts/verify-central-waiver-engine.py
cargo fmt --all --manifest-path apps/guardrail3-rs/Cargo.toml -- --check
cargo fmt --all --manifest-path apps/guardrail3-ts/Cargo.toml -- --check
cargo test --workspace --manifest-path apps/guardrail3-rs/Cargo.toml
cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml
cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-rs/Cargo.toml -- -D warnings
cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings
fixture3 check --all --json
g3rs validate repo --path .
g3ts validate repo --path .
```
