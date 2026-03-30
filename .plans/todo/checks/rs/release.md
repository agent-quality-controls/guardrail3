# RS-RELEASE — Release readiness checker (29 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/release.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/release.md).
> Family-local behavior and file shape now live in [`apps/guardrail3/crates/app/rs/families/release/README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/release/README.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** Cargo.toml + release-plz.toml + cliff.toml + .github/workflows/*.yml + README files
**Current code:** `apps/guardrail3/crates/app/rs/families/release/**` (old `validate/release_checks.rs`, `validate/release_repo_checks.rs`, `validate/release_crate_checks.rs`, `validate/release_crate_deps.rs`, `validate/release_bin_checks.rs`, and `validate/workspace_metadata.rs` are legacy seed material only)
**Family README:** `apps/guardrail3/crates/app/rs/families/release/README.md`

## Implementation mapping contract

- exactly one `RS-RELEASE-*`, `RS-PUB-*`, or `RS-BIN-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs`, `inputs.rs`, and `release_support.rs` may contain shared facts, typed inputs, and release-specific normalization helpers only

Forbidden:

- grouped family test files such as `release_tests.rs`
- helper files that hide multiple unrelated rule predicates behind one API

## Discovery / ownership model

`RS-RELEASE` is a mixed-scope family.

It owns:
- validation-root / repo-level release artifacts:
  - root `Cargo.toml`
  - `release-plz.toml`
  - `cliff.toml`
  - `.github/workflows/*.yml`
  - root license file presence
- per-package publishability facts for local Rust packages
- local release-edge facts between publishable crates

The family must not collapse all release checks into repo-root-only Cargo behavior.

It needs all local package roots because:
- publishable-crate metadata is package-local
- README ownership is package-local
- local path/version release edges are package-local

## Input integrity / fail-closed expectations

The family depends on:
- readable and parseable local `Cargo.toml` files used for repo, package, or edge facts
- readable and parseable `release-plz.toml`
- readable and parseable `cliff.toml`
- readable and parseable workflow YAML files
- readable README content when a README rule needs the file content
- actual command outcomes for `cargo publish --dry-run` in thorough mode

Malformed required inputs must surface explicitly through `RS-RELEASE-12`.

That includes:
- unreadable or unparsable package manifests
- unreadable or unparsable workflow YAML
- unreadable README content required for README quality checks
- parse failures in root release policy files

The family must not silently skip release findings because one required artifact was unreadable.

## Family fact split

The current family is intentionally split into:
- repo-level facts
- publishable-crate facts
- release-edge facts
- input-failure facts

That split should stay explicit in the plan because the rules are not all evaluating the same scope.

## Relocated from RS-ARCH

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-RELEASE-09 | R56 | Info | Publish status inventory (workspace publish field) | Implemented in new family |
| RS-RELEASE-10 | R57 | Info | Release profile settings inventory (LTO, strip, codegen-units, etc.) | Implemented in new family |

## Repo-level rules (release_repo_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-RELEASE-01 | R-REL-01 | Error | LICENSE file at repo root (LICENSE, LICENSE-MIT, LICENSE-APACHE, LICENSE.md) | Implemented in new family |
| RS-RELEASE-02 | R-REL-02 | Warn | release-plz.toml exists at repo root | Implemented in new family |
| RS-RELEASE-03 | R-REL-03 | Warn | release-plz.toml: [workspace] section + [[package]] entries cover all publishable crates | Implemented in new family |
| RS-RELEASE-04 | R-REL-04 | Warn | cliff.toml exists (changelog config) | Implemented in new family |
| RS-RELEASE-05 | R-REL-05 | Warn | GitHub workflow contains an actual release-plz execution step, not just substring mention | Implemented in new family |
| RS-RELEASE-06 | R-REL-06 | Warn | GitHub workflow contains an actual `cargo publish --dry-run` execution step | Implemented in new family |
| RS-RELEASE-07 | R-REL-07 | Warn | GitHub workflow actually wires `CARGO_REGISTRY_TOKEN` into release flow | Implemented in new family |
| RS-RELEASE-08 | R-REL-08 | Warn | cargo-semver-checks installed on PATH | Implemented in new family |

## Per-crate publish metadata (release_crate_checks.rs + release_crate_deps.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-PUB-01 | R-PUB-01 | Error | [package].description present | Implemented in new family |
| RS-PUB-02 | R-PUB-02 | Error | [package].license or license-file present | Implemented in new family |
| RS-PUB-03 | R-PUB-03 | Error | [package].repository URL present | Implemented in new family |
| RS-PUB-04 | R-PUB-04 | Warn | README file exists (explicit field or default README.md) | Implemented in new family |
| RS-PUB-05 | R-PUB-05 | Warn | README quality: ≥200 bytes + has heading (#) | Implemented in new family |
| RS-PUB-06 | R-PUB-06 | Warn | [package].keywords present (1-5 entries) | Implemented in new family |
| RS-PUB-07 | R-PUB-07 | Warn | [package].categories present and non-empty | Implemented in new family |
| RS-PUB-08 | R-PUB-08 | Error | [package].version valid semver (or workspace=true) | Implemented in new family |
| RS-PUB-09 | R-PUB-09 | Error | cargo publish --dry-run succeeds (thorough mode only) | Implemented in new family |
| RS-PUB-10 | R-PUB-10 | Error | No path deps to publish=false crates across normal/build/target-specific dependency tables | Implemented in new family |
| RS-PUB-11 | R-PUB-11 | Error | Interdependent version consistency (semver compat) across local publishable crate edges | Implemented in new family |
| RS-PUB-12 | R-PUB-12 | Info | Crate inventory (publishable + non-publishable count) | Implemented in new family |

## Binary release (release_bin_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-BIN-01 | R-BIN-01 | Info | Binary release workflow (`--release` + GitHub release action) | Implemented in new family |
| RS-BIN-02 | R-BIN-02 | Info | Linux target in workflow | Implemented in new family |
| RS-BIN-03 | R-BIN-03 | Warn | [package.metadata.binstall] for cargo binstall (binary crates only) | Implemented in new family |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-RELEASE-11 | Warn | Accidentally publishable internal crates. If a crate is missing ALL of description, license, and repository AND does NOT have `publish = false`, it's probably not meant for crates.io. | Implemented in new family |
| RS-PUB-13 | Info | `[package.metadata.docs.rs]` for library crates. docs.rs builds are not retryable — if the first build fails, the crate has no docs forever. Library crates should configure features/targets for docs.rs. | Implemented in new family |
| RS-PUB-14 | Info | `include`/`exclude` patterns in [package]. Prevents shipping test fixtures, benchmarks, and large assets to crates.io. Info if neither is set on a publishable crate. | Implemented in new family |
| RS-RELEASE-12 | Error | Release-family input failures: unreadable/unparsable Cargo.toml, release-plz.toml, cliff.toml, workflow YAML, or README content required to evaluate release rules. Fail closed instead of silently skipping. | Implemented in new family |

## Notes for new implementation

- Build `rs/release` as a new-architecture family; do not migrate the old `WalkDir` crate discovery directly.
- The old validator is useful as attack-seed material, not as the target contract.
- Old gaps to avoid copying:
  - silent crate discovery skips for unreadable/unparsable Cargo.toml
  - substring-only workflow checks for `release-plz`, dry-run, token wiring, and binary release
  - `R-PUB-09` success detection based on stderr text instead of command outcome
  - `R-PUB-10` / `11` limited to only part of the dependency surface
  - README quality silently disappearing if the file exists but cannot be read
- The new family should likely split into:
  - repo-level facts (`release-plz`, `cliff`, workflows, license, semver-check tool)
  - publishable-crate facts
  - release-edge facts for local path/version relationships
  - binary-crate facts
  - input failures

## Legacy carry-forward

This plan supersedes the older top-level release setup note that is now archived under `.plans/todo/legacy/release_setup_validator.md`.

No additional Rust release requirements remain stranded in that legacy doc. What is still live for `rs/release` is already represented here.

Residual hardening items from archived legacy notes remain relevant here:

- workflow rules (`RS-RELEASE-05..07`, `RS-BIN-01..02`) now preserve parsed workflow structure through family facts and evaluate that structure directly, but they still classify semantics through release-family helper matching rather than a fuller Actions-specific execution model. Good enough for breadth-first completion, but still a later hardening target.
- the archived `semver_releases.md` template now informs active checker semantics for:
  - `RS-RELEASE-03`:
    - `[workspace].changelog_config = "cliff.toml"`
    - `[workspace].git_release_enable = true`
    - `[workspace].release_always = false`
  - `RS-RELEASE-04`:
    - `[git].conventional_commits = true`
    - `[git].filter_unconventional = true`
    - canonical `commit_parsers` coverage for `feat/fix/doc/perf/refactor/style/test/chore`
- the remaining later-hardening item is richer workflow execution semantics, not release-plz/cliff baseline ownership.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| CHANGELOG.md exists | release-plz auto-generates from cliff.toml. Checking before first release always fails. Workflow state, not config. |
| Git tag format | Runtime state (git tags), not configuration. release-plz controls tag format via [[package]] config. |
| Branch protection | GitHub API setting, not repo file. Outside guardrail3's scope. |
| Coverage threshold | Coverage tooling is varied (tarpaulin, llvm-cov, codecov). CI pipeline handles enforcement. |
