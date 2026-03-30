# RS-DEPS — Tool + dependency policy checker (12 implemented rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/deps.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/deps.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** Tool PATH checks + Cargo.lock + .gitignore + guardrail3.toml allowlists + Cargo.toml dependency tables
**Current code:** `apps/guardrail3/crates/app/rs/families/deps/`

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
- standalone package roots that are not members of a workspace

The family must not collapse lockfile policy to repo-root-only behavior, and it must not confuse crate-local dependency policy with workspace-root lockfile ownership.

## Policy ownership

Dependency allowlist/profile policy comes from validation-root `guardrail3.toml`.

That means:
- the family resolves crate-local allowlist/profile policy from the one root policy surface
- per-app / per-package entries inside that root policy file are still crate-local policy once resolved
- future verification must not invent nearest-local `guardrail3.toml` semantics unless the plan changes explicitly

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
| RS-DEPS-05 | R-DEPS-01 | Error | Unauthorized external dependency in `[dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-06 | — | Error | Unauthorized external dependency in `[build-dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-07 | — | Warn | Unauthorized external dependency in `[dev-dependencies]` not in crate's `allowed_deps` list | Implemented |
| RS-DEPS-08 | R-DEPS-02 | Warn | Library crate has no dependency allowlist configured | Implemented |

Allowlist semantics:
- no `allowed_deps` configured means `RS-DEPS-05..07` stay silent; only `RS-DEPS-08` warns for library-profile crates
- path dependencies are skipped only when they resolve to workspace package paths
- `workspace = true` is **not** automatically skipped
- if `workspace = true` resolves to an external workspace dependency, it must still be allowlisted
- renamed dependencies must be checked against the real `package` name when present

Section ownership:
- `RS-DEPS-05` owns `[dependencies]`
- `RS-DEPS-06` owns `[build-dependencies]`
- `RS-DEPS-07` owns `[dev-dependencies]`
- `RS-DEPS-12` owns the direct-dependency cap across both top-level and target-specific dependency tables
- target-specific dependency tables are still outside the `RS-DEPS-05..07` allowlist contract

## Lockfile rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-09 | Error/Info | `Cargo.lock` committed for each Rust root. Services/binaries: Error if missing. Libraries: Info if missing. | Implemented |
| RS-DEPS-10 | Error | No relevant `.gitignore` may ignore a Rust root's `Cargo.lock` | Implemented |

## Input / parse integrity rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-11 | Error | Required dependency-policy inputs unreadable or unparseable: member Cargo.toml, workspace Cargo.toml for `workspace = true` resolution, or `guardrail3.toml` when needed for profile/allowlist policy. | Implemented |

## Input integrity / fail-closed expectations

The family depends on:
- readable local package manifests
- readable workspace manifests when `workspace = true` resolution needs them
- readable validation-root `guardrail3.toml` when crate policy/profile needs it
- readable `.gitignore` inputs used for lockfile masking checks

Malformed required inputs must surface through `RS-DEPS-11` rather than silently suppressing allowlist or lockfile findings.

## Direct dependency cap rule

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-12 | Error | More than 25 unique direct dependency package names on one crate across owned direct dependency sections/tables. | Implemented |

### RS-DEPS-12 — Direct dependency count cap

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
- non-workspace external path dependencies count

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
- `RS-DEPS-05..07` now have exact section-ownership coverage, renamed-dependency coverage, and explicit `workspace = true` attack coverage
- non-workspace path dependencies are now checked instead of being skipped wholesale
- hybrid workspace roots are now associated with their own workspace facts, which keeps root-package `workspace = true` and workspace-path semantics from failing open
- `RS-DEPS-09` now has explicit multi-root severity coverage across service and library roots
- `RS-DEPS-10` now evaluates ancestor `.gitignore` files with last-match precedence and nested unignore handling instead of returning on the first positive match
- `RS-DEPS-11` now has explicit fail-closed coverage for malformed `guardrail3.toml`, malformed member manifests, and malformed workspace manifests needed for `workspace = true` resolution

### Remaining gaps

- `target.*.{dependencies,build-dependencies,dev-dependencies}` tables are still not part of `RS-DEPS-05..07` allowlist discovery; only `RS-DEPS-12` owns them today
- malformed dependency inputs that affect direct-dependency counting are expected to surface through `RS-DEPS-11`, not partial `RS-DEPS-12` counts

### Policy questions

- none at the moment
