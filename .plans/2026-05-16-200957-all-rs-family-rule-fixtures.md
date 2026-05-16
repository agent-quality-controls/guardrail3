# All Rust Family Rule Fixtures

## Goal

Build `behavior/fixtures/g3rs-rules/<family>/` fixture sets for every active Rust rule family.

Each completed family must have:

- Exactly one clean golden fixture.
- The smallest practical set of broken fixtures.
- Every active rule ID in that family broken by at least one broken fixture.
- No internal ingestion fixtures.
- No fixture-output crates.
- No serialization added only to expose internal structs.

The external contract is CLI behavior:

```text
g3rs validate workspace --path . --family <family> --rules-only --inventory
```

If a rule cannot be broken through that CLI surface, it is not covered by a family-rule fixture. It must stay in the kept-test disposition ledger with a concrete reason.

## Current State

Cargo and fmt are complete under the new standard.

Cargo currently has:

- `cargo-R00-clean-golden`
- `cargo-R10-policy-violations`
- `cargo-R21-root-metadata-missing`
- `cargo-R30-structure-and-input-violations`

Those four fixtures cover all 15 active Cargo rule IDs:

- one clean golden fixture exits 0
- three broken fixtures make every Cargo rule emit `Error` or `Warn`

Fmt currently has:

- `fmt-R00-clean-golden`
- `fmt-R10-filetree-violations`
- `fmt-R20-policy-violations`

Those three fixtures cover all 8 active fmt rule IDs:

- one clean golden fixture exits 0
- two broken fixtures make 7 fmt rules emit `Error` or `Warn`
- `g3rs-fmt/rustfmt-extra-settings-inventory` is explicit Info-only inventory because the implementation has no `Error` or `Warn` branch

## Global Fixture Contract

Each family fixture file must define:

- `id`
- `tool = "g3rs"`
- `run_from = "repo"`
- `commands`
- `expected_exit`
- `level`
- `rule_family`
- `target_rules`
- `expected_findings`

Clean golden fixture requirements:

- `level = "family_rule_clean_golden"`
- `expected_exit = "zero"`
- exactly one per family
- may list all family rules as inventory expectations
- must not be used as broken-rule coverage

Broken fixture requirements:

- `expected_exit = "nonzero"`
- every `target_rules` entry must emit `Error` or `Warn`
- every `target_rules` entry must appear in `expected_findings`
- every intentional `Error` or `Warn` row that defines the fixture purpose must be pinned in the plan manifest `required_results`
- unrelated `Error` or `Warn` output means the fixture is polluted unless it is added to `target_rules` and the fixture exists to cover that rule

## Verifier Contract

`scripts/behavior/verify-family-rule-fixtures.py` must enforce:

- one clean golden fixture per completed family
- fixture IDs are unique
- every fixture `target_rules` entry is a known active rule ID
- every fixture `expected_findings` entry is a known active rule ID
- every fixture appears in `behavior/golden/g3rs-validate/approved.normalized.json`
- clean fixtures exit 0
- broken fixtures exit nonzero
- broken fixtures make every `target_rules` entry emit `Error` or `Warn`
- completed families have every active rule ID broken by at least one broken fixture
- completed families may exclude a rule only if that rule has a kept-test ledger row explaining why CLI replay cannot expose it

The current verifier already enforces most of this. The missing hard requirement is completed-family full broken-rule coverage with ledger-aware exclusions.

## Fixture Minimization Rule

Build fixtures by family, not by individual rule.

For each family:

1. Start from the clean golden fixture.
2. Create one candidate broken fixture per rule layer.
3. Run the family CLI command.
4. Keep a rule in that fixture only if it emits `Error` or `Warn`.
5. If two broken states do not hide each other, merge them.
6. If one broken state stops another rule from running, split them.
7. Stop only when every active rule ID is either broken by a fixture or explicitly recorded as not CLI-exposable.

No fixture is allowed to exist only because a unit test used to exist.

## Family Inventory

The active rule inventory has 247 unique rule IDs across 14 rule namespaces.

