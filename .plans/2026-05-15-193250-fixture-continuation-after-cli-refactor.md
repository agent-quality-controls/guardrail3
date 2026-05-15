# Goal

Continue the fixture migration after the CLI refactor lands without touching the refactor-in-progress worktree.

The fixture migration must still replace test-only behavior coverage with fixture3 outputs, but it must not assume the old CLI surface.

# Current Constraint

Another agent is refactoring the CLI surface now.

Observed in the dirty worktree:

- old command shape is being removed:
  - `g3rs validate-repo`
  - `g3rs validate --path <path>`
- new command shape is being introduced:
  - `g3rs init repo`
  - `g3rs init workspace --path <path>`
  - `g3rs validate repo`
  - `g3rs validate workspace --path <path>`

Until that work is committed, fixture work is planning-only.

Do not edit:

- `fixture3.yaml`
- `scripts/behavior/*`
- `behavior/golden/*`
- `behavior/fixtures/*`
- `behavior/migration/*`
- Rust CLI, validate-command, renderer, hooks, or family-runner code

# Current Fixture State

Last committed fixture infrastructure has these fixture3 suites:

- `g3rs-validate`
- `g3rs-validate-repo`
- `g3rs-code-ingestion`

The public rule replay track is closed in the committed baseline:

- `behavior-rule-coverage: PASS source:266 covered:266 planned:0`

The test-replacement track is not closed.

Current dirty ledger counts after the CLI refactor started:

- `needs_serialized_ingestion_output`: 420
- `needs_rule_fixture_or_golden_output`: 236
- `needs_family_runner_output`: 47
- `needs_validate_command_output`: 23
- `needs_cli_output`: 11
- `needs_renderer_output`: 4
- `keep_public_api_contract`: 13

These counts are not final until the CLI refactor is committed and the ledgers are regenerated from that committed state.

# Stable Architecture Decisions

These decisions do not depend on the CLI command spelling:

- Fixture3 remains the fixture runner.
- Verifier commands emit JSON.
- Owned Rust structs used in fixture output derive `serde::Serialize`.
- `serde_json` belongs in verifier/test-support crates, not family type crates.
- Python fixture scripts may copy fixtures, run Rust binaries, normalize generic unstable values, and wrap command output.
- Python fixture scripts must not parse Rust, TOML, or family data.
- No adapters, exporters, ingestion suites, replay suites, replay record maps, duplicated fixture structs, or canonical fact layers.

# Unstable Names During CLI Refactor

Use these neutral names in all new planning until the CLI refactor lands:

- `workspace validation command`
- `repo validation command`
- `workspace init command`
- `repo init command`
- `CLI command output`
- `validate-command owned decision output`

Do not write new fixture plans that hardcode:

- `g3rs validate-repo`
- `g3rs validate --path <path>`

After the CLI refactor lands, replace neutral names with the committed command names.

# Post-Refactor Rebaseline Steps

After the CLI refactor is committed:

1. Read the committed CLI plan and CLI implementation.
2. Regenerate `behavior/migration/g3rs-test-fixture-ledger.toml`.
3. Regenerate `behavior/migration/g3rs-kept-test-disposition.toml`.
4. Recompute disposition counts.
5. Update this plan if counts changed.
6. Run `fixture3 check --all`.
7. Run `bash scripts/behavior/verify-all.sh`.
8. Run repo-level and workspace-level G3RS validation using only the committed new CLI shape.

# Implementation Order After CLI Refactor

## 1. CLI Output

Target rows:

- `needs_cli_output`

Why first:

- CLI command names and help output are changing now.
- These rows cannot be implemented correctly until the new CLI surface is final.

Expected fixture outputs:

- accepted command parsing
- rejected old command shapes
- rejected invalid family names
- root help contract
- `init repo` command output and filesystem effects
- `init workspace` command output and filesystem effects
- stderr routing for command failures

Fixture boundary:

- fixture3 command output: argv, cwd, exit code, stdout, stderr
- no Rust internal structs unless the committed CLI exposes typed JSON output

## 2. Renderer Output

Target rows:

