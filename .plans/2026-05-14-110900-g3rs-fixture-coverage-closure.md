# G3RS Fixture Coverage Closure

## Goal

Finish the G3RS replay fixture stack so every active `g3rs-*/*` rule ID is either:

- replayed as `Error` or `Warn` through a public CLI fixture, or
- explicitly pinned as intentional `Info` inventory through a public CLI fixture.

The fixture set must stay minimal.

Do not add a fixture when an existing fixture can expose the behavior without hiding another behavior.

Do not merge fixture mutations when one mutation prevents another rule branch from being observable.

## Current Measured State

Measured from:

- `behavior/coverage/g3rs-rule-coverage.toml`
- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`

Current counts:

- Active source rule IDs: `266`
- Covered rows: `229`
- Planned rows: `37`
- Rows currently absent from replay: `6`
- Rows currently emitted as `Info` only: `47`
- Rows currently emitted as `Error` or `Warn`: `213`

Planned rows:

- `4` rows need a new fixture: release workflow fixture.
- `2` absent rows should be tried as existing fixture expansion first.
- `31` info-only rows need rule implementation audit.

## Public Boundary

Only these replay surfaces are allowed:

- `g3rs validate --path <workspace> --inventory`
- `g3rs validate --path <workspace> --family <family> --inventory`
- `g3rs validate --path <workspace> --staged --inventory`
- `g3rs validate-repo`

Do not replay private rule functions.

Do not replay family ingestion functions.

Do not replay assertion crates.

## Goldencheck Boundary

Replay storage is `goldencheck`.

Required commands:

```sh
goldencheck check --all
goldencheck approve --suite <suite> --change <plan-or-worklog>
```

Do not recreate:

- `behavior/baselines`
- `generate-baselines.py`
- `verify-baselines.py`
- custom approved-vs-received comparison code

## Fixture Minimality Rule

For every rule row:

1. Read the rule implementation.
2. Identify the earliest fixture layer where the rule can run.
3. Check whether the target fixture already reaches that layer.
4. Add the mutation to that fixture if it does not hide any existing required result.
5. Split only when the mutation changes unlock state or suppresses another rule branch.
6. Pin the resulting `Error`, `Warn`, or intentional `Info` row in the fixture manifest.
7. Approve the changed golden output through `goldencheck`.
8. Update `behavior/coverage/g3rs-rule-coverage.toml`.

Hiding means:

- root discovery failure hides all workspace checks
- missing guardrail config hides config parser and family checks
- invalid guardrail config hides semantic config checks
- missing required input hides malformed input checks
- malformed required input hides delegated policy checks
- missing delegated tool hides delegated policy wiring checks
- invalid delegated policy hides project/source policy checks that depend on it

## Stage 1: Existing L60 Expansion

Target fixture:

- `L60-delegated-tools-present-policy-invalid`

Rows:

- `g3rs-cargo/approved-allow-inventory`
- `g3rs-toolchain/msrv-consistency`

Required work:

- Read both rule implementations.
- Mutate the existing L60 fixture if both branches can fire without hiding current L60 findings.
- If one mutation hides another branch, split into the smallest new L60 fixture.
- Update the fixture manifest with exact required result rows.
- Update coverage rows from `absent` to the observed state.

Expected result:

- `g3rs-cargo/approved-allow-inventory` no longer absent.
- `g3rs-toolchain/msrv-consistency` no longer absent.

## Stage 2: Release Workflow Fixture

New fixture:

- `L70-release-workflow-policy-violated`

Rows:

- `g3rs-release/publish-dry-run-workflow`
- `g3rs-release/registry-token`
- `g3rs-release/release-plz-workflow`
- `g3rs-release/release-profile-inventory`

Required work:

- Start from the smallest existing release fixture that has valid required inputs and valid delegated policy.
- Add only workflow and package-profile inputs needed to expose these four release rules.
- Keep metadata-only release violations in `L70-release-metadata-policy-violated` unless the workflow mutation does not affect their observability.
- Pin exact required result rows in the fixture manifest.
- Update coverage rows from `planned_fixture` to covered rows.

Expected result:

- No release workflow rule remains absent.
- The new fixture exists only if it proves a real hiding boundary.

## Stage 3: Info-Only Decision Audit

Rows:

- `g3rs-cargo/disallowed-macros-deny`
- `g3rs-cargo/priority-order`
- `g3rs-cargo/workspace-metadata`
- `g3rs-clippy/avoid-breaking-exported-api`
- `g3rs-clippy/ban-reason-quality`
- `g3rs-clippy/duplicate-bans`
- `g3rs-clippy/extra-method-ban`
- `g3rs-clippy/extra-type-ban`
- `g3rs-clippy/library-global-state`
- `g3rs-clippy/macro-bans`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-clippy/policy-context-parseable`
- `g3rs-clippy/unknown-keys`
- `g3rs-code/unsafe-code-lint`
- `g3rs-code/unused-crate-dependencies-allow`
- `g3rs-deny/highlight-inventory`
- `g3rs-fmt/rustfmt-extra-settings-inventory`
- `g3rs-hooks/local-override-inventory`
- `g3rs-hooks/modular-directory-inventory`
- `g3rs-hooks/modular-scripts-inventory`
- `g3rs-hooks/no-bypass-instructions`
- `g3rs-hooks/pre-commit-file-size-inventory`
- `g3rs-hooks/script-stats-inventory`
- `g3rs-release/binary-release-workflow`
- `g3rs-release/cliff-baseline`
- `g3rs-release/crate-inventory`
- `g3rs-release/linux-release-target`
- `g3rs-release/publish-status-inventory`
- `g3rs-release/release-plz-baseline`
- `g3rs-release/semver-checks-installed`

For each row:

- If the rule is pure positive inventory, mark the coverage row `covered`, keep `target_replay = "info_inventory"`, and write a specific reason.
- If the rule has an untested `Error` or `Warn` branch, add that branch to the earliest fixture where it can fire.
- If the rule cannot emit `Error` or `Warn` by design, do not invent a violation fixture.

Expected result:

- No coverage row uses the placeholder reason `currently appears only as Info`.
- No `info_only` row remains `planned_existing_fixture_expansion`.

## Stage 4: Pinning Audit

Required work:

- Check every planned row that becomes covered.
- Ensure the exact replay row is pinned in the relevant fixture manifest `required_results`.
- Intentional `Info` inventory rows must also be pinned if they are the reason the row is covered.

Expected result:

- Fixture output drift cannot silently lose a newly covered rule row.

## Stage 5: Coverage Closure

Required state:

- `coverage_status = "covered"` for all `266` rows.
- `current_replay = "error_or_warn"` for every row with `target_replay = "error_or_warn"`.
- `current_replay = "info_only"` only for intentional inventory rows with `target_replay = "info_inventory"`.
- `fixture` is non-empty for every row.
- `reason` is specific for every row and does not contain placeholder text.

Expected verifier output:

```text
behavior-rule-coverage: PASS source:266 covered:266 planned:0
```

## Required Verification

Run after every stage:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
goldencheck check --all
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

If a repo fixture changes, also run:

```sh
g3rs validate-repo
```

## Required Review

After each stage:

- Compare the implementation against this plan and manifest.
- Check whether any new fixture can be merged into an existing fixture.
- Check whether any fixture mutation hides a branch that was previously observable.
- Check whether every new output row is pinned.
- Check whether all golden output changes were approved through `goldencheck`.

No stage is complete until these checks pass.