### apparch

Rule count: 10.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/apparch/
```

Rules:

- `g3rs-apparch/dev-dependency-direction`
- `g3rs-apparch/io-outbound-dependency-direction`
- `g3rs-apparch/io-traits-in-types`
- `g3rs-apparch/logic-dependency-direction`
- `g3rs-apparch/logic-purity`
- `g3rs-apparch/patch-replace-bypass`
- `g3rs-apparch/same-layer-cycles`
- `g3rs-apparch/types-dependency-direction`
- `g3rs-apparch/types-public-surface`
- `g3rs-apparch/types-purity`

Fixture groups to attempt:

- `apparch-R00-clean-golden`: valid layer graph and source layout.
- `apparch-R10-layer-boundary-violations`: dependency direction, same-layer cycles, patch replacement bypass.
- `apparch-R20-surface-and-purity-violations`: types purity, logic purity, IO trait leakage, public surface leaks.

Expected risk:

- Boundary-cycle failures may hide later purity checks if the orchestrator refuses to classify source files after graph failure. Test merge candidates before committing fixture count.

### arch

Rule count: 10.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/arch/
```

Rules:

- `g3rs-arch/crate-has-facade`
- `g3rs-arch/dependency-count-split`
- `g3rs-arch/feature-contract`
- `g3rs-arch/feature-gated-exports`
- `g3rs-arch/lib-facade-only`
- `g3rs-arch/mod-facade-only`
- `g3rs-arch/mod-rs-required`
- `g3rs-arch/no-boundary-crossing`
- `g3rs-arch/no-path-attr`
- `g3rs-arch/shared-flag-required`

Fixture groups to attempt:

- `arch-R00-clean-golden`: valid crate facade, feature contract, module layout, and dependency shape.
- `arch-R10-package-contract-violations`: missing shared flag, bad feature contract, dependency count split, forbidden dependency edge.
- `arch-R20-filetree-violations`: missing facade and missing `mod.rs`.
- `arch-R30-source-violations`: non-facade exports and `#[path]` usage.

Expected risk:

- Missing facade can hide source facade-only rules. Keep filetree and source violations separate unless CLI output proves they coexist.

### cargo

Rule count: 15.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/cargo/
```

Status: complete.

Fixtures:

- `cargo-R00-clean-golden`
- `cargo-R10-policy-violations`
- `cargo-R21-root-metadata-missing`
- `cargo-R30-structure-and-input-violations`

Rules:

- `g3rs-cargo/approved-allow-inventory`
- `g3rs-cargo/disallowed-macros-deny`
- `g3rs-cargo/input-failures`
- `g3rs-cargo/lint-levels`
- `g3rs-cargo/member-edition-drift`
- `g3rs-cargo/member-local-allows-forbidden`
- `g3rs-cargo/missing-member-cargo`
- `g3rs-cargo/no-weakened-overrides`
- `g3rs-cargo/priority-order`
- `g3rs-cargo/resolver`
- `g3rs-cargo/rust-version-policy`
- `g3rs-cargo/unapproved-allow-entries`
- `g3rs-cargo/workspace-lints`
- `g3rs-cargo/workspace-lints-inherited`
- `g3rs-cargo/workspace-metadata`

### clippy

Rule count: 23.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/clippy/
```

Rules:

