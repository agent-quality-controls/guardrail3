# G3TS/G3RS Infrastructure Parity

## Goal

Bring G3TS and G3RS to the same user-facing and internal infrastructure contract.

Rules and families stay language-specific. Everything else should have the same shape unless the language ecosystem forces a difference.

The target state is:

- same command model
- same help structure
- same init behavior
- same validate behavior
- same report shape
- same inventory semantics
- same family enablement semantics
- same waiver semantics
- same repo/workspace adoption semantics
- same hook generation structure
- same fixture coverage for the shared contract

## Current Evidence

The current code is close, but not at parity.

- G3TS top-level help is more complete than G3RS.
- G3RS report output includes `scope:` and `root:`. G3TS does not.
- G3RS `validate repo --inventory` can exit nonzero because satisfied inventory rows render as `[Error]`.
- G3TS `init repo` writes setup and prints a validation command. G3RS `init repo` writes setup and immediately runs validation.
- G3TS `init workspace` writes setup and returns. G3RS `init workspace` writes setup and immediately runs validation.
- G3TS repo validation is partly implemented in the CLI runtime wrapper.
- G3RS repo validation is implemented in `validate-command`.
- G3TS request types do not carry `staged`, `rules_only`, or repo `include_inventory` in the same way as G3RS.
- G3TS and G3RS both have family opt-out now, but the family-selection constants are not shaped the same way.

## Non-Goals

- Do not force rule-family parity.
- Do not rename language-specific families.
- Do not add TypeScript rules.
- Do not add Rust rules.
- Do not keep backwards compatibility for old command shapes.
- Do not preserve G3RS self-validating init behavior.
- Do not preserve G3RS inventory rows that report satisfied checks as errors.

## Target Contract

### CLI Surface

Both binaries expose exactly this high-level shape:

```text
<tool> init repo [--path <path>] [--force]
<tool> init workspace --path <path> [--force]
<tool> validate repo [--path <path>] [--inventory]
<tool> validate workspace --path <path> [--family <family>] [--inventory] [--staged] [--rules-only]
```

`--path` behavior:

- `init repo --path` defaults to `.`.
- `validate repo --path` defaults to Git repo root discovered from `.`.
- when `validate repo --path <path>` is given, the command resolves the Git repo root from that path.
- `init workspace --path` is required.
- `validate workspace --path` is required.

### Help Text

Both top-level help screens must use the same sections:

- one-line purpose
- ecosystem requirement
- adoption order
- workspace path choices
- concepts
- rules

Language-specific text is allowed only inside those sections.

G3TS ecosystem requirement:

- pnpm-managed TypeScript workspaces

G3RS ecosystem requirement:

- Cargo-managed Rust workspaces or package roots

Workspace path choices:

- root-only package: use `.`
- app with I/O: use `apps/<name>`
- library without I/O: use `packages/<name>`

G3RS must not keep "Deleted command shapes" in top-level help. Rejected command shapes belong in parser errors, not in the main contract.

### Init Behavior

`init` writes setup only.

`init` must not run validation after writing files.

Every init success output must end with the next validation command:

```text
validate with: <tool> validate repo --path <repo-root>
validate with: <tool> validate workspace --path <workspace-root>
```

`init repo` owns:

- `.githooks/`
- `.githooks/pre-commit`
- `.githooks/pre-commit.d/<tool>`
- `git config core.hooksPath=.githooks`

`init workspace` owns:

- language-specific guardrail config file
- language-specific tool policy files
- bounded package or manifest edits required by the language

### Validate Request Types

Both app-types crates must have the same conceptual request model:

- `InitRepoRequest`
- `InitWorkspaceRequest`
- `ValidateRepoRequest`
- `ValidateWorkspaceRequest`

`ValidateRepoRequest` fields:

- `repo_root: PathBuf`
- `include_inventory: bool`

`ValidateWorkspaceRequest` fields:

- `workspace_root: PathBuf`
- `families: Vec<SupportedFamily>`
- `include_inventory: bool`
- `staged: bool`
- `rules_only: bool`

