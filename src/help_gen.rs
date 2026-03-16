//! Runtime help injection for the CLI.
//!
//! Injects comprehensive `after_help` text into clap commands so that
//! `guardrail3 --help`, `guardrail3 rs validate --help`, etc. show
//! all check IDs, profiles, workflow, and examples.

use clap::Command;

/// Inject comprehensive help text into every relevant subcommand.
pub fn inject_help(cmd: Command) -> Command {
    let cmd = cmd
        .after_help(TOP_LEVEL_HELP)
        .after_long_help(TOP_LEVEL_HELP);
    inject_ts_help(inject_rs_help(cmd))
}

fn inject_rs_help(cmd: Command) -> Command {
    cmd.mut_subcommand("rs", |rs| {
        rs.mut_subcommand("validate", |v| {
            v.after_help(RS_VALIDATE_HELP)
                .after_long_help(RS_VALIDATE_HELP)
        })
        .mut_subcommand("init", |i| {
            i.after_help(RS_INIT_HELP).after_long_help(RS_INIT_HELP)
        })
    })
}

fn inject_ts_help(cmd: Command) -> Command {
    cmd.mut_subcommand("ts", |ts| {
        ts.mut_subcommand("validate", |v| {
            v.after_help(TS_VALIDATE_HELP)
                .after_long_help(TS_VALIDATE_HELP)
        })
        .mut_subcommand("init", |i| {
            i.after_help(TS_INIT_HELP).after_long_help(TS_INIT_HELP)
        })
    })
}

// ---------------------------------------------------------------------------
// Top-level help
// ---------------------------------------------------------------------------

const TOP_LEVEL_HELP: &str = "\
QUICK START (single Rust crate):
  guardrail3 rs init --profile service .
  guardrail3 rs generate
  guardrail3 rs validate .

QUICK START (Rust workspace / monorepo):
  guardrail3 rs init --profile service .
  # Edit guardrail3.toml — see CONFIG REFERENCE below
  guardrail3 rs generate
  guardrail3 rs validate .

QUICK START (TypeScript project):
  guardrail3 ts init .
  guardrail3 ts generate
  guardrail3 ts validate .

QUICK START (monorepo with both Rust + TypeScript):
  guardrail3 rs init --profile service .
  guardrail3 ts init .
  # Edit guardrail3.toml — see CONFIG REFERENCE below
  guardrail3 rs generate && guardrail3 ts generate
  guardrail3 validate .

PROFILES (Rust only — TypeScript has no profiles):
  service   For HTTP services (Axum/tokio). Bans dangerous methods (std::fs,
            process::exit, env mutation) and types (HashMap→BTreeMap,
            Mutex→parking_lot). Allows LazyLock in composition-root crates.
  library   Everything in service PLUS bans ALL I/O crates (axum, tokio,
            reqwest, sqlx) and global-state types (LazyLock, OnceLock) in
            every crate. For pure logic packages with zero side effects.

WORKFLOW — WHAT EACH STEP DOES:

  1. rs init --profile service     Creates guardrail3.toml + local/ override dir.
     │                             guardrail3.toml defines your profile, workspace
     │                             root, and per-crate settings.
     │
  2. Edit guardrail3.toml          Configure per-crate profiles and dependency
     │                             allowlists (see CONFIG REFERENCE below).
     │
  3. rs generate                   Reads guardrail3.toml and PRODUCES actual tool
     │                             config files that cargo clippy, cargo deny, etc.
     │                             read: clippy.toml, deny.toml, rustfmt.toml,
     │                             rust-toolchain.toml, Cargo.toml [lints] section.
     │                             Also produces per-crate clippy.toml for workspace
     │                             crates with custom layers/profiles.
     │
  4. rs validate .                 Runs ALL checks against the project. Reports
     │                             errors/warnings/info. Does NOT modify files.
     │                             Exit code 1 if any errors found.
     │
  5. hooks install                 Generates and installs a pre-commit hook that
     │                             runs: gitleaks, cargo fmt, cargo clippy,
     │                             cargo-deny, cargo-machete, cargo test,
     │                             cargo-dupes, structural health checks.
     │
  6. check (CI)                    Verifies generated files match what 'generate'
                                   would produce. Fails if configs are stale.
                                   Add to CI: guardrail3 check

