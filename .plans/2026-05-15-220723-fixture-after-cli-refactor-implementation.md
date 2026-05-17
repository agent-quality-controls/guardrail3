# Goal

Continue fixture migration after the committed CLI refactor without reintroducing obsolete CLI shapes.

This stage must make current verifier status truthful and add fixture3 coverage for the new CLI command surface.

# Current Evidence

- `g3rs --help` exposes `init` and `validate`.
- `g3rs validate --help` exposes `repo` and `workspace`.
- `g3rs init --help` exposes `repo` and `workspace`.
- Old command shapes are intentionally rejected:
  - `g3rs validate-repo`
  - `g3rs validate --path <path>`
- Existing fixture3 suites already run the new command shape:
  - `g3rs-validate`
  - `g3rs-validate-repo`
  - `g3rs-code-ingestion`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py` currently prints replaced hook rows as `planned`.

# Approach

## 1. Coverage Verifier Semantics

Modify `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`.

Required behavior:

- count `coverage_status = "covered"` as covered
- count `coverage_status = "replaced_by_managed_hook"` as replaced
- count all other non-covered rows as planned
- print:
  - `behavior-rule-coverage: PASS source:<n> covered:<n> replaced:<n> planned:<n>`
- fail if any replaced row does not name an active replacement rule in `reason`
- fail if `g3rs-hooks/managed-g3rs-hook-chain` is not covered by replay output while replaced hook rows exist
- allow the replaced row to name the fixture where the replacement rule is exercised

Why:

- `replaced_by_managed_hook` is not a future task.
- It is a terminal state for obsolete hook checks superseded by the managed hook chain rule.
- Calling it `planned` makes the verifier output untrustworthy.

## 2. Coverage Matrix

Update `behavior/coverage/g3rs-rule-coverage.toml` only if verifier semantics require clearer replacement metadata.

No rule row should be moved to `covered` unless a fixture emits it.

No old hook rule should be revived only to satisfy old tests.

## 3. CLI Output Fixture Suite

Add a new fixture3 suite named `g3rs-cli-output`.

Target files:

- `fixture3.yaml`
- `behavior/fixtures/g3rs-cli-output/*/fixture.toml`
- `behavior/golden/g3rs-cli-output/approved.normalized.json`

The suite must use `scripts/behavior/fixture3-g3rs-fixture-replay.py`.

The suite must snapshot command output, not parse CLI internals.

Minimum fixtures:

- `C10-help-contract`
  - commands:
    - `["--help"]`
    - `["validate", "--help"]`
    - `["init", "--help"]`
  - validates public command names and top-level help output
- `C20-reject-old-command-shapes`
  - commands:
    - `["validate-repo"]`
    - `["validate", "--path", "."]`
  - validates deleted CLI shapes fail visibly
- `C30-init-repo-managed-hooks`
  - command:
    - `["init", "repo", "--path", "."]`
  - validates repo init output and managed hook filesystem effects through command output and fixture hash
- `C40-init-repo-refuses-owned-hook`
  - command:
    - `["init", "repo", "--path", "."]`
  - fixture has a project-owned hook before command
  - validates init refuses to overwrite without force
- `C50-workspace-command-shapes`
  - commands:
    - `["init", "workspace", "--path", "."]`
    - `["validate", "workspace", "--path", ".", "--family", "fmt", "--inventory"]`
  - validates accepted workspace init and selected-family inventory command shapes
- `C60-reject-removed-workspace-arguments`
  - commands:
    - `["init", "workspace", "--path", ".", "--profile", "library"]`
    - `["validate", "workspace", "--path", ".", "--family", "hexarch"]`
  - validates deleted init profile flag and removed family names fail at the CLI surface
- `C70-validate-repo-command-shape`
  - command:
    - `["validate", "repo", "--path", "."]`
  - validates the accepted repo validation command shape

Do not create adapters, replay suites, or typed CLI output structs.

## 4. Renderer Output Fixture Suite

Add a new fixture3 suite named `g3rs-report-output`.

Target files:

- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/main.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/mod.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/fixture_output.rs`
- `scripts/behavior/fixture3-g3rs-report-output.py`
- `fixture3.yaml`
- `behavior/fixtures/g3rs-report-output/*/fixture.toml`
- `behavior/golden/g3rs-report-output/approved.normalized.json`

The suite must snapshot rendered text from the real `PlainTextReportRenderer`.
The fixture binary must stay inside the existing report assertions package.
Do not add a separate sibling fixture-output crate under `io/outbound/report/crates`, because that creates a forbidden io-to-io dependency.
The Python fixture3 command wrapper owns JSON output so the Rust assertions package does not need `serde` or `serde_json` only for replay transport.
The report fixture module directory keeps a facade-only `mod.rs`; the Cargo binary entry point is `main.rs`.

Minimum fixtures:

- `P10-hidden-inventory-with-visible-warning`
  - validates inventory rows are hidden by default while visible warning rows remain
- `P20-all-results-hidden`
  - validates the report prints `No findings.` when all rows are hidden inventory
- `P30-rule-message`
  - validates severity, rule ID, file, title, and message formatting
- `P40-scope-root`
  - validates scope and root header formatting

Do not call private renderer functions.

## 5. Ledger Reclassification

After the CLI fixture suite is approved:

- regenerate `behavior/migration/g3rs-test-fixture-ledger.toml`
- regenerate `behavior/migration/g3rs-kept-test-disposition.toml`
- update fixture-related manifests if the verifier requires suite inventory updates

Expected movement:

- CLI rows covered by command output move out of `needs_cli_output`
- renderer rows covered by report output move out of `needs_renderer_output`
- rows that are compile-only or public API-only remain kept

## 6. Verification

Required commands:

- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `bash scripts/behavior/verify-all.sh`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-report-assertions`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-report-assertions --all-targets -- -D warnings`
- `g3rs validate workspace --path apps/guardrail3-rs --family cargo --family apparch --family test --inventory`

# Non-Goals

- Do not modify Rust CLI behavior unless fixture output reveals a real bug.
- Do not touch TypeScript code.
- Do not revive old hook checks.
- Do not implement renderer, validate-command, family-runner, or new ingestion suites in this commit.
