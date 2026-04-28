# Goal

Implement Rust hook contracts for all G3RS families so hook requirements are owned by the families that require them, not hardcoded inside `g3rs-hooks`.

Desired end state:

- every Rust validation family that needs pre-commit enforcement has a `g3rs-<family>-hook-contract` package
- each hook-contract package exposes exactly one parameterless function:

```rust
pub fn hook_contract() -> Vec<G3HookRequirement>;
```

- `g3rs-hooks` aggregates family hook contracts and verifies the effective hook against them
- existing hardcoded hook rules are removed only after the matching contract-driven replacement is tested
- ingestion still only consumes outside-world facts
- checks still only validate their own lane
- hook-contract packages own hook policy

# Current Rust Hook State

Current G3RS hook implementation:

- `packages/rs/hooks/g3rs-hooks-types`
- `packages/rs/hooks/g3rs-hooks-ingestion`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks`
- `packages/rs/hooks/g3rs-hooks-source-checks`
- `packages/rs/hooks/g3rs-hooks-config-checks`

Current app wiring:

- hooks run through `apps/guardrail3-rs/crates/logic/family-runner-process/src/run.rs`
- `SupportedFamily::Hooks` calls:
  - `g3rs_hooks_ingestion::ingest_for_config_checks`
  - `g3rs_hooks_ingestion::ingest_for_file_tree_checks`
  - `g3rs_hooks_ingestion::ingest_for_source_checks`
  - `g3rs_hooks_config_checks::check`
  - `g3rs_hooks_file_tree_checks::check`
  - `g3rs_hooks_source_checks::check`

Current hardcoded Rust hook rules inside `g3rs-hooks-source-checks`:

- `fmt_step_present`
- `clippy_step_present`
- `cargo_deny_step_present`
- `test_step_present`
- `cargo_machete_step_present`
- `duplication_tool_is_cargo_dupes`
- `guardrail_validate_staged_present`
- `clippy_denies_warnings`
- `test_uses_workspace`
- `gitleaks_step_present`
- `cargo_dupes_step_present`
- `cargo_dupes_excludes`
- `config_changes_trigger_validation`
- `shared_target_dir_present`

Current hardcoded installed-tool checks inside `g3rs-hooks-config-checks`:

- `required_tools_installed`
- `guardrail_binary_available`
- `cargo_dupes_installed`

# Architecture

## New Shared Hook Contract Types

Add a shared hook contract type package.

Preferred location:

- `packages/shared/hooks/g3-hooks-contract-types`

If the shared package wiring is too large for the first patch, use this temporary Rust-local location and migrate it immediately after the first family works:

- `packages/rs/hooks/g3rs-hooks-contract-types`

Types:

```rust
pub struct G3HookRequirement {
    pub id: String,
    pub owner_family: String,
    pub trigger_patterns: Vec<G3HookTriggerPattern>,
    pub required_commands: Vec<G3HookCommandRequirement>,
    pub critical_commands: Vec<G3HookCriticalCommand>,
}

pub enum G3HookTriggerPattern {
    ExactPath(String),
    Glob(String),
    Extension(String),
}

pub enum G3HookCommandRequirement {
    CargoFmtCheck,
    CargoClippyDenyWarnings,
    CargoDenyCheck,
    CargoTest,
    CargoMachete,
    CargoDupes,
    CargoDupesExcludeTests,
    Gitleaks,
    G3RsValidatePath,
    SharedTargetDirSetup,
}

pub enum G3HookCriticalCommand {
    Binary(String),
    CargoSubcommand(String),
}
```

Do not add parser logic to this package.

Do not put rule logic into this package.

## Family Hook Contract Package Shape

Each Rust family that needs hooks gets:

```text
packages/rs/<family>/g3rs-<family>-hook-contract/
  Cargo.toml
  src/lib.rs
  crates/assertions/
  crates/runtime/
  crates/types/
```

If a hook-contract package is only a small facade at first, it still follows the package pattern. Do not put contract functions into existing `types`, `ingestion`, or `*-checks` packages.

Public API:

```rust
pub fn hook_contract() -> Vec<G3HookRequirement>;
```

No parameters.

No `static_hook_contract`.

No `parameterized_hook_contract`.

No optional params.

# Contract Inventory By Family

## `g3rs-fmt-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `rustfmt.toml`
  - `.rustfmt.toml`
  - `Cargo.toml`