- `needs_renderer_output`

Why second:

- renderer output format can be affected by CLI scope names and report fields.

Expected fixture outputs:

- inventory hidden by default
- no-findings rendering
- finding message rendering
- scope/root rendering after CLI refactor

Fixture boundary:

- snapshot rendered text if the behavior is formatting
- serialize renderer input only if the test asserts input handling

## 3. Validate-Command Owned Decision Output

Target rows:

- `needs_validate_command_output`

Why third:

- validate-command is being edited by the CLI refactor.
- Existing tests mention cargo gates, staged paths, family selection, workspace routing, and delegated failures.

Expected fixture outputs:

- cargo gate command set
- cargo gate deduplication
- family selection
- staged workspace routing
- rules-only behavior if retained by the committed CLI contract
- delegated command failure handling

Fixture boundary:

- prefer serialized owned validate-command decision structs
- use command stdout/stderr/exit only where the public CLI behavior is the contract

## 4. Family Runner Output

Target rows:

- `needs_family_runner_output`

Expected fixture outputs:

- hook contract injection into source checks
- Rust hook requirements from every family contract
- family selection and inactive family behavior
- duplicate or merged findings if currently tested only by unit tests

Fixture boundary:

- run the real family runner
- serialize owned runner aggregation output
- do not call private rule functions directly

## 5. Serialized Ingestion Output

Target rows:

- `needs_serialized_ingestion_output`

Current proof:

- `g3rs-code-ingestion` has a working fixture-output crate.
- It serializes real ingestion return values.
- It uses 2 fixtures, not all 36, to avoid bloated golden output.

Next family selection rule:

- choose the family with the highest kept-test count where owned type structs already derive `Serialize`
- create a private fixture-output crate under that ingestion workspace
- emit real ingestion return values with `serde_json`
- use the minimum fixture set that covers:
  - success
  - parse failure
  - missing file
  - malformed file
  - path discovery
  - fail-closed behavior

Do not create one fixture per test.

Do not include all public replay fixtures by default.

## 6. Rule Fixture Or Golden Output

Target rows:

- `needs_rule_fixture_or_golden_output`

Decision rule:

- if the behavior can appear through a public validation fixture without hidden earlier failures, use public fixture output
- if the rule is pure and not CLI-visible, use rule-level golden output from the actual public input/finding type
- do not add rule fixtures for behavior already covered by public replay

## 7. Public API Contracts

Target rows:

- `keep_public_api_contract`

Default:

- keep compile/API tests for now

Only replace with fixtures if:

- public API metadata can be generated mechanically from Rust
- the snapshot is not manually maintained

# Ledger Update Rule

Do not mark a kept test as replaced until all three are true:

- fixture3 approved output exists
- the output boundary is named in the disposition ledger
- `bash scripts/behavior/verify-all.sh` passes

For every category migration:

- update the relevant fixture output
- update `behavior/migration/g3rs-kept-test-disposition.toml`
- update any verifier that checks disposition names
- keep forbidden fixture architecture terms blocked

# Verification After Each Post-Refactor Stage

Run:

```sh
fixture3 check --all
bash scripts/behavior/verify-all.sh
g3rs validate repo --path .
```

Run at least one workspace validation command against the clean fixture workspace selected for the stage:

```sh
g3rs validate workspace --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
```

Use the committed command names. If they differ from the draft names above, update this plan before implementation.

# Stop Conditions

Stop and replan before coding if:

- the CLI refactor changes command names again
- fixture3 suite names change
- validate-command no longer owns serializable decision structs
- an owned Rust output type cannot derive `serde::Serialize`
- a fixture output grows because the fixture set is too broad
- Python would need family-specific parsing or field selection

# Done State For The Whole Fixture Migration

The fixture migration is complete only when:

- `behavior/migration/g3rs-kept-test-disposition.toml` has zero rows needing fixture output
- all remaining rows are deliberate public API contracts
- `fixture3 check --all` passes
- `bash scripts/behavior/verify-all.sh` passes
- repo-level validation passes through the committed new CLI
- at least one clean workspace validation passes through the committed new CLI
