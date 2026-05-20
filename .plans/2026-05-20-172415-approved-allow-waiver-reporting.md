# Approved Allow Waiver Reporting

## Goal

Make `g3rs-cargo/approved-allow-inventory` report consistently after central waiver application.

The end state:
- An approved manifest `allow` without a waiver is still an error.
- The same finding with a matching waiver is downgraded by the central waiver engine.
- The visible title and message do not claim the waiver reason is missing when the central waiver engine has already printed the matching waiver reason.
- The rule still does not parse or match waivers locally.

## Approach

1. Add a focused behavior verifier for `approved_allow_inventory`.
   - File: `scripts/verify-approved-allow-waiver-reporting.py`
   - Build a temporary standalone package `Cargo.toml` with `[lints.clippy] multiple_crate_versions = "allow"`.
   - Add a matching `guardrail3-rs.toml` waiver.
   - Run `g3rs validate workspace` against the fixture.
   - Prove the rendered finding uses a neutral title and message that are correct after central waiver application.
   - This verifier fails before the code change because the rendered rule title is `approved allow entry missing reason`.

2. Update the rule wording only.
   - File: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/approved_allow_inventory.rs`
   - Change title from `approved allow entry missing reason` to `approved allow entry requires waiver`.
   - Change the message from "Add a waiver..." to a wording that says the allow entry is an escape hatch and must have an exact waiver.
   - Keep the rule ID, severity, subject, selector, count behavior, and central waiver flow unchanged.

3. Verify the package and the real downstream symptom.
   - Run cargo fmt/checks for `g3rs-cargo-config-checks`.
   - Reinstall `g3rs`.
   - Run `g3rs validate workspace --path /Users/tartakovsky/Projects/websmasher/websmasher/packages/llm/ws-llm-client`.
   - Confirm the output no longer contains `approved allow entry missing reason`.

## Key Decisions

- Do not move waiver matching back into cargo rules.
  - The current central waiver plan says rule packages emit findings and runners apply waivers.
  - Reintroducing local waiver matching would reverse that architecture.

- Do not special-case the renderer.
  - The defect is rule wording that is false after waiver application.
  - The renderer is correctly printing the central waiver reason.

- Do not weaken the rule.
  - The allow entry still requires an exact waiver.
  - A waiver still downgrades the error to a warning instead of hiding the finding.

## Files To Modify

- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/approved_allow_inventory.rs`
- `scripts/verify-approved-allow-waiver-reporting.py`
