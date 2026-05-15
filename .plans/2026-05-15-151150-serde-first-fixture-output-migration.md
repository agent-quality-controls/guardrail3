# Goal

Replace the remaining test-only behavior coverage with fixture3 outputs without creating adapters, exporters, ingestion suites, replay suites, replay record maps, or extra canonical fact layers.

The default implementation is:

- owned Rust output structs derive `serde::Serialize`
- verifier commands serialize those structs with `serde_json`
- fixture3 compares received JSON to approved JSON

# Scope

This plan covers the remaining rows in `behavior/migration/g3rs-kept-test-disposition.toml`.

Current disposition counts:

- `needs_serialized_ingestion_output`: 420 rows
- `needs_rule_fixture_or_golden_output`: 236 rows
- `needs_family_runner_output`: 47 rows
- `needs_validate_command_output`: 23 rows
- `needs_cli_output`: 3 rows
- `needs_renderer_output`: 3 rows
- `keep_public_api_contract`: 13 rows

# Non-Negotiable Architecture Rules

- Do not write per-family adapters.
- Do not write per-family exporters.
- Do not introduce ingestion suites.
- Do not introduce replay suites.
- Do not introduce replay record maps.
- Do not introduce canonical fact layers.
- Do not normalize family-specific values in Python.
- Do not map Rust structs into parallel Python dictionaries except for the existing generic command envelope.
- Do not hand-code JSON field selection for owned Rust structs.

# Fixture Output Contract

Each verifier command produces JSON.

Allowed JSON shapes:

- generic command envelope for CLI-level fixture3 runs
- serialized Rust struct output for ingestion, family-runner, validate-command, and renderer behavior
- rule finding output when the public behavior is already a `g3rs validate` finding

The generic command envelope may contain:

- fixture id
- fixture hash
- command argv
- cwd
- exit code
- stdout
- stderr

It must not contain family-specific remapping logic.

# Serde-First Rule

For every owned Rust struct or enum that needs to appear in fixture output:

- derive `serde::Serialize`
- add `serde` with `derive` feature to the crate that owns the type if missing
- add `serde_json` only to the verifier or test-support crate that emits JSON
- keep serialization derives on the public type, not on a duplicate fixture-only type

If deriving `serde::Serialize` fails:

- stop
- document the exact type path
- document the exact field that blocks serialization
- document whether the field is third-party, private, lifetime-bound, or intentionally non-serializable
- only then decide whether a narrow conversion is necessary

Any narrow conversion must be named as a documented exception, not an adapter/exporter layer.

# Output Stability Rules

Allowed generic normalization:

- absolute repository paths to `$REPO`
- fixture paths to `$FIXTURE`
- temporary target paths to `$TARGET`
- elapsed times to `$TIME`
- Rust build hashes to `$HASH`
- path separators to `/`

Forbidden normalization:

- changing family-specific field names
- deleting fields from owned Rust structs because a fixture would be noisy
- replacing typed output with simplified fixture-only schemas
- sorting a field unless the semantic type is a set or unordered map

If output order is unstable, fix the Rust type or producer so it emits deterministic order.

# Migration Order

## 1. Rule Fixture Output

Rows: `needs_rule_fixture_or_golden_output`.

Use existing `g3rs validate` fixture output when the behavior can surface as a finding without hidden earlier failures.

Only use rule-level golden output when:

- the rule is pure and has no CLI-visible fixture state that can expose the same behavior
- the output is produced by serializing the rule's actual public finding/input type
- no fixture-only simplified schema is created

## 2. Serialized Ingestion Output

Rows: `needs_serialized_ingestion_output`.

Implementation shape:

- create a Rust verifier binary or subcommand that runs the same ingestion code used by the family orchestrator
- emit the actual owned ingestion output structs as JSON with `serde_json`
- derive `serde::Serialize` on every owned ingestion output type needed by that JSON
- make fixtures exercise parse success, parse failure, missing files, malformed files, path discovery, and fail-closed behavior

This verifier must not:

- inspect source files in Python
- parse Rust/TOML/JSON/YAML in Python beyond reading fixture metadata
- create parallel Python representations of ingestion outputs
- call rule functions directly

## 3. Family Runner Output

Rows: `needs_family_runner_output`.

Implementation shape:

- run the real family runner on fixture repositories
- serialize the runner's owned aggregation output with `serde_json`
- cover fan-out, inactive families, family selection, hook contract aggregation, and duplicate/merged findings

If the current runner output type is not serializable, make it serializable.

## 4. Validate Command Output

Rows: `needs_validate_command_output`.

Implementation shape:

- run the real validate command entry point against fixture repositories
- keep command stdout/stderr/exit where the public contract is CLI behavior
- serialize owned validate-command decision structs where the current tests assert internal decisions
- cover cargo gates, staged paths, family selection, workspace routing, and delegated command failures

## 5. CLI Output

Rows: `needs_cli_output`.

Implementation shape:

- use fixture3 command output snapshots
- cover argument parsing, invalid flags, help/version output, and rejected path shapes
- no custom structs are needed unless the CLI already exposes typed output

## 6. Renderer Output

Rows: `needs_renderer_output`.

Implementation shape:

- serialize renderer inputs with `serde_json` if the tests are about input handling
- snapshot rendered text if the tests are about exact report formatting
- do not create fixture-only renderer models

## 7. Public API Contracts

Rows: `keep_public_api_contract`.

Keep these as compile/API tests until a real public API snapshot exists.

A public API snapshot is allowed only if it is generated from real Rust public metadata, not manually maintained.

# Required Mechanical Checks

- `scripts/behavior/verify-fixture-contract-language.py` must keep blocking forbidden architecture terms.
- A new verifier must fail if a kept-test disposition name is not one of the approved output-boundary names.
- A new verifier must fail if future plans mention adapters/exporters for fixture output.
- `scripts/behavior/verify-all.sh` must run every behavior migration verifier.

# Done Criteria

The migration is done only when:

- `behavior/migration/g3rs-kept-test-disposition.toml` has zero rows that need fixture output
- all replaced tests are represented by fixture3 approved outputs or documented public API contracts
- every owned Rust output type used by fixture verifier commands derives `serde::Serialize`
- every non-serializable exception has a documented type path and reason
- `scripts/behavior/verify-all.sh` passes
- `fixture3 check --all` passes
