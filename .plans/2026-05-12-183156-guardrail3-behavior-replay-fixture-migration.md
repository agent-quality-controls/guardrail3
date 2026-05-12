# Guardrail3 Behavior Replay Fixture Migration

## Goal

Replace guardrail3 behavior tests with replay fixtures and golden baselines.

The accepted CLI behavior becomes the oracle.

Old behavior tests are deleted after their behavior is represented in replay output.

The general strategy is defined in:

```text
/Users/tartakovsky/Projects/kb/.plans/2026-05-12-165330-behavior-replay-golden-baselines.md
```

This file is the guardrail3-specific implementation plan.

## Scope

In scope:

- `g3rs validate --path <fixture>/repo --inventory`
- `g3rs validate-repo` run from a fixture repo root
- `g3ts validate --path <fixture>/repo --inventory`
- `g3ts validate-repo` run from a fixture repo root
- existing behavior tests under `packages/rs`, `packages/ts`, `apps/guardrail3-rs`, and `apps/guardrail3-ts`
- parser behavior only when it changes CLI-visible guardrail output

Out of scope:

- tests for the replay runner
- tests for the normalizer
- tests for semantic diff
- tests for baseline metadata fail-closed behavior
- compile-only public API shape tests where compiler output is the contract
- tests in `legacy/`

## Public Replay Boundaries

Use only these public boundaries for behavior fixtures:

```text
g3rs validate --path <workspace> --inventory
g3rs validate --path <workspace> --family <family> --inventory
g3rs validate --path <workspace> --staged --inventory
g3rs validate-repo
g3ts validate --path <workspace> --inventory
g3ts validate --path <workspace> --family <family> --inventory
g3ts validate --path <workspace> --staged --inventory
g3ts validate-repo
```

Do not replay private rule functions.

Do not replay family ingestion functions directly.

Do not replay assertion crates.

## Generated Baseline Output

The replay runner emits generated normalized output.

Agents do not author expected output records.

The generated baseline captures:

- command
- fixture ID
- exit code
- stdout records
- stderr records
- finding IDs
- severity
- family
- relative file path
- message text
- help text
- generated files, when a command produces files

Normalize:

- absolute fixture paths to `$FIXTURE`
- fixture repo paths to `$REPO`
- target directory paths to `$TARGET`
- path separators to `/`
- ordering when ordering is not semantic

Do not normalize:

- rule IDs
- severity
- message text
- help text
- relative file path
- exit code

Guardrail messages are behavior.

If a message changes, the generated baseline diff must show it.

## Fixture Root

Use one fixture root in this repo.

```text
behavior/
  fixtures/
    g3rs/
    g3ts/
  migration/
    g3rs-test-ledger.toml
    g3ts-test-ledger.toml
  baselines/
    index.toml
  deltas/
  schemas/
```

Do not put fixtures under package-local test directories.

Fixtures are product behavior assets, not package tests.

## Live Validation Routing

Replay fixture files stay in normal staged-file safety checks.

The pre-commit hook must still scan them for:

```text
merge-conflict markers
secrets
file size
```

The pre-commit hook must not route them into live workspace validation.

Excluded from fixture paths:

```text
package lockfile checks
database migration consistency checks
Rust owning-unit discovery
TypeScript owning-unit discovery
```

Reason:

```text
behavior fixtures intentionally contain missing, invalid, and policy-violating workspaces
```

The behavior fixture verifier must fail if `.githooks/pre-commit` stops excluding:

```text
behavior/fixtures/
```

from live validation routing.

## Fixture Levels

Use fixture levels based on CLI workspace-root discovery and check unlock state.

Use the same level names for G3RS and G3TS.

Each level has one main fixture unless a concrete hiding conflict forces a split.

```text
L00-workspace-root-not-found
L10-workspace-root-found-guardrail-config-missing
L20-workspace-root-found-guardrail-config-invalid
L30-guardrail-config-valid-required-inputs-missing
L40-required-inputs-present-invalid
L50-required-inputs-valid-delegated-tools-missing
L60-delegated-tools-present-policy-invalid
L70-delegated-policy-valid-project-policy-violated
L80-project-policy-valid-clean
```