- `g3rs-clippy/avoid-breaking-exported-api`
- `g3rs-clippy/ban-reason-quality`
- `g3rs-clippy/cognitive-complexity-threshold`
- `g3rs-clippy/config-parseable`
- `g3rs-clippy/coverage-exists`
- `g3rs-clippy/duplicate-bans`
- `g3rs-clippy/excessive-nesting-threshold`
- `g3rs-clippy/extra-method-ban`
- `g3rs-clippy/extra-type-ban`
- `g3rs-clippy/forbid-clippy-conf-dir-override`
- `g3rs-clippy/library-global-state`
- `g3rs-clippy/macro-bans`
- `g3rs-clippy/max-fn-params-bools`
- `g3rs-clippy/max-struct-bools`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-clippy/policy-context-parseable`
- `g3rs-clippy/same-root-conflict`
- `g3rs-clippy/test-relaxations`
- `g3rs-clippy/too-many-arguments-threshold`
- `g3rs-clippy/too-many-lines-threshold`
- `g3rs-clippy/type-complexity-threshold`
- `g3rs-clippy/unknown-keys`

Fixture groups to attempt:

- `clippy-R00-clean-golden`: valid `clippy.toml`, no shadow config, covered workspace.
- `clippy-R10-config-missing-or-shadowed`: missing coverage, same-root conflict, forbidden `.cargo` override.
- `clippy-R20-config-parse-failures`: malformed config and policy-context parse failures.
- `clippy-R30-threshold-policy-violations`: threshold values and avoid-breaking-exported-api policy.
- `clippy-R40-ban-list-violations`: missing bans, extra bans, duplicate bans, weak ban reasons, macro bans, global-state bans.
- `clippy-R50-unknown-and-test-relaxations`: unknown keys and invalid test relaxations.

Expected risk:

- Malformed `clippy.toml` can hide semantic config rules. Parse failures need a separate fixture from policy-value failures.

### code

Rule count: 30.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/code/
```

Rules:

- `g3rs-code/always-true-cfg-attr-bypass`
- `g3rs-code/cfg-attr-allow-inventory`
- `g3rs-code/crate-level-allow`
- `g3rs-code/deny-forbid-without-reason`
- `g3rs-code/direct-fs-usage`
- `g3rs-code/exception-comment-inventory`
- `g3rs-code/extern-allow`
- `g3rs-code/fs-glob-import`
- `g3rs-code/garde-skip-with-comment`
- `g3rs-code/garde-skip-without-comment`
- `g3rs-code/generic-parameter-cap`
- `g3rs-code/impl-allow-blast-radius`
- `g3rs-code/include-bypass`
- `g3rs-code/input-failures`
- `g3rs-code/item-level-allow-with-reason`
- `g3rs-code/item-level-allow-without-reason`
- `g3rs-code/large-trait-surface`
- `g3rs-code/large-type-inventory`
- `g3rs-code/many-use-imports`
- `g3rs-code/panic-macro`
- `g3rs-code/path-attr-with-reason`
- `g3rs-code/public-struct-named-fields`
- `g3rs-code/public-weak-error-forms`
- `g3rs-code/string-dispatch-cap`
- `g3rs-code/test-expect-message-quality`
- `g3rs-code/todo-macros`
- `g3rs-code/too-many-effective-code-lines`
- `g3rs-code/too-many-use-imports`
- `g3rs-code/unsafe-code-lint`
- `g3rs-code/unused-crate-dependencies-allow`

Fixture groups to attempt:

- `code-R00-clean-golden`: valid source files with no bypasses.
- `code-R10-source-parse-failures`: malformed Rust source for input failure.
- `code-R20-allow-and-attribute-violations`: crate/item/impl/extern/cfg/path allow patterns and missing reasons.
- `code-R30-bypass-macro-violations`: include, todo, panic, direct filesystem, glob filesystem import.
- `code-R40-shape-and-size-violations`: generic cap, string dispatch cap, large trait, large type, import count, effective code lines.
- `code-R50-public-api-violations`: public weak error forms and public tuple/unit struct fields.
- `code-R60-garde-and-test-message-violations`: garde skip rules and test expect message quality.

Expected risk:

- Source parse failure can hide every AST-based source rule. Keep input failure isolated.

### deny

