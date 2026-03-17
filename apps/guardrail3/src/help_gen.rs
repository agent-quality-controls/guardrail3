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
        rs.after_help(RS_HELP)
            .after_long_help(RS_HELP)
            .mut_subcommand("validate", |v| {
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
        ts.after_help(TS_HELP)
            .after_long_help(TS_HELP)
            .mut_subcommand("validate", |v| {
                v.after_help(TS_VALIDATE_HELP)
                    .after_long_help(TS_VALIDATE_HELP)
            })
            .mut_subcommand("init", |i| {
                i.after_help(TS_INIT_HELP).after_long_help(TS_INIT_HELP)
            })
    })
}

// ---------------------------------------------------------------------------
// Top-level help (guardrail3 --help)
// ---------------------------------------------------------------------------

const TOP_LEVEL_HELP: &str = "\
===============================================================================
COMMAND REFERENCE
===============================================================================

RUST:
  guardrail3 rs init [PATH] [OPTIONS]
    --profile <service|library>        Default: service
    --force                            Overwrite existing files

  guardrail3 rs generate [PATH]
    Produces: clippy.toml, deny.toml, rustfmt.toml, rust-toolchain.toml,
    per-crate clippy.toml, .githooks/pre-commit

  guardrail3 rs validate [PATH] [OPTIONS]
    --format <text|json|md>            Default: text
    --staged                           Only git-staged files
    --dirty                            Staged + unstaged files
    --commits <N>                      Files changed in last N commits
    --files <FILE...>                  Specific files only
    --code                             Only code quality checks
    --architecture                     Only architecture checks
    --release                          Only release readiness checks
    --tests                            Only test quality checks
    --thorough                         Include slow checks (cargo publish --dry-run)
    --verbose                          Show every audit trail item individually

  guardrail3 rs check [PATH]           CI: verify configs not stale
  guardrail3 rs diff [PATH]            Dry-run of rs generate (show diffs)
  guardrail3 rs hooks-install [PATH]   Regenerate pre-commit hook only
  guardrail3 rs hooks-validate [PATH]  Check hook setup
  guardrail3 rs list-modules           List embedded config modules
  guardrail3 rs show-module <NAME>     Print module content

TYPESCRIPT:
  guardrail3 ts init [PATH] [OPTIONS]
    --force                            Overwrite existing config

  guardrail3 ts generate [PATH]
    Produces: eslint.config.mjs, tsconfig-base.json, .npmrc, .jscpd.json,
    .githooks/pre-commit

  guardrail3 ts validate [PATH] [OPTIONS]
    Same flags as rs validate.

  guardrail3 ts hooks-install [PATH]   Regenerate pre-commit hook only
  guardrail3 ts hooks-validate [PATH]  Check hook setup

===============================================================================
PROFILES (Rust only — TypeScript has no profiles)
===============================================================================

  service    For binaries and services that do I/O (HTTP servers, CLI tools).
             Services MUST use hex arch: crates/domain, crates/ports,
             crates/adapters, crates/app. Services go in apps/.
             Bans: std::fs (must use centralized module), process::exit,
               env mutation, HashMap (use BTreeMap), Mutex (use parking_lot).
             Allows: axum, tokio, reqwest, sqlx (in adapter crates only).
             Composition-root crate may use LazyLock for global config.

  library    For shared packages with zero I/O and zero side effects.
             Libraries go in packages/. Everything in service, PLUS:
             Bans ALL I/O crates: axum, tokio, reqwest, sqlx, hyper, diesel.
             Bans ALL global state: LazyLock, OnceLock, once_cell.
             MUST have allowed_deps listing every permitted dependency.

