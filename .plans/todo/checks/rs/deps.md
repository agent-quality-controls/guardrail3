# RS-DEPS — Tool + dependency policy checker (12 implemented rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/deps.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/deps.md).
> Keep this file as a detailed rule ledger and migration/history reference.
> Historical note: the legacy app-family mapper / `guardrail3.toml` model described below is no longer the target architecture for package work. The target package architecture uses required workspace `guardrail3-rs.toml`, package-owned ingestion, normalized dependency facts for config checks, and separate file-tree ownership for local path target validation.

**Historical input:** Tool PATH checks + Cargo.lock + .gitignore + guardrail3.toml allowlists + Cargo.toml dependency tables
**Historical code:** `apps/guardrail3/crates/app/rs/families/deps/`

## Target Package Architecture

Package roots:

- `packages/rs/deps/g3rs-deps-types`
- `packages/rs/deps/g3rs-deps-config-checks`
- `packages/rs/deps/g3rs-deps-config-ingestion` (planned)

Required policy input:

- workspace `guardrail3-rs.toml`

Target config-check input contract:

```rust
pub struct G3RsDepsConfigChecksInput {
    pub crate_cargo_rel_path: String,
    pub crate_name: String,
    pub profile: Option<RustProfile>,
    pub allowlist_present: bool,
    pub allowed_deps: Vec<String>,
    pub dependencies: Vec<G3RsDepsResolvedDependency>,
}
```

```rust
pub struct G3RsDepsResolvedDependency {
    pub package_name: String,
    pub section: G3RsDepsDependencySection,
    pub table_label: String,
}
```

This means:

- config checks consume normalized external dependency facts
- config checks keep explicit `allowlist_present` because the current parser does not preserve missing-vs-empty `allowed_deps`
- config checks do not consume raw local path target manifests
- config checks do not consume app `GuardrailConfig`
- local path target validation belongs to file-tree ownership, not config-check ownership

This family owns:
- required external Rust/tooling presence
- dependency allowlist enforcement for external crates
- allowlist coverage policy for library crates
- lockfile policy

This family does **not** own:
- banned crates in the lockfile (`cargo-deny` / `RS-DENY`)
- dependency direction (`RS-HEXARCH`)
- release-specific dependency policy (`RS-RELEASE`)
- hook execution of tools (`HOOK-RS`)

## Discovery / ownership model

`RS-DEPS` is a mixed-scope family:
- tool-install rules are validation-root checks
- allowlist rules are per local crate/package
- lockfile rules are per Rust root

Owned local crates/packages are all discovered local `Cargo.toml` package roots.

Owned Rust roots for lockfile policy are:
- workspace roots
- standalone package roots that are not members of a workspace and are outside any discovered workspace-root subtree

The family must not collapse lockfile policy to repo-root-only behavior, and it must not confuse crate-local dependency policy with workspace-root lockfile ownership.

## Policy ownership

Historical app implementation:

Dependency allowlist/profile policy comes from validation-root `guardrail3.toml`.

That means:
- the family resolves crate-local allowlist/profile policy from the one root policy surface
- per-app / per-package entries inside that root policy file are still crate-local policy once resolved
- future verification must not invent nearest-local `guardrail3.toml` semantics unless the plan changes explicitly

Target package implementation:

- dependency allowlist/profile policy comes from required workspace `guardrail3-rs.toml`
- ingestion normalizes crate-local policy into config-check input
- package code must not depend on app config types

## Tool installation rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-01 | R45 | Error | `cargo-deny` installed on PATH | Implemented |
| RS-DEPS-02 | R46 | Error | `cargo-machete` installed on PATH | Implemented |
| RS-DEPS-03 | R47 | Warn | `cargo-dupes` installed on PATH (recommended) | Implemented |
| RS-DEPS-04 | R48 | Error | `gitleaks` installed on PATH | Implemented |

## Allowlist rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-CONFIG-01 | R-DEPS-01 | Error | Unauthorized external dependency in `[dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-CONFIG-02 | — | Error | Unauthorized external dependency in `[build-dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-CONFIG-03 | — | Warn | Unauthorized external dependency in `[dev-dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-CONFIG-04 | R-DEPS-02 | Warn | Library crate has no dependency allowlist configured | Implemented |

Allowlist semantics:
- no `allowed_deps` configured means `RS-DEPS-CONFIG-01..07` stay silent; only `RS-DEPS-CONFIG-04` warns for library-profile crates
- path dependencies are a normalization concern, not raw config-check input
- `workspace = true` is **not** automatically skipped
- if `workspace = true` resolves to an external workspace dependency, it must still be allowlisted
- local path dependencies that resolve to invalid or non-member local targets belong to file-tree ownership
- renamed dependencies must be checked against the real `package` name when present