G3TS must stop using the generic `ValidateRequest` name for workspace validation.

### Validate Execution Ownership

`validate-command` owns validation orchestration for both tools.

CLI runtime owns only:

- clap parsing
- converting CLI args into app request structs
- injecting runtime adapters
- converting `ExecutionOutcome` into process output

CLI runtime must not own:

- repo marker-pair checks
- repo workspace-adoption checks
- repo required-tool checks
- external toolchain gate orchestration
- exit-code policy

### Report Shape

Both tools use the same report structure:

- `scope: Option<&'static str>`
- `root: Option<PathBuf>`
- `runs: Vec<FamilyRun>`

Both plain-text renderers print `scope:` and `root:` when present.

Both renderers use the same no-findings behavior:

- if no visible rows exist, print `No findings.`
- inventory rows are visible only when `--inventory` is passed

### Inventory Exit Semantics

Inventory must never make a command fail by itself.

Exit code policy:

- non-inventory `Error` means exit `1`
- family runner error means exit `1`
- external gate failure means exit `1`
- inventory-only output means exit `0`
- `Warn` means exit `0` unless the row is a real non-inventory error encoded incorrectly

Satisfied checks must not render as `[Error]`.

For hook inventory specifically:

- actual missing hook file is an error
- actual missing `core.hooksPath` is an error
- actual missing dispatcher is an error
- satisfied presence inventory is info
- satisfied executable bit inventory is info
- satisfied shell safety inventory is info
- satisfied no-bypass inventory is info

### Family Enablement

Both tools use:

- default-on families
- `[checks].family = false` as the only opt-out
- explicit `--family` filter before opt-out removal
- repo-only family constants
- workspace-default family constants

Both tools must expose these concepts in code:

- `SUPPORTED_FAMILIES`
- `REPO_LEVEL_FAMILIES`
- `PER_WORKSPACE_DEFAULT_FAMILIES`
- `selected_families`
- `selected_families_with_opt_out`

### Hooks

Generated hooks stay language-specific, but structure must match.

Both generated managed hooks must:

- set `set -euo pipefail`
- read staged files
- exclude `behavior/fixtures/`
- scan staged files for merge conflict markers
- run `gitleaks protect --staged --no-banner`
- reject staged files over the same byte limit
- run `<tool> validate repo --path "$repo_root"`
- map relevant staged files to nearest owning adopted unit
- run `<tool> validate workspace --path "$unit" --staged`

Language-specific differences are limited to:

- owning marker pair: `package.json` + `guardrail3-ts.toml` vs `Cargo.toml` + `guardrail3-rs.toml`
- relevant file pattern
- package-manager lockfile step for G3TS
- `CARGO_TARGET_DIR` for G3RS

### Waivers

Both tools keep the current shared waiver behavior:

- rules do not know about waivers
- family runners do not decide waivers
- validate-command loads waivers from the workspace config
- validate-command applies waivers to family results
- waiver matching uses `rule`, `subject`, and `selector`

No additional waiver work is required unless parity changes break this.

## Implementation Plan

### Step 1: Add Parity Verifier

Add `scripts/verify-g3ts-g3rs-infrastructure-parity.py`.

The verifier must read this plan's manifest and check:

- command surfaces
- top-level help sections
- request type fields
- report type fields
- plain-text renderer scope/root support
- init does not call validate execution
- repo validation code lives in `validate-command`
- inventory exit fixtures exist
- generated hook templates contain required structural steps

The verifier must print `PASS` or `FAIL` with exact missing rows.

### Step 2: Align App Request Types

Modify:

- `apps/guardrail3-ts/crates/types/app-types/src/request.rs`
- `apps/guardrail3-rs/crates/types/app-types/src/request.rs`

Target:

- both use `ValidateWorkspaceRequest`
- both use `ValidateRepoRequest { repo_root: PathBuf, include_inventory: bool }`
- both workspace requests include `staged` and `rules_only`

### Step 3: Move G3TS Repo Execution Into Validate-Command