===============================================================================
ARCHITECTURE CONVENTION
===============================================================================

  Standard project layout:

    apps/                              Services (hex arch, profile = service)
      my-api/
        crates/
          domain/                      Pure types, zero deps on ports/adapters
          ports/
            inbound/                   Traits for incoming requests
            outbound/                  Traits for external services
          app/                         Business logic, uses ports via traits
          adapters/
            inbound/api/               HTTP handlers (axum, etc.)
            outbound/                  DB, HTTP clients, filesystem

    packages/                          Shared libraries (profile = library)
      my-lib/                          Pure logic, allowed_deps enforced
      my-sdk/                          May need network — allowed_deps = [\"reqwest\", ...]

  Dependency flow (enforced by R-ARCH-02):
    domain   → nothing (pure types)
    ports    → domain only
    app      → domain + ports (via trait bounds, never adapters)
    adapters → everything (implements ports, wires dependencies)

  Services in apps/ get profile = \"service\", layer = \"composition-root\"
    on the top-level crate (allows LazyLock for config/DI wiring).
    Internal crates (domain, ports, app) get layer = \"pure\".

  Libraries in packages/ get profile = \"library\" + allowed_deps.

===============================================================================
CONFIG FILE (guardrail3.toml)
===============================================================================

  Only needed for generate/check/diff. Not needed for validate.

  Single crate (service):
    [profile]
    name = \"service\"
    [rust]
    workspace_root = \".\"

  Single crate (library):
    [profile]
    name = \"library\"
    [rust]
    workspace_root = \".\"

  Workspace / monorepo:
    [profile]
    name = \"service\"
    [rust]
    workspace_root = \".\"

    # --- Services (in apps/) ---
    [rust.crates.my-api]
    profile = \"service\"
    layer = \"composition-root\"

    # --- Libraries (in packages/) ---
    [rust.crates.my-lib]
    profile = \"library\"
    layer = \"pure\"
    allowed_deps = [\"serde\", \"thiserror\"]

    [rust.crates.my-sdk]
    profile = \"library\"
    allowed_deps = [\"serde\", \"serde_json\", \"reqwest\", \"tokio\", \"thiserror\"]

  Per-crate fields:
    profile       \"service\" or \"library\" — overrides workspace default
    layer         \"composition-root\" (allows LazyLock) or \"pure\" (bans global state)
    allowed_deps  Dependency allowlist. Any [dependencies] NOT listed = error (R-DEPS-01).
                  [dev-dependencies] and [build-dependencies] are NOT checked.
                  Workspace path deps (path = \"...\") and workspace = true are NOT checked.

  local/ overrides (created by rs init):
    local/clippy-methods.toml          Extra disallowed methods
    local/clippy-types.toml            Extra disallowed types
    local/deny-bans.toml               Extra crate bans
    local/deny-skip.toml               Duplicate crate skip list
    local/deny-feature-bans.toml       Feature bans

===============================================================================
CHECK CATEGORIES
===============================================================================

  guardrail3 organizes checks into 5 categories. Core checks always run;
  optional categories can be enabled in config or selected via CLI flags.

  CORE (always on):
    Config file completeness (R1-R29, T1-T22), suppression hygiene (R30-R37,
    T23-T35), source health (R38-R42, T32-T33), tools & dependencies
    (R45-R50, T55-T59), hooks (H1-H12), deployment (D1-D5).

  ARCHITECTURE (off by default):
    Hex arch structure and dependency flow enforcement.
    Rust: R-ARCH-01..04, R-DEPS-01..02. TypeScript: T-ARCH-01..02,
    eslint boundary rules. Enable with --architecture flag or in config.

  GARDE (off by default, Rust only):
    Input boundary validation with the garde crate. Ensures every struct
    that crosses a trust boundary (Deserialize, Parser, Args, FromRow)
    derives Validate. Checks: R-GARDE-01/02/05, R34/R35 (garde skip
    hygiene). Enable with --garde flag or in config.

  TESTS (on by default):
    Test quality and organization. Rust: R-TEST-02..09. TypeScript:
    T-TEST-01..05. Checks test file structure, assertions, coverage,
    isolation, and no inline tests in src/. Enable/disable with --tests
    flag or in config.

  RELEASE (off by default, Rust only):
    Crate publish readiness. R-REL-* (release workflow, changelog,
    release-plz config), R-PUB-* (crate metadata: description, license,
    repository, etc.), R-BIN-* (binary release workflow, binstall
    metadata). Enable with --release flag or in config.

  Enable in guardrail3.toml:
    [rust.checks]
    architecture = true
    garde = true
    release = true

    [typescript.checks]
    architecture = true

  CLI flags override config for a single run:
    guardrail3 rs validate --architecture    Run only architecture checks
    guardrail3 rs validate --tests           Run only test quality checks
    guardrail3 rs validate --release         Run only release checks
    guardrail3 rs validate --garde           Run only garde checks

===============================================================================
SETUP GUIDE
===============================================================================

Step 1: IDENTIFY YOUR PROJECT TYPE and run init

  A) Single Rust service or CLI tool
     → guardrail3 rs init --profile service .

  B) Single Rust library (no I/O)
     → guardrail3 rs init --profile library .

  C) Rust workspace (apps/ + packages/)
     → guardrail3 rs init --profile service .
     → Then configure per-crate settings (Step 2)

  D) TypeScript project
     → guardrail3 ts init .

  E) Monorepo with Rust + TypeScript
     → guardrail3 rs init --profile service .
     → guardrail3 ts init .
     → Then configure per-crate settings (Step 2)

