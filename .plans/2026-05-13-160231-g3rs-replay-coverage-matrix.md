# G3RS Replay Coverage Matrix And Full Fixture Coverage Plan

## Goal

Make G3RS behavior replay coverage explicit, complete, and mechanically checked.

The end state:

- every active `g3rs-*/*` rule ID is listed in a coverage matrix
- every rule ID is replayed through a public CLI boundary
- every `Error`, `Warn`, and intentional `Info` replay row is pinned by a verifier
- missing coverage cannot be hidden by fixture ordering
- old tests are not used as the primary coverage source

This plan is about replay fixture coverage, not test deletion.

## Public Replay Boundaries

Allowed replay commands:

- `g3rs validate --path <workspace> --inventory`
- `g3rs validate --path <workspace> --family <family> --inventory`
- `g3rs validate --path <workspace> --staged --inventory`
- `g3rs validate-repo`

If a rule ID cannot appear through one of these commands today, the implementation must add a CLI-visible replay surface before marking that rule covered.

Do not replay private rule functions.

Do not replay family ingestion functions directly.

Do not replay assertion crates.

## Current Measured State

Measured on 2026-05-13 from active source under:

- `packages/rs`
- `apps/guardrail3-rs`

Measured replay output from:

- `behavior/baselines/g3rs`
- `behavior/baselines/g3rs-validate-repo`

Current counts:

- `266` distinct `g3rs-*/*` IDs found in active Rust source
- `249` IDs appear in any replay baseline
- `204` IDs appear as `Error` or `Warn` in replay baselines
- `196` IDs are explicitly required in fixture manifests
- `45` IDs appear only as clean or inventory `Info`
- `17` IDs do not appear in any replay baseline

The current problem is not that existing fixture failures hide unexpected `Error` or `Warn` findings.

The current problem is that some rule IDs never become reachable in the replay fixture stack.

## Required New Artifact

Create:

- `behavior/coverage/g3rs-rule-coverage.toml`

The file must contain one row per active source rule ID.

Required row shape:

```toml
[[rule]]
id = "g3rs-deny/unknown-keys"
family = "g3rs-deny"
coverage_status = "planned_fixture"
current_replay = "absent"
target_replay = "error_or_warn"
fixture = "L60-deny-schema-invalid-policy-invalid"
reason = "deny schema-invalid fixture isolates cargo-deny deserialization failures from cargo-deny-valid policy drift"
```

Allowed `current_replay` values:

- `absent`
- `info_only`
- `error_or_warn`

Allowed `target_replay` values:

- `error_or_warn`
- `info_inventory`

Allowed `coverage_status` values:

- `covered`
- `planned_fixture`
- `planned_existing_fixture_expansion`
- `planned_cli_surface`

No permanent `not_replay_suitable` status is allowed.

If a rule ID is not currently CLI-visible, use `planned_cli_surface` and name the CLI surface that will make it replayable.

## Required New Verifier

Create:

- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`

The script must:

- scan active Rust source for `g3rs-*/*` IDs
- read `behavior/coverage/g3rs-rule-coverage.toml`
- read replay baselines from `behavior/baselines/g3rs`
- read replay baselines from `behavior/baselines/g3rs-validate-repo`
- read fixture manifests
- fail if any active source rule ID is missing from the coverage matrix
- fail if the matrix contains a rule ID not present in active source
- fail if a row says `current_replay = "absent"` but the ID appears in replay baselines
- fail if a row says `current_replay = "info_only"` but the ID is absent or appears as `Error` or `Warn`
- fail if a row says `current_replay = "error_or_warn"` but the ID does not appear as `Error` or `Warn`
- fail if `coverage_status = "covered"` and neither `current_replay == target_replay` nor `current_replay = "info_only"` with `target_replay = "info_inventory"`
- fail if `coverage_status = "covered"` and no fixture is named
- fail if `fixture` names a fixture absent from both fixture manifests
- fail if a non-covered row has an empty `reason`

Output:

- success: `behavior-rule-coverage: PASS source:<n> covered:<n> planned:<n>`
- failure: `behavior-rule-coverage: FAIL` plus one concrete line per mismatch

Wire it into:

- `scripts/behavior/verify-all.sh`

Run it after baseline verification.

## Coverage Matrix First Pass

The first implementation pass must create `behavior/coverage/g3rs-rule-coverage.toml` from measured state.

Initial statuses:

- IDs already emitted as `Error` or `Warn`: `coverage_status = "covered"`, `target_replay = "error_or_warn"`
- IDs emitted only as `Info`: `coverage_status = "planned_existing_fixture_expansion"` unless the row is pure positive inventory, then `target_replay = "info_inventory"` and `coverage_status = "covered"`
- IDs absent from baselines but emitted by ordinary family checks: `coverage_status = "planned_fixture"` or `planned_existing_fixture_expansion`
- hook contract IDs absent from baselines: `coverage_status = "planned_cli_surface"`

Do not guess final fixture placement for IDs that require reading the rule implementation.

For those rows, set:

- `fixture = ""`
- `coverage_status = "planned_fixture"`
- `reason = "requires rule implementation read before fixture placement"`

Then the next implementation pass must replace those placeholder reasons with exact fixture decisions.

## Missing Rule IDs To Cover

These IDs are absent from replay baselines now.

### Stage 2 Hook Replay Covered

Stage 2 now covers these IDs through `validate-repo --inventory`:

- hook contract inventory IDs through `R15-hooks-reachable-no-root-cargo`
- hook weakened-command IDs through `R16-hooks-required-steps-present-but-weakened`
- hook modular-script IDs through `R17-hooks-modular-scripts-invalid`

### Deny Rules Missing From Replay

Missing IDs:

- `g3rs-deny/allow-override-channel`
- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/license-exceptions-inventory`
- `g3rs-deny/stricter-advisories-inventory`
- `g3rs-deny/unknown-keys`
- `g3rs-deny/wrappers`

Expected fixtures:

- `L60-deny-cargo-valid-policy-invalid`
- `L60-deny-schema-invalid-policy-invalid`
- `L60-deny-deprecated-advisories-policy-invalid`
- `L60-deny-allow-override-policy-invalid`

Reason:

- these are deny config policy behaviors
- they require valid required inputs and delegated tools
- cargo-deny-valid policy drift, unknown-key schema failure, deprecated-advisory schema failure, and cargo-deny allow/deny validation failure must not be merged into one fixture

Coverage approach:

- start from the existing L60 valid workspace shape
- mutate only `deny.toml` and minimal `Cargo.toml` policy context needed by the rules
- include all advanced deny policy violations that can coexist
- split only if one deny rule suppresses another by making the policy context invalid

### Release Rules Missing From Replay

Missing IDs:

- `g3rs-release/publish-dry-run-workflow`
- `g3rs-release/registry-token`
- `g3rs-release/release-plz-workflow`
- `g3rs-release/release-profile-inventory`

Expected fixture:

- `L70-release-workflow-policy-violated`

Reason:

- these are release filetree/workflow/package-profile behaviors
- they should not be forced into the existing metadata fixture unless the workflows do not change which release metadata checks are observable

Coverage approach:

- start from `L70-release-metadata-policy-violated`
- add `.github/workflows` files with wrong or missing workflow content
- add package publish/profile combinations that trigger release-profile inventory
- add registry token surface that triggers `registry-token`
- keep existing release metadata fixture unchanged unless implementation review proves all workflow rules can coexist there without changing emitted metadata findings

### Input Failure Rules Missing From Replay

Missing IDs:

- `g3rs-code/input-failures`
- `g3rs-garde/input-failures`
- `g3rs-test/filetree-input-failures`
- `g3rs-test/source-input-failures`

Expected fixture:

- `L45-source-and-filetree-input-failures`

Reason:

- these are not policy violations after delegated tools
- they are parse/read failures after required config inputs are present enough for source/filetree ingestion to run
- they belong above L40 malformed config input and below L50 missing delegated tools

Coverage approach:

- use valid root config files
- add source/filetree entries that cause source or filetree ingestion failures
- avoid malformed `Cargo.toml`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, `release-plz.toml`, or `cliff.toml`, because those would hide source/filetree ingestion

### Toolchain And Cargo Missing Rules

Missing IDs:

- `g3rs-toolchain/msrv-consistency`
- `g3rs-cargo/approved-allow-inventory`

Expected fixture:

- `L60-delegated-tools-present-policy-invalid`

Coverage approach:

- add a package `rust-version` or workspace policy mismatch that triggers `msrv-consistency`
- add an approved manifest allow entry that triggers `approved-allow-inventory`

If either rule is positive inventory only, mark it as `target_replay = "info_inventory"` and pin the exact info row in the coverage matrix.

## Info-Only IDs To Decide

These IDs currently appear only as `Info`.

The coverage matrix must classify each as either:

- intentionally replayed info inventory
- needs a violation fixture

IDs:

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
- `g3rs-deny/extra-deny-bans-inventory`
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

Do not leave these as incidental baseline noise.

If they are intended positive inventory, the matrix must say that.

If they can emit warning/error, add or expand fixtures.

## Fixture Minimality Rules

When adding coverage:

- merge missing IDs into an existing fixture if they do not change earlier unlock state
- merge multiple missing IDs into one new fixture if they can all emit independently
- split only when one mutation prevents another rule from running or changes its branch
- never add one fixture per rule by default
- never add a fixture just because an old unit test exists

Each new fixture must state its hiding boundary:

- which layer it belongs to
- which earlier layers are valid
- which behaviors it intentionally breaks
- which missing IDs it covers
- why those IDs do not hide each other

## Implementation Order

### Stage 1: Matrix Infrastructure

Create:

- `behavior/coverage/g3rs-rule-coverage.toml`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`

Wire:

- `scripts/behavior/verify-all.sh`

Expected result:

- verifier passes with planned rows
- no fixture behavior changes yet

### Stage 2: Hook Replay Surface And Hook Fixtures

Implement:

- CLI-visible inventory for family hook contracts
- `R16-hooks-required-steps-present-but-weakened`
- `R17-hooks-modular-scripts-invalid`

Update:

- validate-repo manifest
- validate-repo baselines
- coverage matrix

Expected result:

- hook-contract IDs covered
- missing hook rule IDs covered or explicitly still planned with reason

### Stage 3: Deny Advanced Policy Fixture

Implement:

- `L60-deny-cargo-valid-policy-invalid`
- `L60-deny-schema-invalid-policy-invalid`
- `L60-deny-deprecated-advisories-policy-invalid`
- `L60-deny-allow-override-policy-invalid`

Update:

- behavior fixture manifest
- behavior baselines
- coverage matrix

Expected result:

- missing deny rule IDs covered

### Stage 4: Source And Filetree Input Failure Fixture

Implement:

- `L45-source-and-filetree-input-failures`

Update:

- behavior fixture manifest
- behavior baselines
- coverage matrix

Expected result:

- `g3rs-code/input-failures` covered
- `g3rs-garde/input-failures` covered
- `g3rs-test/filetree-input-failures` covered
- `g3rs-test/source-input-failures` covered

### Stage 5: Release Workflow Fixture

Implement:

- `L70-release-workflow-policy-violated`

Update:

- behavior fixture manifest
- behavior baselines
- coverage matrix

Expected result:

- missing release workflow/profile/token IDs covered

### Stage 6: Info-Only Rule Decisions

For every `info_only` row:

- read the rule implementation
- decide whether it is positive inventory or an untested violation branch
- update coverage matrix
- add fixture rows if needed

Expected result:

- no rule ID remains incidental
- every source ID is either covered as `Error`/`Warn` or intentionally covered as `Info`

## Required Verification

Every stage must run:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

When a stage changes validate-repo fixtures, also run:

```sh
g3rs validate-repo
```

## Adversarial Review

After each stage, send reviewers against the original plan and code.

Reviewer A:

- checks the coverage matrix against active source rule IDs
- verifies no source rule ID is missing or extra

Reviewer B:

- checks fixture minimality
- verifies new fixture splits are caused by real hiding boundaries
- reports fixture merges that are safe

Reviewer C:

- checks baselines and manifests
- verifies every new `Error`, `Warn`, and intended `Info` row is pinned
- verifies no fixture emits unlisted `Error` or `Warn`

No stage is done until adversarial review returns no `MUST FIX`.