Rule count: 29.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/deny/
```

Rules:

- `g3rs-deny/advisories-baseline`
- `g3rs-deny/allow-git-inventory`
- `g3rs-deny/allow-override-channel`
- `g3rs-deny/allow-registry-baseline`
- `g3rs-deny/allow-wildcard-paths`
- `g3rs-deny/ban-baseline-complete`
- `g3rs-deny/confidence-threshold`
- `g3rs-deny/copyleft-allowlist`
- `g3rs-deny/coverage`
- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/duplicate-entries`
- `g3rs-deny/extra-deny-bans-inventory`
- `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/graph-all-features`
- `g3rs-deny/graph-no-default-features`
- `g3rs-deny/highlight-inventory`
- `g3rs-deny/ignore-accumulation`
- `g3rs-deny/ignore-hygiene`
- `g3rs-deny/license-allow-baseline`
- `g3rs-deny/license-exceptions-inventory`
- `g3rs-deny/multiple-versions-floor`
- `g3rs-deny/shadowing`
- `g3rs-deny/skip-hygiene`
- `g3rs-deny/stricter-advisories-inventory`
- `g3rs-deny/tokio-full-ban`
- `g3rs-deny/unknown-keys`
- `g3rs-deny/unknown-sources-policy`
- `g3rs-deny/wildcards-inventory`
- `g3rs-deny/wrappers`

Fixture groups to attempt:

- `deny-R00-clean-golden`: valid `deny.toml` placement and strict policy.
- `deny-R10-filetree-violations`: missing coverage and shadowing.
- `deny-R20-advisory-policy-violations`: advisory baseline, deprecated advisory handling, graph feature checks.
- `deny-R30-license-policy-violations`: license baseline, confidence threshold, copyleft allowlist, exceptions.
- `deny-R40-ban-policy-violations`: baseline completeness, duplicate entries, wildcards, tokio full, wrappers, multiple version floor.
- `deny-R50-source-policy-violations`: unknown sources, registry/git allow inventory, skip/ignore hygiene and accumulation.
- `deny-R60-inventory-only-policy`: highlight, extra deny bans, extra feature bans, stricter advisories inventory.

Expected risk:

- Deny config parse failure is likely already covered by global invalid-input fixtures. If a deny rule cannot break through family CLI without replacing config parse errors, keep that test internal.

### deps

Rule count: 11.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/deps/
```

Rules:

- `g3rs-deps/build-dependencies-allowlisted`
- `g3rs-deps/cargo-deny-installed`
- `g3rs-deps/cargo-dupes-installed`
- `g3rs-deps/cargo-lock-present`
- `g3rs-deps/cargo-machete-installed`
- `g3rs-deps/dependencies-allowlisted`
- `g3rs-deps/dev-dependencies-allowlisted`
- `g3rs-deps/direct-dependency-cap`
- `g3rs-deps/gitignore-not-ignoring-cargo-lock`
- `g3rs-deps/gitleaks-installed`
- `g3rs-deps/library-allowlist-present`

Fixture groups to attempt:

- `deps-R00-clean-golden`: lockfile present, tools installed, allowlists valid, dependency count valid.
- `deps-R10-required-files-and-tools`: missing lockfile, ignored lockfile, missing delegated tools, missing allowlist.
- `deps-R20-allowlist-and-count-violations`: dependencies, dev-dependencies, build-dependencies, direct dependency cap.

Expected risk:

- Missing allowlist can hide allowlist member checks. If so, split required-file/tool failures from dependency policy failures.

### fmt

Rule count: 8.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/fmt/
```

Rules:

- `g3rs-fmt/dual-file-conflict`
- `g3rs-fmt/edition-mismatch`
- `g3rs-fmt/ignore-escape-hatch`
- `g3rs-fmt/nightly-keys-on-stable`
- `g3rs-fmt/per-crate-override`
- `g3rs-fmt/rustfmt-config-exists`
- `g3rs-fmt/rustfmt-extra-settings-inventory`
- `g3rs-fmt/rustfmt-required-settings`

Status: complete.

Fixtures:

- `fmt-R00-clean-golden`: valid `rustfmt.toml` with required settings and no nested overrides.
- `fmt-R10-filetree-violations`: missing rustfmt config.
- `fmt-R20-policy-violations`: wrong required settings, edition mismatch, nightly keys on stable, ignored paths without reason, extra settings inventory.

Inventory-only rule:

- `g3rs-fmt/rustfmt-extra-settings-inventory`

### garde

Rule count: 13.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/garde/
```

Rules:

- `g3rs-garde/additional-method-bans`
- `g3rs-garde/context-validation-surface`
- `g3rs-garde/core-method-bans`
- `g3rs-garde/dependency-present`
- `g3rs-garde/enum-derive-validate`
- `g3rs-garde/extractor-type-bans`
- `g3rs-garde/field-level-constraints`
- `g3rs-garde/input-failures`
- `g3rs-garde/manual-deserialize-impl`
- `g3rs-garde/nested-validation-dive`
- `g3rs-garde/query-as-inventory`
- `g3rs-garde/reqwest-json-ban`
- `g3rs-garde/struct-derive-validate`

Fixture groups to attempt:

- `garde-R00-clean-golden`: dependency present and valid validated boundary source.
- `garde-R10-config-and-dependency-violations`: missing garde dependency and missing configured bans.
- `garde-R20-source-parse-failures`: malformed Rust input.
- `garde-R30-validation-shape-violations`: missing derive, missing field constraints, missing nested dive, context surface violation, manual deserialize.
- `garde-R40-boundary-bypass-violations`: extractor type bans, reqwest JSON ban, query_as inventory.

Expected risk:

- Missing dependency may not prevent source rules from running, but it is cleaner to keep dependency enforcement separate from source-shape enforcement.

### hooks

Rule count: 36.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/hooks/
```

Rules:

- `g3rs-hooks/cargo-dupes-excludes`
- `g3rs-hooks/cargo-dupes-installed`
- `g3rs-hooks/clippy-denies-warnings`
- `g3rs-hooks/concrete-lockfile-command`
- `g3rs-hooks/contract-critical-command-not-fail-open`
- `g3rs-hooks/contract-required-tools-installed`
- `g3rs-hooks/contract-trigger-coverage`
- `g3rs-hooks/dispatcher-pattern`
- `g3rs-hooks/executable-command-context-only`
- `g3rs-hooks/execution-trust`
- `g3rs-hooks/file-size-step-present`
- `g3rs-hooks/gitleaks-step-present`
- `g3rs-hooks/guardrail-binary-available`
- `g3rs-hooks/hooks-path-configured`
- `g3rs-hooks/local-override-inventory`
- `g3rs-hooks/merge-conflict-step-present`
- `g3rs-hooks/modular-directory-inventory`
- `g3rs-hooks/modular-scripts-executable`
- `g3rs-hooks/modular-scripts-inventory`
- `g3rs-hooks/no-bypass-instructions`
- `g3rs-hooks/no-fail-open-wrappers`
- `g3rs-hooks/no-unconditional-exit-zero`
- `g3rs-hooks/pre-commit-executable`
- `g3rs-hooks/pre-commit-exists`
- `g3rs-hooks/pre-commit-file-size-inventory`
- `g3rs-hooks/real-dispatcher-syntax-only`
- `g3rs-hooks/required-contract-command-present`
- `g3rs-hooks/required-tools-installed`
- `g3rs-hooks/routing-discovers-marker-pair`
- `g3rs-hooks/routing-no-env-override`
- `g3rs-hooks/routing-no-upward-walk-from-units`
- `g3rs-hooks/routing-scope-not-hardcoded-literal`
- `g3rs-hooks/routing-staged-files-diff-filter-acm`
- `g3rs-hooks/script-stats-inventory`
- `g3rs-hooks/shell-error-handling`
- `g3rs-hooks/valid-shebang`

Fixture groups to attempt:

- `hooks-R00-clean-golden`: valid modular hook, executable scripts, correct routing, required commands, required tools.
- `hooks-R10-filetree-violations`: missing pre-commit, not executable, missing modular directory, non-executable modular scripts, hook path not configured.
- `hooks-R20-shell-safety-violations`: fail-open wrappers, unconditional exit zero, bypass instructions, invalid shebang, weak shell error handling.
- `hooks-R30-command-context-violations`: inert text command, missing required commands, critical command fail-open.
- `hooks-R40-routing-violations`: hardcoded scope, env override routing, missing marker discovery, upward walk from unit, wrong staged-files diff filter.
- `hooks-R50-workflow-step-violations`: gitleaks, file-size, merge-conflict, lockfile, cargo-dupes excludes, clippy warnings.
- `hooks-R60-tool-inventory-violations`: required tools and contract-required tools missing.

Expected risk:

- Missing pre-commit hook hides source checks. Keep missing-file fixture separate from malformed-script fixtures.

### release

Rule count: 33.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/release/
```

Rules:

- `g3rs-release/accidentally-publishable`
- `g3rs-release/binary-release-workflow`
- `g3rs-release/binstall-metadata`
- `g3rs-release/categories-present`
- `g3rs-release/cliff-baseline`
- `g3rs-release/cliff-exists`
- `g3rs-release/config-input-failures`
- `g3rs-release/crate-inventory`
- `g3rs-release/description-present`
- `g3rs-release/docs-rs-metadata`
- `g3rs-release/filetree-input-failures`
- `g3rs-release/include-exclude-inventory`
- `g3rs-release/interdependent-version-consistency`
- `g3rs-release/keywords-present`
- `g3rs-release/license-file`
- `g3rs-release/license-present`
- `g3rs-release/linux-release-target`
- `g3rs-release/no-path-deps-to-unpublishable`
- `g3rs-release/publish-dry-run`
- `g3rs-release/publish-dry-run-workflow`
- `g3rs-release/publish-must-be-explicit`
- `g3rs-release/publish-status-inventory`
- `g3rs-release/readme-exists`
- `g3rs-release/readme-quality`
- `g3rs-release/registry-token`
- `g3rs-release/release-plz-baseline`
- `g3rs-release/release-plz-exists`
- `g3rs-release/release-plz-workflow`
- `g3rs-release/release-profile-inventory`
- `g3rs-release/repository-present`
- `g3rs-release/semver-checks-installed`
- `g3rs-release/source-input-failures`
- `g3rs-release/valid-semver`

Fixture groups to attempt:

- `release-R00-clean-golden`: publish-safe package with release metadata, workflows, README, license, release-plz, cliff.
- `release-R10-filetree-violations`: missing README, LICENSE, release-plz, cliff, malformed inputs.
- `release-R20-package-metadata-violations`: missing description, license, repository, categories, keywords, docs.rs, binstall, include/exclude inventory.
- `release-R30-publish-policy-violations`: accidentally publishable, missing explicit publish, no path deps to unpublishable, publish dry run.
- `release-R40-version-policy-violations`: invalid semver, interdependent version mismatch, release profile inventory.
- `release-R50-workflow-violations`: release-plz workflow, publish dry-run workflow, registry token, binary release target.
- `release-R60-source-quality-violations`: README quality and source input failures.

Expected risk:

- Invalid Cargo metadata can hide release semantic checks. Keep parse/input failure fixture separate from policy fixtures.

### test

Rule count: 19.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/test/
```

Rules:

- `g3rs-test/assertions-modules-prove`
- `g3rs-test/cargo-mutants-installed`
- `g3rs-test/external-harnesses-use-assertions`
- `g3rs-test/filetree-input-failures`
- `g3rs-test/ignore-reason`
- `g3rs-test/inline-test-bodies`
- `g3rs-test/mutants-config-sane`
- `g3rs-test/mutants-profile-present`
- `g3rs-test/mutants-toml-exists`
- `g3rs-test/mutation-hook-present`
- `g3rs-test/nextest-timeouts`
- `g3rs-test/owned-sidecar-shape`
- `g3rs-test/real-proof-site`
- `g3rs-test/runtime-assertions-split`
- `g3rs-test/should-panic-expected`
- `g3rs-test/source-input-failures`
- `g3rs-test/tautological-assertions`
- `g3rs-test/test-support-generic`
- `g3rs-test/weak-matches-assert`