Step 2: CONFIGURE guardrail3.toml (workspaces only — skip for single crates)

  Read Cargo.toml [workspace.members] to find all crates. For each:
  - Crates in apps/   → profile = \"service\", layer = \"composition-root\"
  - Crates in packages/ → profile = \"library\", layer = \"pure\", allowed_deps = [...]

  For allowed_deps: read the crate's Cargo.toml [dependencies] and list every
  dependency explicitly. This is the allowlist — anything not listed will be
  flagged as R-DEPS-01 error on validate.

Step 3: GENERATE

    guardrail3 rs generate             (Rust configs + hooks)
    guardrail3 ts generate             (TypeScript configs + hooks)

  Reads guardrail3.toml → writes tool configs + pre-commit hook.
  Re-run after editing guardrail3.toml or updating guardrail3.

Step 4: VALIDATE

    guardrail3 rs validate .           (Rust)
    guardrail3 ts validate .           (TypeScript)

  Fix errors, re-run until clean. Exit code 1 = errors found.

Step 5: CI

    guardrail3 rs check                (fails if generated configs are stale)";

// ---------------------------------------------------------------------------
// rs --help
// ---------------------------------------------------------------------------

const RS_HELP: &str = "\
  init        Create guardrail3.toml + local/ overrides
  generate    Produce clippy.toml, deny.toml, rustfmt.toml, hooks
  validate    Run all Rust checks
  check       CI: verify generated configs are current
  diff        Dry-run generate (show what would change)
  hooks-install    Regenerate pre-commit hook
  hooks-validate   Check hook configuration
  list-modules     List embedded config modules
  show-module      Print module content";

// ---------------------------------------------------------------------------
// ts --help
// ---------------------------------------------------------------------------

const TS_HELP: &str = "\
  init        Create [typescript] section in guardrail3.toml
  generate    Produce eslint.config, tsconfig, npmrc, jscpd, hooks
  validate    Run all TypeScript checks
  hooks-install    Regenerate pre-commit hook
  hooks-validate   Check hook configuration";

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
  R30       Crate-level allow without reason (error)
  R31       Crate-level allow(unused_crate_dependencies) (info)
  R32       Item-level allow without reason (error)
  R33       Item-level allow with reason (info inventory)
  R34       garde(skip) without reason (error)
  R35       garde(skip) with reason (info inventory)
  R36       EXCEPTION comments in config files
  R37       cfg_attr with allow
  R38       File length > 500 lines (error)
  R40       Use count > 20 (error)
  R41       Use count > 15 (warn)
  R58       Direct std::fs usage (catches clippy aliased-import hole)

TOOLS & DEPENDENCIES:
  R45-R48   cargo-deny, cargo-machete, cargo-dupes, gitleaks installed
  R49       CLAUDE.md exists
  R-DEPS-01 Unauthorized dependency (not in allowed_deps)
  R-DEPS-02 Library crate without allowed_deps (warn)

