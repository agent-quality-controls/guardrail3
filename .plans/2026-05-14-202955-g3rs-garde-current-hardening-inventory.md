# G3RS Garde Current Hardening Inventory

## Scope

- Family: `garde`
- Active implementation root: `packages/rs/garde`
- Active runner: `apps/guardrail3-rs/crates/logic/family-runner-quality/src/run.rs`
- Current check lanes wired by runner:
  - config checks
  - source checks
- Current check lanes not wired by runner:
  - file-tree checks

## Evidence Commands

```sh
find packages/rs/garde -path '*/target' -prune -o -path '*/crates/runtime/src/*/rule.rs' -print | sort
find packages/rs/garde -path '*/target' -prune -o -path '*/crates/runtime/src/*/rule_tests/mod.rs' -print | sort
rg -n "GuardrailConfig|RS-GARDE-AST-08|validate-call|parse site" packages/rs/garde .plans/by_family/rs/garde.md .plans/todo/checks/rs/garde.md
sed -n '1288,1426p' behavior/coverage/g3rs-rule-coverage.toml
```

## Active Packages

- `packages/rs/garde/g3rs-garde-types`
- `packages/rs/garde/g3rs-garde-ingestion`
- `packages/rs/garde/g3rs-garde-config-checks`
- `packages/rs/garde/g3rs-garde-source-checks`
- `packages/rs/garde/g3rs-garde-hook-contract`

## Stale Plan References

- `.plans/by_family/rs/garde.md` still names `apps/guardrail3/crates/app/rs/families/garde/`.
- `.plans/todo/checks/rs/garde.md` still names `apps/guardrail3/crates/app/rs/families/garde/**`.
- `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` still names `apps/guardrail3/crates/app/rs/checks/rs/garde/`.
- The active code is under `packages/rs/garde`.
- Any implementation agent must target `packages/rs/garde` and the active `apps/guardrail3-rs` runner only.

## Rule Inventory

- `g3rs-garde/dependency-present`
  - Lane: config
  - Active rule file: `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/dependency_present/rule.rs`
  - Rule tests: `dependency_present/rule_tests`
  - Behavior replay: covered by `L33-release-profile-and-mutants-inputs`
  - Current status: implemented

- `g3rs-garde/core-method-bans`
  - Lane: config
  - Active rule file: `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/core_method_bans/rule.rs`
  - Rule tests: `core_method_bans/rule_tests`
  - Behavior replay: covered by `L30-guardrail-config-valid-required-inputs-missing`
  - Current status: implemented

- `g3rs-garde/extractor-type-bans`
  - Lane: config
  - Active rule file: `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/extractor_type_bans/rule.rs`
  - Rule tests: `extractor_type_bans/rule_tests`
  - Behavior replay: covered by `L30-guardrail-config-valid-required-inputs-missing`
  - Current status: implemented

- `g3rs-garde/reqwest-json-ban`
  - Lane: config
  - Active rule file: `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/reqwest_json_ban/rule.rs`
  - Rule tests: `reqwest_json_ban/rule_tests`
  - Behavior replay: covered by `L30-guardrail-config-valid-required-inputs-missing`
  - Current status: implemented

- `g3rs-garde/additional-method-bans`
  - Lane: config
  - Active rule file: `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/additional_method_bans/rule.rs`
  - Rule tests: `additional_method_bans/rule_tests`
  - Behavior replay: covered by `L30-guardrail-config-valid-required-inputs-missing`
  - Current status: implemented

- `g3rs-garde/struct-derive-validate`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/struct_derive_validate/rule.rs`
  - Rule tests: `struct_derive_validate/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/manual-deserialize-impl`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/manual_deserialize_impl/rule.rs`
  - Rule tests: `manual_deserialize_impl/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/enum-derive-validate`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/enum_derive_validate/rule.rs`
  - Rule tests: `enum_derive_validate/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/query-as-inventory`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/query_as_inventory/rule.rs`
  - Rule tests: `query_as_inventory/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/input-failures`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/input_failures/rule.rs`
  - Rule tests: `input_failures/rule_tests`
  - Behavior replay: covered by `L45-source-and-filetree-input-failures`
  - Current status: implemented

- `g3rs-garde/field-level-constraints`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/field_level_constraints/rule.rs`
  - Rule tests: `field_level_constraints/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/nested-validation-dive`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/nested_validation_dive/rule.rs`
  - Rule tests: `nested_validation_dive/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/context-validation-surface`
  - Lane: source
  - Active rule file: `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/context_validation_surface/rule.rs`
  - Rule tests: `context_validation_surface/rule_tests`
  - Behavior replay: covered by `L70-garde-boundary-policy-violated`
  - Current status: implemented

- `g3rs-garde/hook-contract`
  - Lane: hook contract
  - Active package: `packages/rs/garde/g3rs-garde-hook-contract`
  - Behavior replay: covered by `R15-hooks-reachable-no-root-cargo`
  - Current status: implemented

## Current Structural Status

- Active Garde check rules in `packages/rs/garde`: 13
- Active Garde hook contract rules: 1
- Behavior replay rows for Garde: 14
- Rule-specific `rule_tests/mod.rs` directories for active check rules: 13 of 13
- Legacy `*_tests.rs` sidecars under `packages/rs/garde`: 0
- File-tree check package: absent
- File-tree ingestion type exists only as a placeholder:
  - `packages/rs/garde/g3rs-garde-types/src/lib.rs`
  - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- The active runner does not call `ingest_for_file_tree_checks`, so the placeholder does not currently affect runtime output.

## Open Gaps

### G1: Plan claims one source rule that active code does not implement

- Plan claim:
  - `.plans/todo/checks/rs/garde.md` lists `RS-GARDE-AST-08` as implemented.
  - `.plans/by_family/rs/garde.md` says Garde owns a narrow runtime-usage rule for `GuardrailConfig` parse sites that skip `.validate()`.
- Active code state:
  - No active rule file under `packages/rs/garde` implements that rule.
  - No active rule ID exists for that behavior in `behavior/coverage/g3rs-rule-coverage.toml`.
  - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/source.rs` has tests that intentionally ignore legacy `GuardrailConfig` parse sites.