Modify:

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`

Target:

- G3TS validate-command exposes `execute_workspace`
- G3TS validate-command exposes `execute_repo`
- G3TS CLI runtime does not run repo adoption, marker-pair, or tool-presence checks itself
- G3TS CLI runtime only builds requests and delegates

If names cannot be changed cleanly without broad churn, keep public names but make ownership match the target.

### Step 4: Remove G3RS Init Self-Validation

Modify:

- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/init.rs`

Target:

- `execute_init_repo` writes setup and prints `validate with: g3rs validate repo --path <repo-root>`
- `execute_init_workspace` writes setup and prints `validate with: g3rs validate workspace --path <workspace-root>`
- no call to `execute_repo`
- no call to `execute`
- no `crawler`, `family_runner`, or `renderer` needed by init functions

Then align the G3RS CLI runtime call shape with G3TS.

### Step 5: Add Scope/Root To G3TS Reports

Modify:

- `apps/guardrail3-ts/crates/types/app-types/src/report.rs`
- `apps/guardrail3-ts/crates/io/outbound/report/crates/runtime/src/plain_text.rs`

Target:

- G3TS `ValidateReport` matches G3RS structure
- G3TS renders `scope:` and `root:`
- G3TS uses `ValidateReport::scoped(...)` in workspace and repo validation

### Step 6: Fix Inventory Exit Semantics

Modify both validate-command runtimes as needed:

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs`

Target:

- exit code ignores inventory-only rows
- `--inventory` cannot turn a clean command into a failing command
- `g3rs validate repo --path . --inventory` exits `0` when the non-inventory command exits `0`

Fix G3RS hook checks that emit satisfied inventory rows as errors.

The correct fix is in the rule output severity, not only in exit-code filtering.

### Step 7: Align Family Selection Constants

Modify:

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/selection.rs`

Target:

- both have `REPO_LEVEL_FAMILIES`
- both have `PER_WORKSPACE_DEFAULT_FAMILIES`
- both `selected_families` implementations use the same structure

### Step 8: Align Help Text

Modify:

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`

Target:

- same section order
- same language
- language-specific ecosystem line only
- no deleted command list in G3RS help
- `validate repo --path` help says the same thing in both tools

### Step 9: Add Behavior Fixtures

Add or update fixtures under:

- `behavior/fixtures/g3ts-rule/family-enablement`
- `behavior/fixtures/g3rs-rule/family-enablement`

Required fixture coverage:

- missing workspace config fails before families run
- disabled family does not run
- disabled family hook contract does not run
- `validate repo --inventory` does not fail on inventory-only output
- `validate workspace --inventory --rules-only` prints scope/root and exits `0` for clean workspace

Do not create one fixture per implementation detail. Add the minimum fixtures that expose the public contract.

### Step 10: Verification

Required commands:

```bash
python3 scripts/verify-g3ts-g3rs-infrastructure-parity.py
cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all -- --check
cargo fmt --manifest-path apps/guardrail3-rs/Cargo.toml --all -- --check
cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings
fixture3 check --all --json
g3ts validate repo --path .
g3rs validate repo --path .
g3ts validate repo --path . --inventory
g3rs validate repo --path . --inventory
```

## Files Expected To Change

- `apps/guardrail3-ts/crates/types/app-types/src/request.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/report.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection.rs`
- `apps/guardrail3-ts/crates/io/outbound/report/crates/runtime/src/plain_text.rs`
- `apps/guardrail3-rs/crates/types/app-types/src/request.rs`
- `apps/guardrail3-rs/crates/types/app-types/src/report.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/selection.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs`
- G3RS hook rule crates that currently emit satisfied inventory as `Error` or `Warn`
- family-enablement fixtures for both tools
- `scripts/verify-g3ts-g3rs-infrastructure-parity.py`

## Definition Of Done

- manifest verifier passes
- both repo inventory commands exit `0` on this repo
- both plain-text outputs include scope/root consistently
- both init commands only write and print next validation command
- both validation commands use the same request model
- both CLIs show the same help structure
- both generated hooks keep language-specific differences only where listed above
- `fixture3 check --all --json` passes
- `g3ts validate repo --path .` passes
- `g3rs validate repo --path .` passes