ARCHITECTURE:
  R55-R57   Workspace metadata (edition, publish, release profile)
  R-ARCH-01 Service missing hex arch structure (domain/adapters layers)
  R-ARCH-02 Dependency flow violation (layer depends on forbidden layer)
  R-ARCH-03 Library depends on service internals (packages/ → apps/*/crates/)
  R-ARCH-04 Workspace member not configured in guardrail3.toml

RELEASE:
  R-REL-*   Release workflow, changelog, release-plz config
  R-PUB-*   Crate metadata (description, license, repository, etc.)
  R-BIN-*   Binary release workflow, binstall metadata

GARDE: R-GARDE-01/02/05  Garde dependency, clippy bans, input boundary structs (Deserialize/Parser/Args/FromRow) without Validate
TESTS: R-TEST-02..09     Test files, structure, assertions, coverage, isolation, no inline tests in src/";

// ---------------------------------------------------------------------------
// ts validate help
// ---------------------------------------------------------------------------

const TS_VALIDATE_HELP: &str = "\
TYPESCRIPT CHECKS:

ESLINT:
  T1-T8     eslint.config.mjs existence, plugins, strict mode, file patterns
  T36-T51   Individual ESLint rule presence (no-explicit-any, naming, promises, etc.)

TSCONFIG:
  T9-T10    tsconfig.json existence + strict settings
  T52-T54   noUncheckedIndexedAccess, exactOptionalPropertyTypes, isolatedModules

NPMRC:
  T11-T14   .npmrc existence + strict peer deps + settings

PACKAGE.JSON:
  T15-T18   package.json existence, overrides, engine-strict, banned deps
  T55-T58   Build/lint/type-check/test scripts

JSCPD:
  T19-T22   .jscpd.json existence, thresholds, reporters, patterns
  T60-T61   Content imports, Velite configuration

SOURCE SCAN (AST-based via tree-sitter — immune to strings/comments):
  T23-T26   eslint-disable without/with reason
  T27       @ts-ignore usage
  T28-T29   @ts-expect-error without/with reason
  T30       Direct process.env usage
  T31       Explicit any type usage
  T32-T33   File length (>300 error, >200 warn)
  T34-T35   IDE/coverage suppressions
  T59       Banned packages in node_modules

ARCHITECTURE:
  T-ARCH-01   TS app missing hex arch structure (domain/adapters in src/modules/)
  T-ARCH-02   Import boundary violation (layer imports from forbidden layer)

TESTS: T-TEST-01..05  Test naming, describe blocks, assertions, co-location, coverage";

// ---------------------------------------------------------------------------
// rs init help
// ---------------------------------------------------------------------------

const RS_INIT_HELP: &str = "\
PROFILES:
  service    For HTTP services, CLI tools, binaries.
             Bans dangerous methods and types. Allows I/O crates.
  library    For pure logic packages. Bans ALL I/O crates and global state.

FILES CREATED:
  guardrail3.toml              Config file (profile, workspace, crate settings)
  local/clippy-methods.toml    Extra disallowed methods
  local/clippy-types.toml      Extra disallowed types
  local/deny-bans.toml         Extra crate bans
  local/deny-skip.toml         Duplicate crate skip list
  local/deny-feature-bans.toml Feature bans
  release-plz.toml             Release automation (service only)
  cliff.toml                   Changelog generation (service only)

AFTER INIT:
  For single crates:  guardrail3 rs generate && guardrail3 rs validate .
  For workspaces:     Edit guardrail3.toml (add [rust.crates.*] sections),
                      then guardrail3 rs generate && guardrail3 rs validate .

EXAMPLES:
  guardrail3 rs init --profile service .         Service project
  guardrail3 rs init --profile library .          Pure logic library
  guardrail3 rs init --profile service --force .  Overwrite existing";

// ---------------------------------------------------------------------------
// ts init help
// ---------------------------------------------------------------------------

const TS_INIT_HELP: &str = "\
Creates or appends a [typescript] section to guardrail3.toml.
Does NOT create local/ or any Rust-specific files.

AFTER INIT:
  guardrail3 ts generate        Produce eslint.config, tsconfig, npmrc, jscpd
  guardrail3 ts validate .      Check TypeScript compliance

EXAMPLES:
  guardrail3 ts init .                  New TypeScript project
  guardrail3 ts init --force .          Overwrite existing config
  guardrail3 ts init /path/to/project   Specific directory";
