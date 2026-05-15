# Goal

Audit every `kept_compile_contract` row in `behavior/migration/g3rs-test-fixture-ledger.toml` so the remaining tests are not hidden behind one vague bucket.

# Approach

- Keep the existing fixture ledger as the current behavior replay mapping.
- Add a second generated ledger for rows that still have `status = "kept_compile_contract"`.
- Classify each kept row into one concrete migration disposition.
- Add a verifier that fails if any kept row lacks a disposition or if disposition counts drift.
- Wire the verifier into `scripts/behavior/verify-all.sh`.

# Dispositions

- `needs_rule_fixture_or_golden_output`
  - Direct rule sidecar tests that still need fixture or golden replay coverage.
  - These are not true compile/API contracts.
- `needs_serialized_ingestion_output`
  - Ingestion tests that validate parsing, normalization, fail-closed input handling, and family fact extraction.
  - These need fixture output that serializes the owned Rust ingestion structs with Serde, not a custom adapter layer.
- `needs_family_runner_output`
  - Family runner and `run_tests` tests that validate fan-out and aggregation.
  - These need runner-level replay output or fixture states that prove dispatch.
- `needs_validate_command_output`
  - `validate-command` tests for cargo gates, staged paths, family selection, and command execution.
  - These need CLI command replay, not rule fixture output.
- `needs_cli_output`
  - CLI parse/error behavior.
  - These need CLI stdout/stderr/exit snapshots.
- `needs_renderer_output`
  - Plain text report rendering.
  - These need renderer snapshot replay.
- `keep_public_api_contract`
  - Hook-contract crate public policy tests.
  - These remain compile/API contracts unless replaced by public API snapshot verification.

# Files to Modify

- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `scripts/behavior/verify-kept-test-dispositions.py`
- `scripts/behavior/verify-all.sh`

# Verification

- `python3 scripts/behavior/classify-kept-test-dispositions.py --check`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `scripts/behavior/verify-all.sh`
