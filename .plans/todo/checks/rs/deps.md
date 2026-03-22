# RS-DEPS — Tool + dependency checker (9 rules)

**Input:** Tool PATH checks + Cargo.lock + .gitignore + guardrail3.toml allowlist
**Current code:** `dependency_scan.rs`, `dependency_allowlist.rs`

## Tool installation rules (dependency_scan.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-01 | R45 | Error | cargo-deny installed on PATH | Implemented |
| RS-DEPS-02 | R46 | Error | cargo-machete installed on PATH | Implemented |
| RS-DEPS-03 | R47 | Warn | cargo-dupes installed on PATH (recommended) | Implemented |
| RS-DEPS-04 | R48 | Error | gitleaks installed on PATH | Implemented |
| RS-DEPS-05 | R49 | Warn | CLAUDE.md exists at project root | Implemented |

## Dependency rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-06 | R50 | — | Banned crate in Cargo.lock | REMOVED (cargo-deny handles this now) |

## Allowlist rules (dependency_allowlist.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DEPS-07 | R-DEPS-01 | Error | Unauthorized dependency not in crate's allowed_deps list | Implemented |
| RS-DEPS-08 | R-DEPS-02 | Warn | Library crate has no dependency allowlist configured | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-DEPS-09 | Error/Info | Cargo.lock committed. Services/binaries: Error if missing (reproducibility + cargo-deny needs it). Also Error if .gitignore contains `Cargo.lock`. Libraries: Info if missing (Cargo guidance says optional). | Planned |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Yanked dependency detection | Requires network (crates.io API or local index). cargo/cargo-deny handle this. |
| Duplicate dependency versions | Tool installation check (RS-DEPS-03 for cargo-dupes) is the right layer. Pre-commit hook runs cargo-dupes. |