Level meanings:

```text
L00: CLI cannot select the workspace root it is supposed to validate
L10: CLI selects a workspace root, guardrail config is absent
L20: workspace root exists, guardrail config exists, guardrail config cannot be parsed or validated
L30: guardrail config is valid, required input files are absent
L40: required input files exist, at least one required input file is malformed
L50: required input files parse, delegated external tools or packages are absent
L60: delegated tools or packages exist, delegated policy wiring is weakened or wrong
L70: delegated policy wiring is valid, project source, content, package structure, or architecture violates policy
L80: realistic project satisfies all policies and emits no findings
```

Workspace root means:

```text
G3RS: the root selected by Cargo workspace or package discovery
G3TS: the root selected by package/adopted-unit discovery
```

Guardrail config means:

```text
G3RS: guardrail3-rs.toml
G3TS: guardrail3-ts.toml
```

Fixture split rule:

```text
split only when one expected CLI-visible behavior prevents another expected CLI-visible behavior from being observable
```

Allowed split reasons:

```text
workspace root not found hides all workspace checks
missing guardrail config hides guardrail parser and configured-family checks
invalid guardrail config hides semantic config checks
missing required input hides invalid input checks
invalid required input hides semantic policy wiring checks
missing delegated tool hides weakened delegated policy checks
weakened delegated policy hides source/content policy checks that rely on delegated enforcement
validate-repo-only behavior cannot be observed from validate --path
validate --path behavior cannot be observed from validate-repo
```

Disallowed split reasons:

```text
one old test existed
one bug existed
one rule existed
one package existed
smaller fixture feels easier
clean smoke case
family ownership feels separate
```

## Fixture Metadata

Each fixture has `fixture.toml`.

Example:

```toml
id = "L60-delegated-tools-present-policy-invalid"
tool = "g3rs"
run_from = "repo"
commands = [
  ["validate", "--path", "repo", "--inventory"],
  ["validate-repo"]
]
expected_exit = "nonzero"
level = "delegated_tools_present_policy_invalid"

valid_state = [
  "workspace_root_found",
  "guardrail_config_valid",
  "required_inputs_present",
  "required_inputs_valid",
  "delegated_tools_present"
]

intentionally_invalid = [
  "delegated_policy_invalid"
]
```

`fixture.toml` explains why the fixture exists.

It does not contain expected findings.

The generated baseline owns observed output.

## Fixture Size Rule

Only the clean fixture keeps a full copied workspace.

For G3RS, the initial full clean fixture is:

```text
behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo
```

It is copied from:

```text
packages/rs/deny/g3rs-deny-config-checks
```

All lower levels must be stripped to the smallest workspace that exposes that level.

Do not copy full source trees into lower levels.

Lower levels contain only:

```text
fixture.toml
repo/Cargo.toml when workspace-root selection must succeed
repo/guardrail3-rs.toml when guardrail config must exist
required config files needed by that level
small source files only when source policy must be observed
```

If a lower level needs a full copied workspace, split the reason into the plan before adding it.

## Temporary Migration Ledger

Use a temporary ledger only during migration.

The ledger is not a permanent behavior oracle.

```text
behavior/migration/g3rs-test-ledger.toml
behavior/migration/g3ts-test-ledger.toml
```

One row maps one old test file or old test function to migration status.

Required shape:

```toml
[[test]]
old_test_path = "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/edition_mismatch/rule_tests/mismatched.rs"
old_test_name = "reports_mismatched_rustfmt_edition"
kind = "behavior"
fixture = "L70-delegated-policy-valid-project-policy-violated"
status = "migrated_deleted"
reason = "behavior is now visible through generated replay baseline for the fixture"
```

Allowed `kind` values:

```text
behavior
replay_system
compile_contract
private_implementation_only
obsolete
```