CONFIG REFERENCE (guardrail3.toml):

  Single crate:
    [profile]
    name = \"service\"               # or \"library\"
    [rust]
    workspace_root = \".\"

  Workspace with per-crate settings:
    [profile]
    name = \"service\"               # workspace default
    [rust]
    workspace_root = \".\"

    [rust.crates.my-api]            # HTTP service — full access
    profile = \"service\"
    layer = \"composition-root\"     # allows LazyLock for global config

    [rust.crates.my-domain]         # pure logic — locked down
    profile = \"library\"
    layer = \"pure\"
    allowed_deps = [\"serde\", \"thiserror\", \"chrono\"]

    [rust.crates.my-sdk]            # HTTP client library — needs network
    profile = \"library\"
    allowed_deps = [\"serde\", \"reqwest\", \"tokio\", \"thiserror\"]

  Per-crate fields:
    profile       \"service\" or \"library\" — overrides workspace profile for this crate
    layer         \"composition-root\" (allows global state) or \"pure\" (bans it)
    allowed_deps  Dependency allowlist — any dep NOT listed is an error (R-DEPS-01)
                  Only checks [dependencies], not [dev-dependencies] or [build-dependencies]

  local/ overrides (created by init):
    local/clippy-methods.toml     Extra disallowed methods
    local/clippy-types.toml       Extra disallowed types
    local/deny-bans.toml          Extra crate bans
    local/deny-skip.toml          Advisory skip entries
    local/deny-feature-bans.toml  Feature bans

COMMANDS:
  rs init [--profile service|library]    Scaffold Rust config + local overrides
  rs validate [path] [--format json]     Run Rust checks
  rs generate                            Produce clippy.toml, deny.toml, rustfmt.toml
  ts init                                Scaffold TypeScript config
  ts validate [path]                     Run TypeScript checks
  ts generate                            Produce eslint.config, tsconfig, npmrc
  validate [path]                        Auto-detect stacks, run both
  generate                               Generate all config files
  check                                  CI: verify generated files current
  diff                                   Dry-run generate with diffs
  hooks install                          Install pre-commit hook
  hooks validate                         Check hook setup
  list-modules                           List embedded config modules
  show-module <name>                     Print module content

OUTPUT FORMATS:
  --format text    Colored terminal (default)
  --format json    Machine-readable JSON
  --format md      Markdown tables

SCOPE (for validate):
  --staged           Only staged files
  --dirty            Staged + unstaged changes
  --commits N        Files changed in last N commits
  --files a.rs b.rs  Specific files only

DOMAIN FILTERS (for validate):
  --code             Only code quality checks
  --architecture     Only architecture checks
  --release          Only release readiness checks
  --tests            Only test quality checks
  --thorough         Run slow checks (cargo publish --dry-run, etc.)";

// ---------------------------------------------------------------------------
// rs validate help
// ---------------------------------------------------------------------------

const RS_VALIDATE_HELP: &str = "\
RUST CHECKS:

CONFIG COMPLETENESS:
  R1        clippy.toml exists
  R2        clippy.toml max-struct-bools threshold
  R3        clippy.toml type-complexity-threshold
  R4        Clippy method ban completeness
  R5        Clippy type ban completeness
  R6        Missing method bans (inventory)
  R7        Missing type bans (inventory)
  R8        deny.toml exists and parses
  R9        deny.toml [advisories] settings
  R10       deny.toml [bans] settings
  R11       deny.toml multiple-versions strategy
  R12       deny.toml crate ban completeness
  R13       deny.toml crate ban inventory
  R14       deny.toml [licenses] settings
  R15       deny.toml allowed licenses list
  R16       deny.toml [sources] settings
  R17       deny.toml feature-bans completeness
  R18       deny.toml feature-bans inventory
  R19       deny.toml skip list inventory
  R20       deny.toml advisory ignore inventory
  R21       rustfmt.toml exists
  R22       rustfmt.toml settings (edition, imports, etc.)
  R23       rustfmt.toml extra settings
  R24       rust-toolchain.toml exists
  R25       rust-toolchain.toml settings (channel, components)
  R26       Cargo.toml workspace lints completeness
  R27       Cargo.toml workspace lint inventory
  R28       Cargo.toml lint inheritance per crate
  R29       Cargo.toml lint inheritance issues

SOURCE SCAN (AST-based via syn — immune to strings/comments):
  R30       Crate-level #![allow] without reason comment (error)
  R31       Crate-level #![allow(unused_crate_dependencies)] inventory (info)
  R32       Item-level #[allow] without reason comment (error)
  R33       Item-level #[allow] with reason (info inventory)
  R34       #[garde(skip)] without reason comment (error)
  R35       #[garde(skip)] with reason (info inventory)
  R36       EXCEPTION comments in config files
  R37       cfg_attr with allow
  R38       File length > 500 lines (error)
  R39       File length > 400 lines (warn)
  R40       Use statement count > 20 (error)
  R41       Use statement count > 15 (warn)
  R42       Unsafe usage
  R43       todo!/unimplemented! macros
  R44       .unwrap()/.expect() calls
  R58       Direct std::fs usage (bypassing centralized fs module)

TOOLS & DEPENDENCIES:
  R45       cargo-deny installed
  R46       cargo-machete installed
  R47       cargo-dupes installed
  R48       gitleaks installed
  R49       CLAUDE.md exists
  R50       Banned crates in Cargo.lock

ARCHITECTURE:
  R51       Dependency direction violations
  R52       Dependency graph inventory
  R53       unsafe_code = \"forbid\" in workspace lints
  R55       Workspace metadata: edition
  R56       Workspace metadata: publish
  R57       Workspace metadata: release profile

RELEASE READINESS:
  R-REL-01  Release workflow exists
  R-REL-02  Changelog configuration
  R-REL-03  Release-plz configuration

  R-PUB-02  Crate description
  R-PUB-04  Crate metadata (license, repository, etc.)
  R-PUB-05  Crate keywords/categories
  R-PUB-06  Path dependencies have version
  R-PUB-07  Git dependencies
  R-PUB-08  Crate readme
  R-PUB-09  Dependency version requirements
  R-PUB-10  Wildcard dependencies
  R-PUB-11  Unpublished dependencies
  R-PUB-12  Cargo publish dry-run (--thorough only)

BINARY RELEASE:
  R-BIN-01  Binary release workflow exists
  R-BIN-02  Linux target in release workflow
  R-BIN-03  Binstall metadata in Cargo.toml

GARDE VALIDATION:
  R-GARDE-01  Garde dependency detection
  R-GARDE-02  Garde clippy bans in clippy.toml
  R-GARDE-05  Garde validation patterns

TEST QUALITY:
  R-TEST-02  Test file naming convention
  R-TEST-03  Test module structure
  R-TEST-04  Test assertion quality
  R-TEST-05  Test coverage indicators
  R-TEST-06  Test isolation
  R-TEST-07  Test naming patterns
  R-TEST-08  Integration test structure";

// ---------------------------------------------------------------------------
// ts validate help
// ---------------------------------------------------------------------------

const TS_VALIDATE_HELP: &str = "\
TYPESCRIPT CHECKS:

ESLINT CONFIGURATION:
  T1        eslint.config.mjs exists
  T2        ESLint TypeScript plugin
  T3        ESLint import plugin
  T4        ESLint unused-imports plugin
  T5        ESLint boundaries plugin
  T6        ESLint flat config format
  T7        ESLint strict type-checking
  T8        ESLint file pattern coverage

ESLINT AUDIT (rule presence):
  T36       no-explicit-any rule
  T37       no-unused-vars rule
  T38       consistent-type-imports rule
  T39       naming-convention rule
  T40       no-floating-promises rule
  T41       no-misused-promises rule
  T42       require-await rule
  T43       no-unsafe-assignment rule
  T44       no-unsafe-member-access rule
  T45       no-unsafe-call rule
  T46       no-unsafe-return rule
  T47       no-unsafe-argument rule
  T48       restrict-template-expressions rule
  T49       boundaries/element-types rule
  T50       no-console rule
  T51       import-x/no-cycle rule

TSCONFIG:
  T9        tsconfig.json exists and parses
  T10       TypeScript strict mode settings
  T52       noUncheckedIndexedAccess
  T53       exactOptionalPropertyTypes
  T54       isolatedModules

NPMRC:
  T11       .npmrc exists
  T12       strict-peer-dependencies
  T13       auto-install-peers
  T14       Additional npmrc settings

PACKAGE.JSON:
  T15       package.json exists and parses
  T16       pnpm overrides
  T17       engine-strict / packageManager
  T18       Banned dependencies
  T55       Build script exists
  T56       Lint script exists
  T57       Type-check script exists
  T58       Test script exists

JSCPD / DUPLICATION:
  T19       .jscpd.json exists
  T20       jscpd threshold settings
  T21       jscpd reporters
  T22       jscpd file patterns
  T60       Content imports pattern
  T61       Velite configuration

SOURCE SCAN:
  T23       eslint-disable without reason (error)
  T24       eslint-disable with reason (info inventory)
  T25       eslint-disable-next-line without reason (error)
  T26       eslint-disable-next-line with reason (info inventory)
  T27       @ts-ignore usage
  T28       @ts-expect-error without reason
  T29       @ts-expect-error with reason (info inventory)
  T30       Direct process.env usage
  T31       Explicit any type usage
  T32       File length > 300 lines (error)
  T33       File length > 200 lines (warn)
  T34       IDE-generated suppressions
  T35       Coverage tool suppressions
  T59       Banned packages in node_modules

TEST QUALITY:
  T-TEST-01 Test file naming convention
  T-TEST-02 Test describe blocks
  T-TEST-03 Test assertion patterns
  T-TEST-04 Test file co-location
  T-TEST-05 Test coverage configuration";

// ---------------------------------------------------------------------------
// rs init help
// ---------------------------------------------------------------------------

const RS_INIT_HELP: &str = "\
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
  local/clippy-methods.toml    Extra disallowed methods (project-specific)
  local/clippy-types.toml      Extra disallowed types (project-specific)
  local/deny-bans.toml         Extra crate bans (project-specific)
  local/deny-skip.toml         Advisory skip list
  local/deny-feature-bans.toml Feature bans (project-specific)
  release-plz.toml             Release automation config (service only)
  cliff.toml                   Changelog generation config (service only)

EXAMPLES:
  guardrail3 rs init --profile service          New service project
  guardrail3 rs init --profile library           New library
  guardrail3 rs init --profile service --force   Overwrite existing";

// ---------------------------------------------------------------------------
// ts init help
// ---------------------------------------------------------------------------

const TS_INIT_HELP: &str = "\
WHAT IT CREATES:
  guardrail3.toml              Project config (adds [typescript] section)

AFTER INIT:
  guardrail3 ts generate       Produce eslint.config.mjs, tsconfig.json, .npmrc, .jscpd.json
  guardrail3 ts validate .     Check TypeScript compliance

EXAMPLES:
  guardrail3 ts init                     New TypeScript project
  guardrail3 ts init --force             Overwrite existing config
  guardrail3 ts init /path/to/project    Specific project directory";

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    use crate::cli::Cli;

    #[test]
    #[allow(clippy::expect_used, clippy::indexing_slicing)] // reason: test assertions
    fn inject_help_does_not_panic() {
        let cmd = Cli::command();
        let cmd = inject_help(cmd);
        // Verify help contains our injected text
        let after = cmd.get_after_help().expect("after_help set").to_string();
        assert!(after.contains("QUICK START"), "missing QUICK START in help");
        assert!(after.contains("PROFILES"), "missing PROFILES in help");
        assert!(after.contains("WORKFLOW"), "missing WORKFLOW in help");
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertions
    fn rs_validate_help_contains_check_ids() {
        let cmd = inject_help(Cli::command());
        // Navigate to rs > validate
        let rs = cmd
            .get_subcommands()
            .find(|c| c.get_name() == "rs")
            .expect("rs subcommand");
        let validate = rs
            .get_subcommands()
            .find(|c| c.get_name() == "validate")
            .expect("validate subcommand");
        let after = validate
            .get_after_help()
            .expect("after_long_help set")
            .to_string();
        assert!(after.contains("R1"));
        assert!(after.contains("R58"));
        assert!(after.contains("R-TEST-08"));
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertions
    fn ts_validate_help_contains_check_ids() {
        let cmd = inject_help(Cli::command());
        let ts = cmd
            .get_subcommands()
            .find(|c| c.get_name() == "ts")
            .expect("ts subcommand");
        let validate = ts
            .get_subcommands()
            .find(|c| c.get_name() == "validate")
            .expect("validate subcommand");
        let after = validate
            .get_after_help()
            .expect("after_long_help set")
            .to_string();
        assert!(after.contains("T1"));
        assert!(after.contains("T-TEST-05"));
    }
}