- required command:
  - `CargoFmtCheck`
- critical commands:
  - `cargo fmt`

Replaces hardcoded rule:

- `g3rs-hooks/fmt-step-present`

## `g3rs-clippy-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `Cargo.toml`
  - `Cargo.lock`
  - `clippy.toml`
  - `.clippy.toml`
  - `rust-toolchain.toml`
- required commands:
  - `CargoClippyDenyWarnings`
- critical commands:
  - `cargo clippy`

Replaces hardcoded rules:

- `g3rs-hooks/clippy-step-present`
- `g3rs-hooks/clippy-denies-warnings`

## `g3rs-deny-hook-contract`

Requirements:

- trigger patterns:
  - `deny.toml`
  - `Cargo.toml`
  - `Cargo.lock`
- required command:
  - `CargoDenyCheck`
- critical commands:
  - `cargo deny`
  - `cargo-deny`

Replaces hardcoded rule:

- `g3rs-hooks/cargo-deny-step-present`

Also feeds installed-tool checks for:

- `cargo-deny`

## `g3rs-cargo-hook-contract`

Requirements:

- trigger patterns:
  - `Cargo.toml`
  - `Cargo.lock`
  - `.cargo/config.toml`
  - `.cargo/config`
- required command:
  - lockfile integrity command through existing concrete lockfile rule
- critical commands:
  - `cargo`

Does not replace a single old Rust command rule directly.

Feeds shared hook rules:

- `g3rs-hooks/concrete-lockfile-command`

## `g3rs-test-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `Cargo.toml`
  - `Cargo.lock`
- required commands:
  - `CargoTest`
- advisory command:
  - workspace projects should use `cargo test --workspace`
- critical commands:
  - `cargo test`

Replaces hardcoded rules:

- `g3rs-hooks/test-step-present`
- `g3rs-hooks/test-uses-workspace`

The current workspace/single-crate distinction needs input. Do not parameterize `hook_contract()` yet.

First slice:

- keep `test-uses-workspace` hardcoded until a separate design decides how hook contracts express advisory conditions.
- move only unconditional `CargoTest` into `g3rs-test-hook-contract`.

## `g3rs-deps-hook-contract`

Requirements:

- trigger patterns:
  - `Cargo.toml`
  - `Cargo.lock`
- required commands:
  - `CargoMachete`
  - `CargoDupes`
  - `CargoDupesExcludeTests`
- critical commands:
  - `cargo machete`
  - `cargo-machete`
  - `cargo dupes`
  - `cargo-dupes`

Replaces hardcoded rules:

- `g3rs-hooks/cargo-machete-step-present`
- `g3rs-hooks/duplication-tool-is-cargo-dupes`
- `g3rs-hooks/cargo-dupes-step-present`
- `g3rs-hooks/cargo-dupes-excludes`

Also feeds installed-tool checks for:

- `cargo-machete`
- `cargo-dupes`

## `g3rs-code-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `guardrail3-rs.toml`
  - `clippy.toml`
  - `deny.toml`
  - `rustfmt.toml`
  - `rust-toolchain.toml`
  - `Cargo.toml`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Replaces hardcoded rule:

- `g3rs-hooks/guardrail-validate-staged-present`

The old rule message says `g3rs validate --path ...`, not `--staged`. Keep the current accepted command shape unless a separate staged-mode CLI exists.

## `g3rs-garde-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `guardrail3-rs.toml`
  - `Cargo.toml`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Does not add a separate command beyond `g3rs validate`.

Contributes trigger coverage to the generic contract-driven trigger rule.

## `g3rs-apparch-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `guardrail3-rs.toml`
  - `Cargo.toml`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Does not add a separate command beyond `g3rs validate`.

Contributes trigger coverage to the generic contract-driven trigger rule.

## `g3rs-arch-hook-contract`

Requirements:

- trigger patterns:
  - `**/*.rs`
  - `guardrail3-rs.toml`
  - `Cargo.toml`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Contributes trigger coverage to the generic contract-driven trigger rule.

## `g3rs-toolchain-hook-contract`

Requirements:

- trigger patterns:
  - `rust-toolchain.toml`
  - `Cargo.toml`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Contributes trigger coverage to the generic contract-driven trigger rule.

## `g3rs-release-hook-contract`

Requirements:

- trigger patterns:
  - release config files owned by `g3rs-release`
  - `Cargo.toml`
  - `Cargo.lock`
- required command:
  - `G3RsValidatePath`
- critical commands:
  - `g3rs`

Do not invent exact release config path list without reading the release family implementation during implementation.

# Hooks Package Changes

## Types

Update `packages/rs/hooks/g3rs-hooks-types`:

- add `requirements: Vec<G3HookRequirement>` to `G3RsHooksSourceChecksInput`
- add `requirements: Vec<G3HookRequirement>` to `G3RsHooksConfigChecksInput`
- keep file-tree input unchanged

If adding full requirement vectors to every script input creates duplication, add:

```rust
pub struct G3RsHooksSourceChecksBundle {
    pub scripts: Vec<G3RsHooksSourceChecksInput>,
    pub requirements: Vec<G3HookRequirement>,
}
```

Prefer the smallest change that keeps rule inputs explicit.

## Ingestion

Do not make `g3rs-hooks-ingestion` define family policy.

Keep it responsible for:

- effective hook selection
- hook content reading
- `hook-shell-parser`
- file metadata
- installed tools from `PATH`
- trust risks

## Orchestration

Update `apps/guardrail3-rs/crates/logic/family-runner-process/src/run.rs`.

For `SupportedFamily::Hooks`:

1. collect hook requirements from family hook-contract packages
2. ingest hook config/file-tree/source facts
3. pass requirements to config/source hook checks

Initial function shape:

```rust
fn rust_hook_requirements() -> Vec<G3HookRequirement> {
    [
        g3rs_fmt_hook_contract::hook_contract(),
        g3rs_clippy_hook_contract::hook_contract(),
        g3rs_deny_hook_contract::hook_contract(),
        g3rs_cargo_hook_contract::hook_contract(),
        g3rs_test_hook_contract::hook_contract(),
        g3rs_deps_hook_contract::hook_contract(),
        g3rs_code_hook_contract::hook_contract(),
        g3rs_garde_hook_contract::hook_contract(),
        g3rs_apparch_hook_contract::hook_contract(),
        g3rs_arch_hook_contract::hook_contract(),
        g3rs_toolchain_hook_contract::hook_contract(),
        g3rs_release_hook_contract::hook_contract(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
```

This is orchestration, not ingestion.

# New Generic Hook Rules

Add to `g3rs-hooks-source-checks`:

- `g3rs-hooks/required-contract-command-present`
- `g3rs-hooks/contract-trigger-coverage`
- `g3rs-hooks/contract-critical-command-not-fail-open`

## `required-contract-command-present`

For every `G3HookRequirement.required_commands`, prove the parsed effective pre-commit hook contains an equivalent executable command.

Use `hook-shell-parser` command facts.

Do not count:

- comments
- `echo`
- inert text

## `contract-trigger-coverage`

For every `G3HookRequirement.trigger_patterns`, prove staged-file trigger logic routes that pattern to the required command.

This is the weakest current area.

Implementation must not use raw substring matching as the final architecture.

First implementation can be conservative:

- if trigger logic is not parseable enough, emit warning explaining that the hook cannot prove config-only changes trigger validation
- keep existing `config_changes_trigger_validation` until this rule is strong enough to replace it

## `contract-critical-command-not-fail-open`

Replace hardcoded critical command list in `no_fail_open_wrappers` with aggregate `critical_commands` from all hook contracts plus universal commands:

- `g3rs`
- `gitleaks`

Keep old `no_fail_open_wrappers` until contract-driven rule has parity tests.

# Migration Order

## Slice 1: Shared Types And One Family

Goal:

- prove architecture with `g3rs-fmt-hook-contract`
- do not remove old hardcoded fmt rule yet

Tasks:

1. Add hook contract type package.
2. Add `packages/rs/fmt/g3rs-fmt-hook-contract`.
3. Add unit tests for `hook_contract()`.
4. Wire `g3rs-fmt-hook-contract` into `family-runner-process`.
5. Add `requirements` to hook source input.
6. Add `required-contract-command-present` supporting `CargoFmtCheck`.
7. Add tests proving:
   - real `cargo fmt --check` satisfies contract
   - comment mentioning `cargo fmt --check` does not satisfy contract
   - `echo "cargo fmt --check"` does not satisfy contract
   - missing command reports `g3rs-hooks/required-contract-command-present`