Allowed `status` values:

```text
unclassified
migrated_deleted
kept_replay_system
kept_compile_contract
deleted_private_implementation
deleted_obsolete
```

Rules:

- `behavior` rows must end as `migrated_deleted`.
- `private_implementation_only` rows must end as `deleted_private_implementation`.
- `obsolete` rows must end as `deleted_obsolete`.
- `replay_system` rows may end as `kept_replay_system`.
- `compile_contract` rows may end as `kept_compile_contract`.

The ledger can be archived after migration is complete.

It must not become the long-term oracle.

## G3RS Fixture Levels

Use this initial clean source for G3RS fixtures:

```text
packages/rs/deny/g3rs-deny-config-checks
```

Reason:

```text
largest active Rust workspace by Rust file count
286 Rust files
300 tracked source/config files excluding target and legacy
4 Cargo.toml files
g3rs validate --path packages/rs/deny/g3rs-deny-config-checks --inventory exits 0
```

Do not use `apps/guardrail3-rs` as the first clean source.

It is smaller and has less package-local rule behavior.

Use `packages/rs/code/g3rs-code-source-checks` as the second source if the first fixture does not expose enough source-policy behavior.

Default placement for G3RS behavior:

```text
workspace root cannot be selected -> L00
Cargo workspace or package root exists, guardrail3-rs.toml missing -> L10
guardrail3-rs.toml invalid -> L20
Cargo.toml missing after config selects it -> L30
rust-toolchain.toml missing -> L30
rustfmt.toml missing -> L30
clippy.toml missing -> L30
deny.toml missing -> L30
nextest.toml missing -> L30
release-plz.toml missing -> L30
mutants.toml missing -> L30
Cargo.toml malformed -> L40
rust-toolchain.toml malformed -> L40
rustfmt.toml malformed -> L40
clippy.toml malformed -> L40
deny.toml malformed -> L40
nextest.toml malformed -> L40
release-plz.toml malformed -> L40
mutants.toml malformed -> L40
required cargo tools absent -> L50
required hook commands absent or weakened -> L60 validate-repo fixture
required config policy weakened -> L60
source policy violations -> L70
architecture policy violations -> L70
dependency policy violations with parseable manifests -> L70
clean package and clean repo behavior -> L80
```

Per-workspace commands cover:

```text
fmt
toolchain
clippy
deny
cargo
code
apparch
deps
garde
test
release
```

Repo commands cover:

```text
hooks
topology
tool presence
marker-pair completeness
repo-wide nesting
```

## G3TS Fixture Levels

Default placement for G3TS behavior:

```text
workspace root cannot be selected -> L00
package/adopted-unit root exists, guardrail3-ts.toml missing -> L10
guardrail3-ts.toml invalid -> L20
package.json missing after config selects it -> L30
tsconfig missing -> L30
eslint config missing -> L30
stylelint config missing -> L30
cspell config missing -> L30
syncpack config missing -> L30
astro config missing -> L30
package.json malformed -> L40
tsconfig malformed -> L40
eslint config malformed -> L40
stylelint config malformed -> L40
cspell config malformed -> L40
syncpack config malformed -> L40
astro config malformed -> L40
required npm/pnpm tools absent -> L50
required npm packages absent -> L50
delegated ESLint/Stylelint/Syncpack/Nuasite policy weakened -> L60
Astro content, MDX, SEO, media, i18n source violations -> L70
TS source/package/architecture policy violations -> L70
clean Astro app and clean TS package behavior -> L80
```

Per-workspace commands cover:

```text
astro setup
astro content
astro mdx
astro seo
astro media
astro i18n
astro state
eslint
fmt
spelling
style
typecov
tsconfig
jscpd
package
arch
apparch
```

Repo commands cover:

```text
hooks
topology
tool presence
marker-pair completeness
repo-wide nesting
```

## Migration Procedure

Run this procedure per package group.

