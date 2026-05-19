# Family Enablement Contract

## Goal

G3TS and G3RS must use the same family-selection model:

- Every workspace family is enabled by default.
- A family is disabled only when the workspace guardrail config explicitly sets that family to `false` in `[checks]`.
- No runner may auto-disable a family by probing framework files, package dependencies, or config files.
- Enabled families own their own missing-setup findings.
- Missing-setup findings must tell the user to either configure the missing tool/framework contract or disable the family in the guardrail config.

This replaces the current G3TS Astro-specific default skip.

## Key Decisions

- Do not add an applicability layer.
  - It would duplicate family ingestion and rule checks.
  - It would make the runner decide framework semantics that belong to rule families.

- Keep `[checks]` as the only family enablement mechanism.
  - G3RS already uses this model for workspace validation.
  - G3TS must match it.

- Missing or invalid guardrail config is a workspace-adoption error.
  - G3RS already fails when `guardrail3-rs.toml` is missing or invalid.
  - G3TS must fail when `guardrail3-ts.toml` is missing or invalid.
  - Repo validation can still report unadopted package roots, but workspace validation must not silently treat missing config as "no opt-outs".

- Toolchain gates and hook contracts must respect the selected enabled family set.
  - If `[checks].astro_setup = false`, Astro setup checks and Astro setup hook/toolchain requirements must not run.
  - If `[checks].style = false`, style checks and style hook/toolchain requirements must not run.

- Explicit `--family` does not override `[checks]`.
  - If a family is disabled in config, it is disabled for default and explicit validation.
  - This matches current G3RS selection behavior and keeps config authoritative.

## Current Incorrect Code

### G3TS

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
  - `default_disabled_families`
  - `astro_families`
  - `is_astro_workspace`
  - `has_astro_config_file`
  - `package_json_declares_astro`
  - `guardrail_config_declares_astro`

These functions implement a narrow Astro applicability shortcut and must be removed.

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`
  - `disabled_families` silently returns an empty list when `guardrail3-ts.toml` is missing or invalid.
  - It must return a typed error like G3RS.

- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/run.rs`
  - `hook_contracts()` collects all TS hook requirements unconditionally.
  - It must collect only contracts for enabled families.

- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`
  - `toolchain_gate_list` accepts disabled families.
  - It should accept enabled families, matching G3RS `cargo_gates`.

### G3RS

- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`
  - This is the target behavior for required config and `[checks]` parsing.

- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates.rs`
  - This is the target behavior for passing enabled families to toolchain gates.

- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
  - `rust_hook_requirements()` currently collects all Rust hook requirements unconditionally.
  - It must be audited and changed if disabled families can still affect repo-level hooks.

## Implementation Steps

### Step 1: Remove G3TS framework auto-skip

Modify `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`.

- Replace `default_disabled_families(request)` with direct config parsing:
  - `family_opt_out::disabled_families(&request.workspace_root)`
- Remove Astro detection helpers:
  - `default_disabled_families`
  - `astro_families`
  - `is_astro_workspace`
  - `has_astro_config_file`
  - `package_json_declares_astro`
  - `guardrail_config_declares_astro`

Done means:

- No `is_astro_workspace` symbol remains in active G3TS runtime code.
- No `default_disabled_families` symbol remains in active G3TS runtime code.
- Non-Astro workspaces run Astro families unless those Astro families are disabled in `[checks]`.

### Step 2: Make G3TS config opt-out match G3RS

Modify `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`.

- Add `GuardrailConfigError`.
- Change `disabled_families(package_root)` return type to `Result<Vec<SupportedFamily>, GuardrailConfigError>`.
- Missing `guardrail3-ts.toml` must return an error.
- Invalid `guardrail3-ts.toml` must return an error.
- A valid config without `[checks]` returns no disabled families.

Modify `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`.

- On config error, return exit code `1`.
- Stderr must name `guardrail3-ts.toml`.
- Stderr must say this is required at the workspace root.

Done means:

- G3TS and G3RS both fail workspace validation before family execution when the workspace guardrail config is missing or invalid.

### Step 3: Pass enabled families to TS toolchain gates

Modify `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`.

- Change `toolchain_gate_list(path, manager, disabled)` to `toolchain_gate_list(path, manager, enabled_families)`.
- Iterate only over `enabled_families`.
- Remove disabled-family filtering from inside the gate loop.

Modify call sites that currently pass disabled families.

Done means:

- A disabled family cannot contribute a toolchain gate command.
- TS toolchain gate code matches the G3RS enabled-family model.

### Step 4: Pass enabled families to TS hook requirements

Modify `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/run.rs`.

- Change `hook_contracts()` to accept an enabled-family slice.
- Collect hook contracts only for families in the enabled set.
- Do not collect Astro, style, fmt, spelling, or typecov contracts when their families are disabled.

The hooks runner must receive the enabled-family set from the validate runtime.

If the current `FamilyRunner` trait cannot pass this data, add the smallest typed context needed:

- either add enabled families to the crawl/request context passed to the hooks runner
- or add an explicit runner input object

Do not make hooks re-parse `guardrail3-ts.toml`.

Done means:

- Family selection is computed once in validate runtime.
- Hooks use the same enabled-family list as normal family execution.

### Step 5: Audit and align G3RS hooks

Inspect `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`.

- If repo-level hooks can require commands from disabled families, change the Rust hook runner to collect contracts only from enabled families.
- The enabled family list must come from the same selection path as workspace validation.
- Do not make hooks parse config independently.

Done means:

- G3RS hooks and G3TS hooks follow the same family enablement contract.

### Step 6: Fix family missing-setup messages

Audit family rules that emit missing setup/config/package errors.

For every family-owned missing setup error:

- Keep the specific setup instruction.
- Add the family disable instruction:
  - G3TS example: `If this workspace should not enforce Astro setup, set astro_setup = false under [checks] in guardrail3-ts.toml.`
  - G3RS example: `If this workspace should not enforce Clippy, set clippy = false under [checks] in guardrail3-rs.toml.`

Do not add this text to unrelated findings that are not setup/adoption errors.

Done means:

- Missing framework/tool config errors explain both valid resolutions:
  - configure the family
  - disable the family

### Step 7: Fixtures

Add or update fixtures for both CLIs.

G3TS fixtures:

- A non-Astro workspace with no Astro opt-outs must emit Astro missing-setup findings.
- The same workspace with all Astro families set to `false` under `[checks]` must not emit Astro family findings.
- A workspace with missing `guardrail3-ts.toml` must fail before family output.
- A workspace with invalid `guardrail3-ts.toml` must fail before family output.
- A workspace with a disabled family must not run that family's hook/toolchain requirements.

G3RS fixtures:

- A workspace with a disabled family must not run that family's hook/toolchain requirements.
- Missing and invalid `guardrail3-rs.toml` behavior must remain unchanged.

Done means:

- Fixture output proves family enablement affects static rules, hooks, and toolchain gates.

### Step 8: Verifier

Add a manifest verifier script.

Required checks:

- G3TS active runtime code contains no `is_astro_workspace`.
- G3TS active runtime code contains no `default_disabled_families`.
- G3TS `family_opt_out::disabled_families` returns `Result`.
- G3TS toolchain gate function accepts enabled families, not disabled families.
- TS and RS config opt-out modules both define a typed `GuardrailConfigError`.
- TS and RS selection code both use `[checks] = false` as the only family opt-out source.
- No active code contains framework-specific default skipping.

Done means:

- The verifier fails if a future change reintroduces a family applicability shortcut.

## Files To Modify

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
- G3TS fixture roots under `behavior/fixtures/g3ts-rule`
- G3RS fixture roots under `behavior/fixtures/g3rs-rule`
- New verifier script under `scripts/`
- This plan manifest

## Out Of Scope

- No framework detection.
- No package.json-based family applicability.
- No file-existence-based family applicability.
- No presets.
- No compatibility alias for old behavior.