Section ownership:
- `RS-DEPS-CONFIG-01` owns `[dependencies]`
- `RS-DEPS-CONFIG-02` owns `[build-dependencies]`
- `RS-DEPS-CONFIG-03` owns `[dev-dependencies]`
- `RS-DEPS-CONFIG-01..07` also own the matching `target.*` dependency tables
- `RS-DEPS-CONFIG-05` owns the direct-dependency cap across both top-level and target-specific dependency tables

## Lockfile rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-09 | Error/Info | `Cargo.lock` committed for each Rust root. Services/binaries: Error if missing. Libraries: Info if missing. | Implemented |
| RS-DEPS-10 | Error | No relevant `.gitignore` may ignore a Rust root's `Cargo.lock` | Implemented |

## Input / parse integrity rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-11 | Error | Required dependency-policy inputs unreadable or unparseable: member Cargo.toml, workspace Cargo.toml for `workspace = true` resolution, or `guardrail3-rs.toml` when needed for profile/allowlist policy. | Target contract |

## Input integrity / fail-closed expectations

The family depends on:
- readable local package manifests
- readable workspace manifests when `workspace = true` resolution needs them
- readable workspace `guardrail3-rs.toml` when crate policy/profile needs it
- readable `.gitignore` inputs used for lockfile masking checks

Malformed required config inputs must fail the config lane rather than silently suppressing allowlist or lockfile findings.
Malformed local path target structure belongs to file-tree ownership rather than config-check ownership.

## Direct dependency cap rule

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-CONFIG-05 | Error | More than 25 unique direct dependency package names on one crate across owned direct dependency sections/tables. | Implemented |

### RS-DEPS-CONFIG-05 — Direct dependency count cap

**Intent**
- catch dependency sprawl and agentic crate accretion

**Trigger surface**
- one crate/package has more than 25 unique direct dependency names across:
  - `[dependencies]`
  - `[build-dependencies]`
  - `[dev-dependencies]`
  - `target.*.dependencies`
  - `target.*.build-dependencies`
  - `target.*.dev-dependencies`

**Count**
- unique dependency package names only
- renamed dependencies count by real `package` name when present
- `workspace = true` entries count when they resolve to external packages
- non-workspace external path dependencies count once normalized
- local path dependency identity is resolved before config checks run

**Do not count**
- repeated occurrences of the same crate name across multiple sections/tables more than once
- internal workspace-path dependencies

**Severity**
- `Error`

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Tool minimum version checks | Real problem, but version policy/upgrade cadence is not frozen yet. |
| Independent banned-crate lockfile scanning | `cargo-deny` already owns the actual policy surface; duplicating it in guardrail3 would be the wrong layer. |
| Duplicate dependency versions | Tool installation check (`cargo-dupes`) plus hook enforcement is the right layer. |

## Hardening status

### Closed gaps

- rule tests for `RS-DEPS-01..11` now use rule-specific `*_tests/` module directories instead of flat `*_tests.rs` sidecars
- tool-presence rules now have exact owned-hit coverage proving one missing tool only trips its own rule and preserves exact severities
- `RS-DEPS-CONFIG-01..07` now have exact section-ownership coverage, renamed-dependency coverage, and explicit `workspace = true` attack coverage
- non-workspace path dependencies are now checked instead of being skipped wholesale
- hybrid workspace roots are now associated with their own workspace facts, which keeps root-package `workspace = true` and workspace-path semantics from failing open
- `RS-DEPS-09` now has explicit multi-root severity coverage across service and library roots
- `RS-DEPS-10` now evaluates ancestor `.gitignore` files with last-match precedence and nested unignore handling instead of returning on the first positive match
- `RS-DEPS-11` now has explicit fail-closed coverage for malformed `guardrail3.toml`, malformed member manifests, and malformed workspace manifests needed for `workspace = true` resolution
- `RS-DEPS-11` now ignores foreign `rust.*` and crate-policy keys when deps-owned fields remain valid
- undeclared local Cargo packages under a workspace root now fail closed instead of being tolerated as ordinary external path dependencies
- nested `apps/*` and `packages/*` zones now resolve deps policy and runtime applicability by zone segment, not only by top-level prefix
- deps routing/runtime now preserve ancestor workspace roots needed for `Cargo.lock` policy when scoped app/package config enables the family
- malformed `target.*` dependency tables now fail closed through `RS-DEPS-11` without suppressing `RS-DEPS-CONFIG-01..07`
- subtree-scoped regressions now prove sibling crates do not leak into routed `RS-DEPS-CONFIG-01` and `RS-DEPS-CONFIG-05` findings
- `RS-DEPS-09/10` now keep nested non-member helper crates under a workspace root subordinate to that workspace root instead of forcing separate `Cargo.lock` policy on them

### Remaining gaps

- root-package allowlist policy is still weaker than app/package-scoped policy because the current config surface has no exact root-crate `allowed_deps` entry
- malformed dependency inputs that affect direct-dependency counting are expected to surface through `RS-DEPS-11`, not partial `RS-DEPS-CONFIG-05` counts

### Policy questions

- none at the moment
