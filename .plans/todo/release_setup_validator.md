# Release Setup Validator — guardrail3 check for crates.io publishability

## Problem

Publishing Rust crates to crates.io requires 8-10 config files across the repo with non-obvious interactions. If one file is wrong, releases silently fail or publish garbage. This setup needs to work across 10+ repos with different structures (single crate, workspace, multi-workspace, monorepo with binaries).

## What this check validates

A new guardrail3 command (or extension of `validate`) that scans any Rust repo and reports what's needed for automated publishing. Structure-agnostic — discovers layout by reading Cargo.toml files.

### Per-crate checks (for each publishable Cargo.toml)

A crate is "publishable" if it has `[package]` and does NOT have `publish = false`.

| Check | What | How to detect |
|-------|------|--------------|
| R-PUB-01 | `description` field present and non-empty | Read Cargo.toml `[package].description` |
| R-PUB-02 | `license` field present | Read `[package].license` or `[package].license-file` |
| R-PUB-03 | `repository` field present | Read `[package].repository` |
| R-PUB-04 | `readme` field points to existing file | Read `[package].readme`, check file exists |
| R-PUB-05 | README is not a stub | File > 200 bytes, contains at least one `#` heading |
| R-PUB-06 | `keywords` present (max 5) | Read `[package].keywords` |
| R-PUB-07 | `categories` present | Read `[package].categories` |
| R-PUB-08 | `version` follows semver | Parse `[package].version` |
| R-PUB-09 | `cargo publish --dry-run` passes | Run command, check exit code 0 |
| R-PUB-10 | No path dependencies that aren't also publishable | Check `[dependencies]` for `path = "..."` — each must point to another publishable crate or be a dev-dependency |

### Repo-level checks

| Check | What | How to detect |
|-------|------|--------------|
| R-REL-01 | LICENSE file exists at repo root | Check for LICENSE, LICENSE-MIT, LICENSE-APACHE, etc. |
| R-REL-02 | `release-plz.toml` exists | File exists at root |
| R-REL-03 | release-plz.toml lists all publishable crates | Parse TOML, compare `[[package]]` names against discovered publishable crates |
| R-REL-04 | `cliff.toml` exists (changelog config) | File exists at root |
| R-REL-05 | GitHub Actions release workflow exists | `.github/workflows/` contains a YAML with `release-plz` reference |
| R-REL-06 | CI has `cargo publish --dry-run` job | Parse CI YAML for `cargo publish --dry-run` |
| R-REL-07 | `CARGO_REGISTRY_TOKEN` secret likely set | Can't verify directly, but can check if release workflow references it |

### Binary distribution checks (for repos with `[[bin]]` targets)

| Check | What | How to detect |
|-------|------|--------------|
| R-BIN-01 | Binary has a release workflow | `.github/workflows/` has a YAML that builds `--release` and uses `action-gh-release` or similar |
| R-BIN-02 | Release workflow builds for linux-x86_64 at minimum | Check matrix/target in workflow |
| R-BIN-03 | `[package.metadata.binstall]` present (optional but recommended) | Read Cargo.toml |

## Output format

Same as other guardrail3 checks — `Vec<CheckResult>` with ID, severity, title, message:

```
R-PUB-01 [error] limit3r: missing `description` in Cargo.toml
R-PUB-04 [warn]  shedul3r-rs-sdk: README.md is 42 bytes (likely a stub)
R-REL-01 [error] repo: LICENSE file not found
R-REL-03 [warn]  release-plz.toml: missing package "pipelin3r" (found in packages/pipelin3r/Cargo.toml)
R-BIN-01 [warn]  shedul3r: no binary release workflow found
```

Severities:
- **error**: blocks publishing, must fix
- **warn**: publishing works but quality is poor
- **info**: nice-to-have

## Discovery algorithm

```
1. Find all Cargo.toml files (recursive, skip target/ and .git/)
2. For each:
   a. Parse [package] section
   b. If publish = false → skip
   c. If no [package] → workspace root, not a crate, skip
   d. Otherwise → publishable crate, run R-PUB-* checks
   e. If [[bin]] present → also run R-BIN-* checks
3. Run R-REL-* repo-level checks once
4. Cross-reference: release-plz.toml packages vs discovered publishable crates
```

This works for any structure: single crate, workspace, multi-workspace, monorepo with excluded workspaces. No hardcoded paths.

## Implementation location

New module in guardrail3: `src/rs/validate/release.rs` (or `src/rs/validate/publish.rs`).

Wired into the existing `guardrail3 rs validate` command, or as a separate `guardrail3 release validate` subcommand.

`cargo publish --dry-run` (R-PUB-09) is optional and slow — should be behind a `--thorough` flag. All other checks are fast (file reads + TOML parsing).

## Files needed for a healthy publishable repo (reference)

```
repo/
├── LICENSE                        # R-REL-01
├── release-plz.toml               # R-REL-02
├── cliff.toml                     # R-REL-04
├── .github/workflows/
│   ├── ci.yml                     # R-REL-06 (publish dry-run job)
│   └── release.yml                # R-REL-05 (release-plz action)
│
├── packages/some-crate/
│   ├── Cargo.toml                 # R-PUB-01 through R-PUB-10
│   └── README.md                  # R-PUB-04, R-PUB-05
│
└── apps/some-binary/
    ├── Cargo.toml                 # R-BIN-03
    └── .github/workflows/
        └── release-binary.yml     # R-BIN-01, R-BIN-02
```

## Existing art

- `cargo publish --dry-run` catches most R-PUB issues but doesn't check repo-level setup
- `release-plz` has some validation but only for its own config
- `cargo-audit` checks security, not publishability
- No existing tool combines all of these into one check

## First target repos

1. pipelin3r (3 libraries + 1 binary)
2. low-expectations (1 library)
3. websmasher-parsers (multiple libraries)