```text
1. List every old test file and old test function in the package group.
2. Classify each row as behavior, replay_system, compile_contract, private_implementation_only, or obsolete.
3. For behavior rows, choose the earliest fixture level where the behavior is CLI-visible.
4. Mutate that fixture so the behavior is present.
5. Run replay with the accepted baseline binary.
6. Store generated normalized baseline output.
7. Run replay with the candidate binary.
8. Confirm candidate output matches baseline unless a behavior delta exists.
9. Delete old behavior tests represented by the fixture baseline.
10. Delete private implementation tests.
11. Delete obsolete tests.
12. Keep replay-system and compile-contract tests only in files that contain no old behavior tests.
13. Update the temporary ledger.
```

Do not write expected findings into the ledger.

Do not write expected messages into the ledger.

Do not write one manifest row per generated baseline output.

## Migration Order

Start with G3RS because its active package architecture is more stable and current project direction is Rust-first.

Order:

```text
1. apps/guardrail3-rs CLI behavior
2. packages/shared/g3-workspace-crawl
3. packages/parsers used by G3RS
4. packages/rs/topology
5. packages/rs/hooks
6. packages/rs/fmt
7. packages/rs/toolchain
8. packages/rs/cargo
9. packages/rs/clippy
10. packages/rs/deny
11. packages/rs/test
12. packages/rs/code
13. packages/rs/apparch
14. packages/rs/deps
15. packages/rs/garde
16. packages/rs/release
17. apps/guardrail3-ts CLI behavior
18. packages/parsers used only by G3TS
19. packages/ts/topology and hooks
20. packages/ts/tooling families
21. packages/ts/astro families
22. packages/ts/arch and apparch
```

Do not migrate a later package group until the previous group has:

```text
fixture baseline generated
old behavior tests deleted
private implementation tests deleted
obsolete tests deleted
remaining tests classified as replay_system or compile_contract
ledger has no unclassified rows for that package group
```

## Verifiers

Add deterministic verifiers for the migration.

```text
scripts/behavior/verify-ledger.py
scripts/behavior/verify-fixtures.py
scripts/behavior/verify-baselines.py
scripts/behavior/verify-test-deletion.py
scripts/behavior/verify-all.sh
```

Verifier requirements:

```text
verify-ledger: every old test file in migrated package groups is represented or explicitly excluded
verify-fixtures: every fixture.toml points at an existing repo and uses valid level names
verify-baselines: every fixture has generated baseline output for each listed command
verify-test-deletion: no old behavior test remains in migrated package groups
verify-all: runs all behavior migration verifiers
```

These verifiers check migration correctness.

They do not replace replay comparison.

They do not replace `g3rs validate`, `g3ts validate`, or cargo tests for replay-system crates.

## Baseline Rules

Baseline outputs are generated files.

Agents may not hand-edit them.

Accepted baseline metadata must include:

```toml
tool = "g3rs"
baseline_commit = "<git sha>"
fixture_hash = "sha256:<hash>"
runner_version = "1"
normalizer_version = "1"
output_schema_version = "1"
created_at = "<timestamp>"
```

If any metadata field mismatches, comparison fails closed.

## Deletion Rule

An old test file can be deleted only when every test in that file is one of:

```text
migrated_deleted
deleted_private_implementation
deleted_obsolete
```

If the file contains replay-system tests or compile-contract tests, move those tests to a replay-system or compile-contract test file before deleting old behavior tests.

Do not keep mixed files that contain both replay-system tests and old behavior tests.

## Done Definition

The migration is complete when:

```text
behavior/migration/g3rs-test-ledger.toml has zero unclassified rows
G3RS fixture baselines exist for every G3RS fixture command
old G3RS behavior tests are deleted
remaining G3RS tests are only replay_system or compile_contract
behavior/migration/g3ts-test-ledger.toml has zero unclassified rows
G3TS fixture baselines exist for every G3TS fixture command
old G3TS behavior tests are deleted
remaining G3TS tests are only replay_system or compile_contract
scripts/behavior/verify-all.sh exits 0
```