Fixture groups to attempt:

- `test-R00-clean-golden`: valid sidecar tests, nextest, mutants, hook contract, proof assertions.
- `test-R10-required-config-violations`: missing nextest, mutants config, mutants profile, cargo-mutants install, mutation hook.
- `test-R20-filetree-violations`: owned sidecar shape, runtime assertions split, generic test support, filetree input failure.
- `test-R30-source-parse-failures`: malformed test/source input.
- `test-R40-test-body-violations`: inline tests, weak matches assert, tautological assertions, should_panic expected, ignore reason.
- `test-R50-proof-quality-violations`: assertions modules prove, external harnesses, real proof site.

Expected risk:

- Source parse failure hides all source-based test quality rules. Keep parse failure isolated.

### toolchain

Rule count: 4.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/toolchain/
```

Rules:

- `g3rs-toolchain/channel-and-components`
- `g3rs-toolchain/exists`
- `g3rs-toolchain/legacy-file`
- `g3rs-toolchain/msrv-consistency`

Fixture groups to attempt:

- `toolchain-R00-clean-golden`: valid `rust-toolchain.toml`, no legacy file, MSRV aligned.
- `toolchain-R10-filetree-violations`: missing `rust-toolchain.toml`, legacy `rust-toolchain` file.
- `toolchain-R20-policy-violations`: wrong channel/components and MSRV older than package policy.

Expected risk:

- Missing `rust-toolchain.toml` hides channel and MSRV checks. Keep missing-file fixture separate from wrong-value fixture.

### topology

Rule count: 6.

Target fixture directory:

```text
behavior/fixtures/g3rs-rules/topology/
```

Rules:

- `g3rs-topology/declared-workspace-members-only`
- `g3rs-topology/member-paths-must-not-escape-root`
- `g3rs-topology/no-nested-guardrail3-rs-toml`
- `g3rs-topology/no-nested-workspaces`
- `g3rs-topology/required-inputs-fail-closed`
- `g3rs-topology/workspace-local-file-placement`

Fixture groups to attempt:

- `topology-R00-clean-golden`: valid workspace topology and local file placement.
- `topology-R10-workspace-membership-violations`: undeclared member, missing declared member, escaping member path.
- `topology-R20-nesting-and-placement-violations`: nested workspace, nested guardrail config, illegal workspace-local files.
- `topology-R30-input-failure-violations`: malformed required input that must fail closed.

Expected risk:

- Malformed root Cargo input can hide membership rules. Keep required input failure separate.

## Build Order

Implement in this order:

1. `toolchain`
2. `deps`
3. `clippy`
4. `deny`
5. `topology`
6. `arch`
7. `apparch`
8. `garde`
9. `test`
10. `release`
11. `code`
12. `hooks`

Cargo is already complete and stays as the reference implementation.

This order closes small config/filetree families first, then policy config families, then source-heavy families, then hook behavior last.

## Verification Commands

Run after each family:

```bash
fixture3 check --suite g3rs-validate
python3 scripts/behavior/verify-family-rule-fixtures.py
python3 scripts/behavior/verify-rule-coverage.py
python3 scripts/behavior/verify-kept-test-dispositions.py
python3 scripts/behavior/verify-test-deletion.py
g3rs validate repo --path "$PWD"
git diff --check
```

Run before committing:

```bash
bash scripts/behavior/verify-all.sh
g3rs validate repo --path "$PWD"
git diff --check
```

## Done Criteria

The work is done only when:

- Every active `g3rs-*` rule namespace has a directory under `behavior/fixtures/g3rs-rules`.
- Every family directory has exactly one clean golden fixture.
- Every active rule ID emits `Error` or `Warn` in at least one broken fixture, unless a kept-test ledger row says it is not CLI-exposable.
- `verify-family-rule-fixtures.py` enforces that completed-family coverage mechanically.
- `fixture3 check --all` passes.
- `bash scripts/behavior/verify-all.sh` passes.
- `g3rs validate repo --path "$PWD"` passes.