8. Run full G3RS tests.

## Slice 2: Clippy And Deny

Tasks:

1. Add `g3rs-clippy-hook-contract`.
2. Add `CargoClippyDenyWarnings` command matcher.
3. Add `g3rs-deny-hook-contract`.
4. Add `CargoDenyCheck` command matcher.
5. Keep old hardcoded rules.
6. Add parity tests against existing hardcoded `clippy_step_present`, `clippy_denies_warnings`, and `cargo_deny_step_present`.

## Slice 3: Test And Deps

Tasks:

1. Add `g3rs-test-hook-contract`.
2. Add `CargoTest` command matcher.
3. Add `g3rs-deps-hook-contract`.
4. Add command matchers:
   - `CargoMachete`
   - `CargoDupes`
   - `CargoDupesExcludeTests`
5. Keep `test_uses_workspace` hardcoded until parameterization is explicitly designed.
6. Add parity tests for existing test/deps hook rules.

## Slice 4: G3RS Validate Families

Tasks:

1. Add hook-contract packages:
   - `g3rs-code-hook-contract`
   - `g3rs-garde-hook-contract`
   - `g3rs-apparch-hook-contract`
   - `g3rs-arch-hook-contract`
   - `g3rs-toolchain-hook-contract`
   - `g3rs-release-hook-contract`
2. Add `G3RsValidatePath` command matcher.
3. Aggregate duplicate `G3RsValidatePath` requirements into one hook finding where possible, but keep owner-family details in the message.
4. Add tests proving one valid `g3rs validate --path ...` can satisfy multiple family requirements.

## Slice 5: Installed Tools From Contracts

Tasks:

1. Replace hardcoded installed tool checks with contract-derived tool requirements.
2. Keep universal tools:
   - `gitleaks`
   - `g3rs`
3. Contract-derived tools:
   - `cargo-deny`
   - `cargo-machete`
   - `cargo-dupes`
4. Remove old hardcoded installed-tool rules only after parity tests pass.

## Slice 6: Trigger Coverage

Tasks:

1. Design typed staged-file trigger extraction from parsed shell facts.
2. Implement `contract-trigger-coverage`.
3. Migrate current `config_changes_trigger_validation` to contract-driven trigger coverage.
4. Remove old rule after parity plus stricter tests pass.

This is intentionally last because it is the hardest and current G3RS behavior is weak.

## Slice 7: Remove Replaced Hardcoded Rules

Remove hardcoded rules only when contract-driven replacements have:

- unit tests
- integration tests through hooks ingestion
- same or stricter findings on representative hooks

Candidate removals:

- `fmt_step_present`
- `clippy_step_present`
- `cargo_deny_step_present`
- `test_step_present`
- `cargo_machete_step_present`
- `duplication_tool_is_cargo_dupes`
- `guardrail_validate_staged_present`
- `clippy_denies_warnings`
- `gitleaks_step_present` only if universal hook contract covers it
- `cargo_dupes_step_present`
- `cargo_dupes_excludes`
- `config_changes_trigger_validation` only after trigger coverage is stronger

# Tests

Per package:

- each hook-contract package has tests proving exact requirement IDs, triggers, required commands, and critical commands
- every new hook source rule has rule-specific sidecar tests
- no grouped test module replacing rule-specific tests

Cross-package:

- hooks ingestion pipeline tests prove requirements reach source checks
- process runner tests prove `SupportedFamily::Hooks` aggregates contracts

Required adversarial cases:

- command only in comment does not satisfy
- command only in echo does not satisfy
- command behind `|| true` fails critical-command rule
- path-qualified command satisfies if executable command parser resolves it
- shell function wrapper satisfies only when parser proves the function is executed
- unrelated config path does not satisfy trigger coverage

# Verification

For each slice:

- `cargo test --manifest-path <new-or-touched-package>/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline --locked`
- `g3rs validate --path <each touched package>`
- `g3rs validate --path /Users/tartakovsky/Projects/websmasher/guardrail3 --family hooks --inventory`

# Non-Goals

- Do not implement TypeScript hooks in this plan.
- Do not parameterize `hook_contract()`.
- Do not move all shared hook code to `packages/shared` in the first slice.
- Do not remove current hardcoded hook rules until contract-driven replacements are proven.
- Do not make ingestion author hook policy.
- Do not use raw substring matching for new command presence rules.
