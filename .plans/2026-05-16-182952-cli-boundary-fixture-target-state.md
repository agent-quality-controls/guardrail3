# CLI Boundary Fixture Target State

## Goal

Make fixture coverage target the real product boundary:

```text
fixture repo -> g3rs CLI/report command -> exit code + stdout + stderr + structured output
```

Do not build durable fixture infrastructure for internal ingestion output, rule input structs, or cross-crate implementation boundaries. The Rust crates are split to enforce code boundaries, not to create user-facing APIs.

## Current Fixture Inventory

Current `fixture3.yaml` suites:

- `g3rs-validate`
  - Uses `behavior/fixtures/g3rs/*/fixture.toml`
  - Current approved records: 55
  - Boundary: client-facing CLI validation output
  - Keep
- `g3rs-validate-repo`
  - Uses `behavior/fixtures/g3rs-validate-repo/*/fixture.toml`
  - Current approved records: 10
  - Boundary: client-facing repo-level CLI validation output
  - Keep
- `g3rs-cli-output`
  - Uses `behavior/fixtures/g3rs-cli-output/*/fixture.toml`
  - Current approved records: 12
  - Boundary: CLI help, rejected arguments, init behavior, command shape
  - Keep
- `g3rs-report-output`
  - Uses `behavior/fixtures/g3rs-report-output/*/fixture.toml`
  - Current approved records: 4
  - Boundary: report renderer output
  - Keep
- `g3rs-code-ingestion`
  - Uses two shared `behavior/fixtures/g3rs/*/fixture.toml` fixtures
  - Current approved records: 2
  - Boundary: internal ingestion struct JSON
  - Remove as durable test infrastructure

Current useful client-facing fixture records:

- `g3rs-validate`: 55
- `g3rs-validate-repo`: 10
- `g3rs-cli-output`: 12
- `g3rs-report-output`: 4
- Total client-facing records: 81

Current internal-only fixture records:

- `g3rs-code-ingestion`: 2

## Target State

### Fixture Suites To Keep

Keep only suites that execute a user-visible command or render a user-visible output:

- `g3rs-validate`
- `g3rs-validate-repo`
- `g3rs-cli-output`
- `g3rs-report-output`

These suites are valid because their approved output is the behavior a user or calling automation can observe.

### Fixture Suites To Remove

Remove:

- `g3rs-code-ingestion`

Reason:

- It calls an internal package-specific fixture binary.
- It serializes internal ingestion structs.
- Users never interact with ingestion output.
- The package split is an internal modularity boundary, not a public product contract.

### Fixture Packages To Remove

Remove:

```text
packages/rs/code/g3rs-code-ingestion/crates/fixture-output
```

Reason:

- It exists only to print internal ingestion data.
- It adds a durable internal CLI surface that the product does not need.
- It encourages multiplying the same pattern across 13 more ingestion packages.

### Fixture Scripts To Remove

Remove:

```text
scripts/behavior/fixture3-g3rs-code-ingestion.py
```

Reason:

- It exists only to run the internal code-ingestion fixture-output binary.
- Its output is not a product boundary.

### Golden Outputs To Remove

Remove:

```text
behavior/golden/g3rs-code-ingestion
```

Reason:

- It stores approved internal ingestion struct output.
- It is not useful after the ingestion fixture suite is removed.

### Plans To Retire

Do not continue these plans:

```text
.plans/2026-05-15-172324-code-ingestion-fixture-output.md
.plans/2026-05-15-172324-code-ingestion-fixture-output.md.manifest.toml
.plans/2026-05-16-173104-all-ingestion-serialized-fixtures.md
.plans/2026-05-16-173104-all-ingestion-serialized-fixtures.md.manifest.toml
```

Do not delete them automatically if historical plans are expected to remain immutable. Instead, add a short superseded marker at the top of each file or create a supersession index.

## Ledger Target State

The ledger must stop treating `needs_serialized_ingestion_output` as a valid migration target.

Current count:

- `needs_serialized_ingestion_output`: 421

Replace that disposition with one of:

- `needs_cli_fixture`
  - Use when behavior matters and can be exposed by `g3rs validate`, `g3rs validate repo`, CLI output, or report output.
- `covered_by_cli_output`
  - Use when an existing fixture already exposes the behavior in user-visible output.
- `needs_rule_fixture_or_golden_output`
  - Use when the behavior is a pure rule contract that is cheaper and clearer to test at the rule level.
- `keep_internal_unit_test`
  - Use only for dense algorithmic logic where CLI fixtures would be too broad to diagnose and the behavior is still important.
- `delete_internal_assertion`
  - Use when the assertion only verifies internal shape and no product behavior depends on it.

