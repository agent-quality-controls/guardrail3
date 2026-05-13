# Summary

Audited the G3RS L50 delegated-tool layer and confirmed host executable availability is currently limited to the deps family checks for `cargo-deny`, `cargo-machete`, `cargo-dupes`, and `gitleaks`.

Cleaned the L50 and L60 fixtures so normal `g3rs validate --path . --inventory` no longer fails on unrelated rustfmt or linker noise before the intended findings.

# Decisions

- Did not add another L50 fixture because all four missing-tool checks are independent and visible in the existing L50 fixture.
- Kept L50 as normal validate instead of `--rules-only` because delegated-tool failures should be visible through the public validate path.
- Replaced the behavior runner's hand-written PATH whitelist with a filtered copy of the current executable PATH surface, excluding only the delegated tool names under test.
- Added a rustfmt-clean comment-only `src/lib.rs` to L50 and L60 so cargo gates can run without introducing facade or formatting findings.

# Key Files

- `.plans/2026-05-13-025358-fix-g3rs-l50-delegated-tool-fixture.md`
- `scripts/behavior/baseline_common.py`
- `behavior/fixtures/g3rs/L50-required-inputs-valid-delegated-tools-missing/repo/src/lib.rs`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid/repo/src/lib.rs`
- `behavior/baselines/g3rs/L50-required-inputs-valid-delegated-tools-missing/command-00.json`
- `behavior/baselines/g3rs/L60-delegated-tools-present-policy-invalid/command-00.json`

# Verification

- `scripts/behavior/verify-all.sh`
- Direct L50/L60 baseline inspection for Error/Warn rows and forbidden stderr fragments.
- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- `g3rs validate-repo`

# Next Steps

- Continue at L60 only after inventorying delegated policy wiring failures that are visible after delegated tools are present.
