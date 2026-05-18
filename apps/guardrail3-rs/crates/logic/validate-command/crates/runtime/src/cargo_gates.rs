//! Cargo gate execution: derives gate commands from family hook contracts.
//!
//! Cargo gate commands are emitted by walking each enabled family's
//! `hook_contract()` and resolving each `G3HookCommandRequirement` to the argv
//! that must run for the adopted workspace. Adding a new required command to a
//! family's contract automatically appears here. Variants with no cargo-gate
//! command (e.g. `Gitleaks`, `G3RsValidatePath`) are skipped because the hook
//! executes them inline or they map to the in-binary validator entry point
//! itself.

use std::path::Path;
use std::process::{Command, Stdio};

use cargo_toml_parser::types::CargoStringFieldState;
use g3rs_hooks_contract_types::G3HookCommandRequirement;
use g3rs_rust_family_contracts::{RustFamily, family_hook_contract};
use guardrail3_rs_app_types::SupportedFamily;

/// Returns the cargo gate commands implied by the enabled families.
///
/// Each entry is the concrete argv of one `G3HookCommandRequirement` declared
/// in some enabled family's hook contract. Duplicate commands across families
/// are deduplicated by argv equality.
///
/// `rust_source_staged` is `true` when at least one staged path inside the
/// workspace root has a `.rs` extension. The single skip rule is:
/// `CargoDupesExcludeTests` is suppressed when `staged && !rust_source_staged`.
/// All other contract-derived commands run unconditionally for their enabled
/// families.
#[must_use]
pub fn cargo_gate_commands(
    workspace_root: &Path,
    enabled_families: &[SupportedFamily],
    staged: bool,
    rust_source_staged: bool,
) -> Vec<Vec<String>> {
    let dupes_excluded_skipped = staged && !rust_source_staged;
    let workspace_msrv = workspace_rust_version(workspace_root);
    let mut seen: Vec<Vec<String>> = Vec::new();
    for family in enabled_families {
        let Some(rust_family) = rust_family_for(*family) else {
            continue;
        };
        collect_family_commands(
            rust_family,
            dupes_excluded_skipped,
            workspace_msrv.as_deref(),
            &mut seen,
        );
    }
    seen
}

/// Appends the concrete argv of every non-skipped command requirement for the
/// given Rust family into `seen`, preserving argv-equality deduplication.
fn collect_family_commands(
    rust_family: RustFamily,
    dupes_excluded_skipped: bool,
    workspace_msrv: Option<&str>,
    seen: &mut Vec<Vec<String>>,
) {
    for requirement in family_hook_contract(rust_family) {
        for command_requirement in requirement.required_commands {
            push_command(
                command_requirement,
                dupes_excluded_skipped,
                workspace_msrv,
                seen,
            );
        }
    }
}

/// Resolves a single command requirement to its argv and appends it to `seen`
/// when not skipped and not already present.
fn push_command(
    command_requirement: G3HookCommandRequirement,
    dupes_excluded_skipped: bool,
    workspace_msrv: Option<&str>,
    seen: &mut Vec<Vec<String>>,
) {
    if dupes_excluded_skipped
        && command_requirement == G3HookCommandRequirement::CargoDupesExcludeTests
    {
        return;
    }
    let Some(argv) = concrete_gate_command(command_requirement, workspace_msrv) else {
        return;
    };
    if !seen.contains(&argv) {
        seen.push(argv);
    }
}

/// Resolves a hook command requirement to the argv used by in-binary cargo gates.
fn concrete_gate_command(
    command_requirement: G3HookCommandRequirement,
    workspace_msrv: Option<&str>,
) -> Option<Vec<String>> {
    if command_requirement == G3HookCommandRequirement::CargoMsrvVerifyCargoCheckLocked {
        let mut argv = vec!["cargo", "msrv", "verify"];
        if let Some(msrv) = workspace_msrv {
            argv.extend(["--rust-version", msrv]);
        }
        argv.extend(["--", "cargo", "check", "--locked"]);
        return Some(argv.into_iter().map(str::to_owned).collect());
    }

    command_requirement
        .concrete_command()
        .map(|argv| argv.iter().map(|token| (*token).to_owned()).collect())
}

/// Reads the workspace root Cargo.toml rust-version used by the MSRV gate.
fn workspace_rust_version(workspace_root: &Path) -> Option<String> {
    let cargo_toml = crate::fs::read_to_string(&workspace_root.join("Cargo.toml")).ok()?;
    let document = cargo_toml_parser::parse_document(&cargo_toml).ok()?;
    match cargo_toml_parser::document::root_package_string_field(&document, "rust-version") {
        CargoStringFieldState::Value(version) => Some(version.to_owned()),
        CargoStringFieldState::Missing
        | CargoStringFieldState::Inherit
        | CargoStringFieldState::WrongType(_) => None,
    }
}

