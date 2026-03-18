# Comprehensive CLI Help Generation (graf-style)

**Date:** 2026-03-16 14:11
**Task:** Add runtime help injection so every --help explains everything

## Goal
`guardrail3 --help` tells you EVERYTHING: all commands, all check IDs, all profiles, workflow, examples. An agent reading the help has zero questions.

## Architecture (from graf)
- `help_gen.rs` — assembles help text programmatically
- Injects via `after_help()` into clap command tree at runtime
- No build.rs, no codegen — pure runtime

## Help Content Per Command

### Top-level (`guardrail3 --help`)
```
GETTING STARTED:
  guardrail3 rs init --profile service     Set up Rust guardrails
  guardrail3 ts init                       Set up TypeScript guardrails
  guardrail3 rs generate                   Generate tool config files
  guardrail3 rs validate .                 Run all checks

PROFILES:
  service    Full guardrails for HTTP services (Axum/tokio). All clippy bans,
             all deny bans, tokio feature gating. Composition-root crates
             may use LazyLock for global config.
  library    Same as service + bans ALL I/O crates (axum, tokio, reqwest, sqlx)
             + global-state bans on every crate. For pure logic packages.

WORKFLOW:
  1. guardrail3 rs init --profile service    Create guardrail3.toml
  2. Edit guardrail3.toml                    Set workspace_root, crate layers
  3. guardrail3 rs generate                  Produce clippy.toml, deny.toml, etc.
  4. guardrail3 rs validate .                Check compliance
  5. guardrail3 hooks install                Install pre-commit hook
  6. guardrail3 check                        CI: verify configs not stale

COMMANDS:
  rs init [--profile service|library]    Scaffold Rust config + local overrides
  rs validate [path] [--format json]     Run Rust checks (134 checks)
  rs generate                            Produce clippy.toml, deny.toml, rustfmt.toml
  ts init                                Scaffold TypeScript config
  ts validate [path]                     Run TypeScript checks (83 checks)
  ts generate                            Produce eslint.config, tsconfig, npmrc
  validate [path]                        Auto-detect stacks, run both
  generate                               Generate all config files
  check                                  CI: verify generated files current
  diff                                   Dry-run generate with diffs
  hooks install                          Install pre-commit hook
  hooks validate                         Check hook setup
  list-modules                           List embedded config modules
  show-module <name>                     Print module content

OUTPUT:
  --format text     Colored terminal output (default)
  --format json     Machine-readable JSON
  --format md       Markdown tables

SCOPE (for validate):
  --staged           Only staged files
  --dirty            Staged + unstaged
  --commits N        Files changed in last N commits
  --files a.rs b.rs  Specific files
```

### `guardrail3 rs validate --help`
After the clap-generated args, append ALL Rust check IDs grouped:
```
RUST CHECKS (134 total):

Config completeness:
  R1-R3     clippy.toml existence + thresholds
  R4-R7     Method/type ban completeness
  R8-R20    deny.toml structure, bans, licenses, sources
  R21-R25   rustfmt + toolchain settings
  R26-R29   Cargo.toml workspace lints

Source scan (AST-based, syn):
  R30-R31   Crate-level #![allow] without reason
  R32-R33   Item-level #[allow] without reason
  ...
```

### `guardrail3 rs init --help`
```
PROFILES:
  service    HTTP service guardrails. Includes:
             - 6 clippy method ban modules (env-vars, filesystem, http, etc.)
             - 4 clippy type ban modules (collections, sync, filesystem, global-state)
             - 16 deny.toml ban categories (json, tls, http, async, etc.)
             - Pre-commit hook with cargo fmt/clippy/deny/test/dupes

  library    Everything in service, PLUS:
             - Bans ALL I/O crates (axum, tokio, reqwest, sqlx, etc.)
             - Global-state bans on ALL crates (no LazyLock anywhere)
             - For pure logic packages with zero side effects

FILES CREATED:
  guardrail3.toml              Project config (profile, workspace, crate layers)
  local/clippy-methods.toml    Extra disallowed methods
  local/clippy-types.toml      Extra disallowed types
  local/deny-bans.toml         Extra crate bans
  local/deny-skip.toml         Advisory skip list
  local/deny-feature-bans.toml Feature bans
  release-plz.toml             Release automation (service only)
  cliff.toml                   Changelog generation (service only)

EXAMPLES:
  guardrail3 rs init --profile service          New service project
  guardrail3 rs init --profile library           New library
  guardrail3 rs init --profile service --force   Overwrite existing
```

## Implementation

### Check ID Registry
Define check IDs as const arrays in each check module:
```rust
pub const CHECKS: &[(&str, &str)] = &[
    ("R30", "Crate-level #![allow] without reason comment"),
    ("R31", "Crate-level #![allow(unused_crate_dependencies)] inventory"),
    ...
];
```

`help_gen.rs` collects these from all modules to build the check list.

### help_gen.rs structure
```rust
pub fn inject_help(cmd: clap::Command) -> clap::Command {
    let cmd = cmd.after_help(top_level_help());
    inject_rs_help(inject_ts_help(cmd))
}

fn inject_rs_help(cmd: clap::Command) -> clap::Command {
    cmd.mut_subcommand("rs", |rs| {
        rs.mut_subcommand("validate", |v| v.after_help(rs_validate_help()))
          .mut_subcommand("init", |i| i.after_help(rs_init_help()))
    })
}
```

## Depends On
- Hex arch refactor — help_gen.rs lives in adapters/inbound/cli/
- Per-crate allowlists — help needs to document R-DEPS-01, R-DEPS-02

## Files to Create
- src/adapters/inbound/help_gen.rs (or src/help_gen.rs depending on final structure)
- Update main.rs to inject help