Forbidden disposition after rollback:

- `needs_serialized_ingestion_output`

## Review Of Current Fixtures

### `behavior/fixtures/g3rs/*`

Use these as the main layered product-behavior corpus.

These are useful because they exercise `g3rs validate` against concrete repo states:

- workspace root missing
- guardrail config missing or invalid
- required inputs missing
- invalid Rust tool configs
- missing delegated tools
- invalid delegated policy
- apparch/garde/release/workspace package policy violations
- clean workspace baseline

Do not split these by family unless one fixture hides another behavior.

### `behavior/fixtures/g3rs-validate-repo/*`

Keep.

These test repo-level adoption and hook behavior through the CLI:

- invalid repo root
- repo with no adoption
- hooks reachable without root Cargo
- weakened hooks
- invalid modular hook scripts
- path-qualified safe comments
- adoption marker policy
- complete root adoption
- default repo root behavior

### `behavior/fixtures/g3rs-cli-output/*`

Keep.

These test the CLI surface directly:

- help contract
- rejected old commands
- init-managed hooks
- init refusing owned hook
- workspace command shapes
- removed workspace arguments
- validate repo command shape

### `behavior/fixtures/g3rs-report-output/*`

Keep.

These test report rendering:

- visible warning with hidden inventory
- all results hidden
- rule message rendering
- scope root rendering

### `behavior/golden/g3rs-code-ingestion/*`

Remove.

These are internal ingestion snapshots. They should not be part of the durable fixture corpus.

## What To Roll Back

### Direct Removal

Remove internal serializer infrastructure:

- delete `packages/rs/code/g3rs-code-ingestion/crates/fixture-output`
- delete `scripts/behavior/fixture3-g3rs-code-ingestion.py`
- delete `behavior/golden/g3rs-code-ingestion`
- remove `g3rs-code-ingestion` from `fixture3.yaml`

### Cargo Cleanup

After deleting the fixture-output crate:

- remove the crate from `packages/rs/code/g3rs-code-ingestion/Cargo.toml` workspace members if listed
- remove related package entries from `Cargo.lock` through normal Cargo update/check behavior

### Verification Cleanup

Check these scripts for hardcoded expectations of `g3rs-code-ingestion`:

- `scripts/behavior/verify-all.sh`
- `scripts/behavior/verify-fixture3-migration.py`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-kept-test-dispositions.py`

If any require the code-ingestion suite, update them to require only client-facing suites.

### Ledger Cleanup

Update:

```text
behavior/migration/g3rs-kept-test-disposition.toml
```

Rules:

- remove every `needs_serialized_ingestion_output` disposition
- do not mark all 421 rows covered blindly
- classify each row by the target disposition listed above
- prefer CLI fixtures when the behavior can be observed in final output
- keep unit tests only when the behavior is internal but still has high diagnostic value

## What Not To Roll Back

Do not remove:

- `behavior/fixtures/g3rs/*`
- `behavior/golden/g3rs-validate`
- `behavior/fixtures/g3rs-validate-repo/*`
- `behavior/golden/g3rs-validate-repo`
- `behavior/fixtures/g3rs-cli-output/*`
- `behavior/golden/g3rs-cli-output`
- `behavior/fixtures/g3rs-report-output/*`
- `behavior/golden/g3rs-report-output`
- `scripts/behavior/fixture3-g3rs-fixture-replay.py`
- `scripts/behavior/fixture3-g3rs-report-output.py`
- `scripts/behavior/replay_common.py`

These target product-facing behavior.

## Replacement Testing Strategy

For every remaining test row:

1. Identify the behavior being protected.
2. Ask whether a user-visible command can expose it.
3. If yes, add or extend a CLI fixture.
4. If no, ask whether the behavior is worth preserving as a local unit test.
5. If no, delete the test row as internal-shape-only.

Valid product-facing surfaces:

- `g3rs validate workspace`
- `g3rs validate repo`
- CLI help and argument errors
- init command behavior
- inventory output
- hidden/visible report output
- exit code
- stdout
- stderr

Invalid durable surfaces:

- serialized ingestion structs
- rule input structs
- private helper return values
- traversal ordering unless it changes output
- internal classification unless visible through findings or inventory

## Verification Commands

After rollback:

```bash
fixture3 check --all
python3 scripts/behavior/verify-test-deletion.py
bash scripts/behavior/verify-all.sh
g3rs validate repo --path "$PWD"
git diff --check
```

Expected fixture3 suites after rollback:

- `g3rs-validate`
- `g3rs-validate-repo`
- `g3rs-cli-output`
- `g3rs-report-output`

Expected removed suite:

- `g3rs-code-ingestion`