/// Maps a `SupportedFamily` to its Rust hook-contract family. Families that
/// have no concrete cargo gate (e.g. `Hooks`, which is purely a meta family)
/// return `None` and are skipped by the gate walker.
const fn rust_family_for(family: SupportedFamily) -> Option<RustFamily> {
    match family {
        SupportedFamily::Topology => Some(RustFamily::Topology),
        SupportedFamily::Toolchain => Some(RustFamily::Toolchain),
        SupportedFamily::Fmt => Some(RustFamily::Fmt),
        SupportedFamily::Cargo => Some(RustFamily::Cargo),
        SupportedFamily::Clippy => Some(RustFamily::Clippy),
        SupportedFamily::Deny => Some(RustFamily::Deny),
        SupportedFamily::Code => Some(RustFamily::Code),
        SupportedFamily::Arch => Some(RustFamily::Arch),
        SupportedFamily::Deps => Some(RustFamily::Deps),
        SupportedFamily::Garde => Some(RustFamily::Garde),
        SupportedFamily::Test => Some(RustFamily::Test),
        SupportedFamily::Release => Some(RustFamily::Release),
        SupportedFamily::Apparch => Some(RustFamily::Apparch),
        SupportedFamily::Hooks => None,
    }
}

/// Outcome of one cargo gate invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoGateOutcome {
    /// The argv tokens that were executed.
    command: Vec<String>,
    /// The exit code returned by the process (127 when spawn failed).
    exit_code: i32,
}

impl CargoGateOutcome {
    /// Returns the command tokens that were run.
    #[must_use]
    pub fn command(&self) -> &[String] {
        &self.command
    }

    /// Returns the process exit code.
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        self.exit_code
    }

    /// Returns true when this gate succeeded.
    #[must_use]
    pub const fn ok(&self) -> bool {
        self.exit_code == 0
    }
}

/// Runs the cargo gate sequence with the given working directory and target dir.
///
/// Delegated tool output is suppressed so validator output stays deterministic.
/// Failures are reported through the command and exit code stored in the
/// resulting `CargoGateOutcome`.
///
/// Returns the per-gate outcomes in execution order. Stops at the first failure.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized cargo gate runner"
)]
#[must_use]
pub fn run_cargo_gates(
    cwd: &Path,
    cargo_target_dir: &Path,
    commands: &[Vec<String>],
) -> Vec<CargoGateOutcome> {
    let mut outcomes = Vec::new();
    for cmd in commands {
        let Some((program, args)) = cmd.split_first() else {
            continue;
        };
        let mut command = Command::new(program);
        let stdout = if suppress_gate_stdout(cmd.as_slice()) {
            Stdio::null()
        } else {
            Stdio::inherit()
        };
        let _ = command
            .args(args)
            .current_dir(cwd)
            .env("CARGO_TARGET_DIR", cargo_target_dir)
            .env("CARGO_TERM_COLOR", "never")
            .env("CLICOLOR", "0")
            .env("CLICOLOR_FORCE", "0")
            .env("NO_COLOR", "1")
            .env("TERM", "dumb")
            .stdout(stdout)
            .stderr(Stdio::null());
        let exit_code = command
            .status()
            .map_or(127, |status| status.code().unwrap_or(1));
        let outcome = CargoGateOutcome {
            command: cmd.clone(),
            exit_code,
        };
        let failed = !outcome.ok();
        outcomes.push(outcome);
        if failed {
            break;
        }
    }
    outcomes
}

/// Returns true when a cargo gate's output should not be shown in normal
/// validator output.
#[must_use]
pub(crate) const fn suppress_gate_stdout(cmd: &[String]) -> bool {
    !cmd.is_empty()
}

/// Returns true if at least one of the staged paths is Rust-relevant (would be
/// validated by the cargo workflow).
#[must_use]
pub fn any_rust_relevant(staged_paths: &[String]) -> bool {
    staged_paths.iter().any(|path| is_rust_relevant_path(path))
}

/// Returns true if at least one staged path has a `.rs` extension.
#[must_use]
pub fn any_rust_source(staged_paths: &[String]) -> bool {
    staged_paths.iter().any(|path| has_rs_extension(path))
}

/// Filters staged paths to those that fall under the given workspace root
/// (the workspace root is given relative to the repo root, e.g. `apps/guardrail3-rs`).
#[must_use]
pub fn paths_under_workspace(staged_paths: &[String], workspace_rel: &str) -> Vec<String> {
    let prefix = if workspace_rel.is_empty() || workspace_rel == "." {
        String::new()
    } else {
        format!("{}/", workspace_rel.trim_end_matches('/'))
    };
    staged_paths
        .iter()
        .filter(|path| prefix.is_empty() || path.starts_with(&prefix))
        .cloned()
        .collect()
}

/// Returns true if the given path is Rust-relevant.
#[must_use]
pub fn is_rust_relevant_path(path: &str) -> bool {
    if has_rs_extension(path) {
        return true;
    }
    let basename = path.rsplit('/').next().unwrap_or(path);
    matches!(
        basename,
        "Cargo.toml"
            | "Cargo.lock"
            | "rust-toolchain"
            | "rust-toolchain.toml"
            | "rustfmt.toml"
            | "clippy.toml"
            | "deny.toml"
            | ".rustfmt.toml"
            | ".clippy.toml"
            | ".deny.toml"
            | "guardrail3-rs.toml"
            | "config"
            | "config.toml"
    ) || path.contains(".cargo/config")
}

/// Returns true when `path`'s extension is `rs` (case-insensitive).
fn has_rs_extension(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
}
