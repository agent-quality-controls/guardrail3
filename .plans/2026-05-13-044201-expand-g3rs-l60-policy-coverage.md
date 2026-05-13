# Expand G3RS L60 Policy Coverage

## Goal

`L60-delegated-tools-present-policy-invalid` must include every delegated-policy configuration failure that can be visible together after required inputs are valid and delegated tools are present.

The fixture must not include failures that stop execution before G3RS emits the rest of the rule output.

## Current Boundary

The fixture can include:

- Parsed `clippy.toml` policy values.
- Parsed `deny.toml` policy values.
- Parsed `.cargo/mutants.toml` policy values.
- Parsed `.config/nextest.toml` timeout policy values if async test activation can be added without triggering source/file-tree policy rows.

The fixture must not include:

- Failing `cargo deny check`.
- Failing `cargo machete`.
- Failing `cargo dupes`.
- Failing `gitleaks`.
- Failing `cargo publish --dry-run`.
- Hook shell command failures.

Reason: `run_cargo_gates` stops at the first failing delegated command, so these failures would hide later G3RS rows.

## Probe Process

1. Copy `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid` to a temporary directory outside `behavior/fixtures`.
2. Add candidate invalid policy values in groups.
3. Run the fixture command through the same behavior runner path, or run `g3rs validate --path . --inventory` from the temporary fixture repo with the same installed tool environment.
4. Compare Error/Warn rows against the committed L60 baseline.
5. Merge only candidate rows that add new Error/Warn rows without removing any existing L60 Error/Warn rows.

## Candidate Groups

### Deny Policy

Try to add visible rows for:

- `g3rs-deny/graph-no-default-features`
- `g3rs-deny/multiple-versions-floor`
- `g3rs-deny/wildcards-inventory`
- `g3rs-deny/allow-wildcard-paths`
- `g3rs-deny/tokio-full-ban`
- `g3rs-deny/advisories-baseline`
- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/unknown-sources-policy`
- `g3rs-deny/allow-registry-baseline`
- `g3rs-deny/allow-git-inventory`
- `g3rs-deny/ignore-hygiene`
- `g3rs-deny/ignore-accumulation`
- `g3rs-deny/skip-hygiene`
- `g3rs-deny/unknown-keys`

### Test Policy

Try to add visible rows for:

- `g3rs-test/nextest-timeouts`

Only merge it into L60 if activating async test surface does not add L70 source/file-tree rows.

### Clippy Policy

Only add more clippy rows if they become Error/Warn rows and are not already covered as Info inventory in the current L60 output.

Do not expand L60 with extra Info-only inventory rows.

## Files To Modify

- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid`
- `behavior/baselines/g3rs/L60-delegated-tools-present-policy-invalid/command-00.json`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.worklogs/<timestamp>-expand-g3rs-l60-policy-coverage.md`

## Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate-repo`
- `git diff --check`