- Required decision:
  - Implement the rule in `g3rs-garde-source-checks`, or remove the claim from both Garde plans.
- Inventory recommendation:
  - Implement it, because both current plans describe it as an active Garde-owned boundary rule.

### G2: Plan says package-driven Garde policy inheritance exists, but active parser cannot represent it

- Plan claim:
  - `.plans/todo/checks/rs/garde.md` says package-driven `[rust.packages]` policy must affect Garde gating.
  - `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` repeats the same requirement.
- Active code state:
  - `packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs` has top-level `checks.garde`.
  - It has no typed `[rust.packages]` package policy surface.
  - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs` reads only `parsed.checks.garde`.
- Required decision:
  - Add typed package policy to `g3rs-toml-parser` and consume it in Garde ingestion, or remove the package-driven inheritance claim from the Garde plans.
- Inventory recommendation:
  - Do not implement Garde-local ad hoc package parsing. If package policy is real, add it to the shared `g3rs-toml-parser` first.

### G3: File-tree placeholder is dead surface

- Active code state:
  - `G3RsGardeFileTreeChecksInput` exists.
  - `ingest_for_file_tree_checks` always returns `FileTreeIngestionNotImplemented`.
  - No `g3rs-garde-file-tree-checks` package exists.
  - The active runner does not call file-tree ingestion.
- Required decision:
  - Delete the placeholder until a real Garde file-tree rule exists, or implement a real file-tree package and wire it.
- Inventory recommendation:
  - Delete the placeholder. No current Garde plan names an enforceable file-tree rule.

### G4: Behavior replay covers rule presence, not semantic depth

- Active code state:
  - Every active Garde rule has a behavior replay row.
  - Most behavior replay rows are covered by broad fixture layers:
    - config ban warnings come from missing `clippy.toml`
    - source boundary errors come from one `L70` fixture
- What this proves:
  - The rule IDs are reachable in CLI output.
- What this does not prove:
  - alias behavior for every boundary macro
  - cross-file validation-state resolution
  - duplicate simple-name ambiguity behavior
  - waiver matching edge cases
  - inactive-root non-hit behavior across all source rules
  - malformed policy interaction with source findings
- Required action:
  - Add a semantic hardening matrix before changing source rules.

### G5: Garde plans are split between active and historical paths

- Active code and current package layout are clean enough to continue.
- The plan files are not clean enough to hand to another implementation agent without correction.
- Required action:
  - Update `.plans/by_family/rs/garde.md`.
  - Update `.plans/todo/checks/rs/garde.md`.
  - Update `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` or replace it with a current package-root brief.

## Next Implementation Inventory

- `GARDE-HARDEN-01`
  - Fix stale plan paths.
  - Files:
    - `.plans/by_family/rs/garde.md`
    - `.plans/todo/checks/rs/garde.md`
    - `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md`
  - Done means no active Garde plan points at `apps/guardrail3/crates/app`.

- `GARDE-HARDEN-02`
  - Resolve `RS-GARDE-AST-08` contradiction.
  - Files:
    - `packages/rs/garde/g3rs-garde-types/src/lib.rs`
    - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/parse/analysis.rs`
    - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs`
    - `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/lib.rs`
    - new source rule module under `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src`
  - Done means a semantic rule ID exists, rule tests exist, ingestion produces typed sites, and behavior replay covers it.

- `GARDE-HARDEN-03`
  - Remove dead file-tree placeholder unless a concrete file-tree rule is designed first.
  - Files:
    - `packages/rs/garde/g3rs-garde-types/src/lib.rs`
    - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
    - `packages/rs/garde/g3rs-garde-ingestion/crates/types/src/error.rs`
  - Done means no public Garde file-tree API exists without a matching checks package.

- `GARDE-HARDEN-04`
  - Decide package-policy inheritance at the parser layer.
  - Files:
    - `packages/parsers/g3rs-toml-parser`
    - `packages/rs/garde/g3rs-garde-ingestion`
  - Done means either package policy is typed and consumed, or the Garde plans stop claiming it exists.

- `GARDE-HARDEN-05`
  - Build current semantic hardening matrix for the 13 active Garde check rules.
  - Output file:
    - `.plans/todo/check_review/test_hardening/19-garde-current-semantic-matrix.md`
  - Done means every active rule has exact current coverage and exact missing attack vectors.

## Recommended Next Work

Do `GARDE-HARDEN-01` and `GARDE-HARDEN-05` first.

Reason:
- The current plans point at stale paths.
- One planned rule is claimed as implemented but absent.
- Implementing source logic before fixing the inventory would repeat the same plan/code drift.
