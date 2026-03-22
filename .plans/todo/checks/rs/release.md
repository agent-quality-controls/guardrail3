# RS-RELEASE — Release readiness checker (28 rules)

**Input:** Cargo.toml + release-plz.toml + cliff.toml + .github/workflows/*.yml + README files
**Current code:** `release_checks.rs`, `release_repo_checks.rs`, `release_crate_checks.rs`, `release_crate_deps.rs`, `release_bin_checks.rs`, `workspace_metadata.rs`

## Relocated from RS-ARCH

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-RELEASE-09 | R56 | Info | Publish status inventory (workspace publish field) | Implemented (was in workspace_metadata.rs) |
| RS-RELEASE-10 | R57 | Info | Release profile settings inventory (LTO, strip, codegen-units, etc.) | Implemented (was in workspace_metadata.rs) |

## Repo-level rules (release_repo_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-RELEASE-01 | R-REL-01 | Error | LICENSE file at repo root (LICENSE, LICENSE-MIT, LICENSE-APACHE, LICENSE.md) | Implemented |
| RS-RELEASE-02 | R-REL-02 | Warn | release-plz.toml exists at repo root | Implemented |
| RS-RELEASE-03 | R-REL-03 | Warn | release-plz.toml: [workspace] section + [[package]] entries cover all publishable crates | Implemented |
| RS-RELEASE-04 | R-REL-04 | Warn | cliff.toml exists (changelog config) | Implemented |
| RS-RELEASE-05 | R-REL-05 | Warn | GitHub workflow references "release-plz" | Implemented |
| RS-RELEASE-06 | R-REL-06 | Warn | GitHub workflow contains "cargo publish --dry-run" | Implemented |
| RS-RELEASE-07 | R-REL-07 | Warn | GitHub workflow references CARGO_REGISTRY_TOKEN | Implemented |
| RS-RELEASE-08 | R-REL-08 | Warn | cargo-semver-checks installed on PATH | Implemented |

## Per-crate publish metadata (release_crate_checks.rs + release_crate_deps.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-PUB-01 | R-PUB-01 | Error | [package].description present | Implemented |
| RS-PUB-02 | R-PUB-02 | Error | [package].license or license-file present | Implemented |
| RS-PUB-03 | R-PUB-03 | Error | [package].repository URL present | Implemented |
| RS-PUB-04 | R-PUB-04 | Warn | README file exists (explicit field or default README.md) | Implemented |
| RS-PUB-05 | R-PUB-05 | Warn | README quality: ≥200 bytes + has heading (#) | Implemented |
| RS-PUB-06 | R-PUB-06 | Warn | [package].keywords present (1-5 entries) | Implemented |
| RS-PUB-07 | R-PUB-07 | Warn | [package].categories present and non-empty | Implemented |
| RS-PUB-08 | R-PUB-08 | Error | [package].version valid semver (or workspace=true) | Implemented |
| RS-PUB-09 | R-PUB-09 | Error | cargo publish --dry-run succeeds (thorough mode only) | Implemented |
| RS-PUB-10 | R-PUB-10 | Error | No path deps to publish=false crates | Implemented |
| RS-PUB-11 | R-PUB-11 | Error | Interdependent version consistency (semver compat) | Implemented |
| RS-PUB-12 | R-PUB-12 | Info | Crate inventory (publishable + non-publishable count) | Implemented |

## Binary release (release_bin_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-BIN-01 | R-BIN-01 | Info | Binary release workflow (--release + action-gh-release) | Implemented |
| RS-BIN-02 | R-BIN-02 | Info | Linux target in workflow | Implemented |
| RS-BIN-03 | R-BIN-03 | Warn | [package.metadata.binstall] for cargo binstall (binary crates only) | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-RELEASE-11 | Warn | Accidentally publishable internal crates. If a crate is missing ALL of description, license, and repository AND does NOT have `publish = false`, it's probably not meant for crates.io. | Planned |
| RS-PUB-13 | Info | `[package.metadata.docs.rs]` for library crates. docs.rs builds are not retryable — if the first build fails, the crate has no docs forever. Library crates should configure features/targets for docs.rs. | Planned |
| RS-PUB-14 | Info | `include`/`exclude` patterns in [package]. Prevents shipping test fixtures, benchmarks, and large assets to crates.io. Info if neither is set on a publishable crate. | Planned |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| CHANGELOG.md exists | release-plz auto-generates from cliff.toml. Checking before first release always fails. Workflow state, not config. |
| Git tag format | Runtime state (git tags), not configuration. release-plz controls tag format via [[package]] config. |
| Branch protection | GitHub API setting, not repo file. Outside guardrail3's scope. |
| Coverage threshold | Coverage tooling is varied (tarpaulin, llvm-cov, codecov). CI pipeline handles enforcement. |
