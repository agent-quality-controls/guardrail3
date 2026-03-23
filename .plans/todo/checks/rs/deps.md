# RS-DEPS — Tool + dependency policy checker (11 rules + 1 planned hard cap)

**Input:** Tool PATH checks + Cargo.lock + .gitignore + guardrail3.toml allowlists + Cargo.toml dependency tables
**Current code:** `dependency_scan.rs`, `dependency_allowlist.rs`

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

## Tool installation rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-01 | R45 | Error | `cargo-deny` installed on PATH | Planned |
| RS-DEPS-02 | R46 | Error | `cargo-machete` installed on PATH | Planned |
| RS-DEPS-03 | R47 | Warn | `cargo-dupes` installed on PATH (recommended) | Planned |
| RS-DEPS-04 | R48 | Error | `gitleaks` installed on PATH | Planned |

## Allowlist rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-05 | R-DEPS-01 | Error | Unauthorized external dependency in `[dependencies]` not in crate's `allowed_deps` list | Planned |
| RS-DEPS-06 | — | Error | Unauthorized external dependency in `[build-dependencies]` not in crate's `allowed_deps` list | Planned |
| RS-DEPS-07 | — | Warn | Unauthorized external dependency in `[dev-dependencies]` not in crate's `allowed_deps` list | Planned |
| RS-DEPS-08 | R-DEPS-02 | Warn | Library crate has no dependency allowlist configured | Planned |

Allowlist semantics:
- no `allowed_deps` configured means `RS-DEPS-05..07` stay silent; only `RS-DEPS-08` warns for library-profile crates
- workspace path dependencies are skipped
- `workspace = true` is **not** automatically skipped
- if `workspace = true` resolves to an external workspace dependency, it must still be allowlisted
- renamed dependencies must be checked against the real `package` name when present

## Lockfile rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-09 | Error/Info | `Cargo.lock` committed for each Rust root. Services/binaries: Error if missing. Libraries: Info if missing. | Planned |
| RS-DEPS-10 | Error | No relevant `.gitignore` may ignore a Rust root's `Cargo.lock` | Planned |

## Input / parse integrity rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-11 | Error | Required dependency-policy inputs unreadable or unparseable: member Cargo.toml, workspace Cargo.toml for `workspace = true` resolution, or `guardrail3.toml` when needed for profile/allowlist policy. | Planned |

## Next-wave planned universal rule

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-12 | Error | More than 25 unique direct dependency names on one crate across direct dependency sections/tables. | Planned |

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
- unique crate names only
- `workspace = true` entries count
- path dependencies count

**Do not count**
- repeated occurrences of the same crate name across multiple sections/tables more than once

**Severity**
- `Error`

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Tool minimum version checks | Real problem, but version policy/upgrade cadence is not frozen yet. |
| Independent banned-crate lockfile scanning | `cargo-deny` already owns the actual policy surface; duplicating it in guardrail3 would be the wrong layer. |
| Duplicate dependency versions | Tool installation check (`cargo-dupes`) plus hook enforcement is the right layer. |
