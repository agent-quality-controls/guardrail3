//! Validate repo / validate workspace execution: orchestrates the static rule
//! pipeline, toolchain gates, marker-pair walker, and tool-presence check.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use guardrail3_ts_app_types::{
    FamilyRunner, ReportRenderer, SupportedFamily, ValidateRequest, WorkspaceCrawler,
};
use guardrail3_ts_family_runner_hooks::run_toolchain_gates;
use guardrail3_ts_validate_command::{disabled_families, execute};

use crate::cli::FamilyArg;
use crate::fs as g3ts_fs;
use crate::marker_pairs::check_marker_pair_completeness;
use crate::process::run_git;
use crate::run::CliOutput;
use crate::tool_presence::check_required_tools_present;

/// Runs the `validate workspace` subcommand against `path`: orchestrates the static
/// rule pipeline, then (unless `rules_only`) the toolchain gates, returning
/// merged `stdout` / `stderr` / `exit_code` as a `CliOutput`.
#[expect(
    clippy::too_many_arguments,
    reason = "stable CLI surface threads path, family list, three flags, and three injected adapters; collapsing into a struct hides the call shape without reducing arity"
)]
pub(crate) fn run_validate(
    path: &Path,
    family: &[FamilyArg],
    inventory: bool,
    staged: bool,
    rules_only: bool,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    if staged && !has_relevant_staged_files(path) {
        return CliOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
    }

    let request = ValidateRequest {
        workspace_root: path.to_path_buf(),
        families: family.iter().copied().flat_map(FamilyArg::expand).collect(),
        include_inventory: inventory,
    };
    let static_outcome = match execute(&request, crawler, family_runner, renderer) {
        Ok(outcome) => outcome,
        Err(error) => {
            return CliOutput {
                stdout: String::new(),
                stderr: format!("{error}\n"),
                exit_code: 1,
            };
        }
    };

    let mut stdout = static_outcome.stdout().to_owned();
    let mut stderr = static_outcome.stderr().to_owned();
    let mut exit_code = static_outcome.exit_code();

    if !rules_only {
        let disabled = disabled_families(path);
        let toolchain = run_toolchain_gates(path, &disabled, inventory);
        if !toolchain.stdout.is_empty() && stdout.trim() == "No findings." {
            stdout.clear();
        }
        if !toolchain.stdout.is_empty() {
            stdout.push_str(&toolchain.stdout);
        }
        if !toolchain.stderr.is_empty() {
            stderr.push_str(&toolchain.stderr);
        }
        if toolchain.exit_code != 0 {
            exit_code = toolchain.exit_code;
        }
    }

    CliOutput {
        stdout,
        stderr,
        exit_code,
    }
}

/// Runs the `validate repo` subcommand: validates repository-level
/// invariants (hooks, topology, marker pairs, required-tool presence). When
/// `path` is `None`, the repo root is discovered via `git rev-parse`.
pub(crate) fn run_validate_repo(
    path: Option<&Path>,
    inventory: bool,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    let Some(repo_root) = path
        .map(Path::to_path_buf)
        .or_else(|| git_root(Path::new(".")))
    else {
        return CliOutput {
            stdout: String::new(),
            stderr: "validate repo: could not resolve git repo root\n".to_owned(),
            exit_code: 1,
        };
    };

    let request = ValidateRequest {
        workspace_root: repo_root.clone(),
        families: vec![SupportedFamily::Hooks, SupportedFamily::Topology],
        include_inventory: inventory,
    };
    let outcome = match execute(&request, crawler, family_runner, renderer) {
        Ok(outcome) => outcome,
        Err(error) => {
            return CliOutput {
                stdout: String::new(),
                stderr: format!("{error}\n"),
                exit_code: 1,
            };
        }
    };

    let mut stdout = outcome.stdout().to_owned();
    let stderr = outcome.stderr().to_owned();
    let mut exit_code = outcome.exit_code();

    let adoption_findings = check_workspace_adoption(&repo_root, inventory);
    if !adoption_findings.is_empty() {
        if stdout.trim() == "No findings." {
            stdout.clear();
        }
        stdout.push_str("== workspace-adoption ==\n");
        for finding in &adoption_findings {
            let _ = writeln!(&mut stdout, "{finding}");
            if finding.starts_with("[Error]") {
                exit_code = 1;
            }
        }
    }

    let marker_findings = check_marker_pair_completeness(&repo_root);
    if !marker_findings.is_empty() {
        stdout.push_str("== marker-pairs ==\n");
        for finding in &marker_findings {
            let _ = writeln!(&mut stdout, "[Error] {finding}");
        }
        exit_code = 1;
    }

    let tool_findings = check_required_tools_present();
    if !tool_findings.is_empty() {
        stdout.push_str("== tools ==\n");
        for finding in &tool_findings {
            let _ = writeln!(&mut stdout, "[Error] {finding}");
        }
        exit_code = 1;
    }

    CliOutput {
        stdout,
        stderr,
        exit_code,
    }
}

/// Reports package roots that have not been adopted by G3TS.
fn check_workspace_adoption(repo_root: &Path, inventory: bool) -> Vec<String> {
    let mut findings = Vec::new();
    for rel_path in package_root_candidates(repo_root) {
        let abs_path = repo_root.join(&rel_path);
        let display_path = if rel_path.as_os_str().is_empty() {
            "."
        } else {
            rel_path.to_str().unwrap_or("<non-utf8>")
        };
        if abs_path.join("guardrail3-ts.toml").is_file() {
            if inventory {
                findings.push(format!(
                    "[Info] g3ts-repo/workspace-adoption-inventory {display_path} TypeScript package root is adopted"
                ));
                findings.push(format!("  package root `{display_path}` is adopted."));
            }
        } else {
            findings.push(format!(
                "[Error] g3ts-repo/unadopted-workspace {display_path} TypeScript package root is not adopted"
            ));
            findings.push(format!(
                "  `{display_path}` has package.json but no guardrail3-ts.toml. Run: g3ts init workspace --path {display_path}"
            ));
        }
    }
    findings
}

/// Returns root, apps/*, and packages/* package roots visible to repo validation.
fn package_root_candidates(repo_root: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if repo_root.join("package.json").is_file() {
        candidates.push(PathBuf::new());
    }
    for parent in ["apps", "packages"] {
        for path in g3ts_fs::read_dir_paths(&repo_root.join(parent)) {
            let Some(candidate) = package_root_child(parent, &path) else {
                continue;
            };
            candidates.push(candidate);
        }
    }
    candidates
}

/// Converts an immediate child directory into an app/package root candidate.
fn package_root_child(parent: &str, path: &Path) -> Option<PathBuf> {
    if !path.join("package.json").is_file() {
        return None;
    }
    path.file_name()
        .map(|name| PathBuf::from(parent).join(name))
}

/// Returns true when staged TS-relevant files exist inside `path`.
fn has_relevant_staged_files(path: &Path) -> bool {
    let Ok(output) = run_git(
        &["diff", "--cached", "--name-only", "--diff-filter=ACM"],
        path,
    ) else {
        return true;
    };
    if !output.status.success() {
        return true;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(is_ts_relevant_path)
}

/// Recognized TypeScript source / config extensions for the `--staged`
/// filter.
const TS_RELEVANT_EXTENSIONS: &[&str] =
    &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "css"];

/// Recognized workspace config filenames that should also force the validate
/// pipeline to run when staged.
const TS_RELEVANT_FILENAMES: &[&str] = &[
    "package.json",
    "guardrail3-ts.toml",
    ".syncpackrc",
    "cspell.json",
    ".cspell.json",
    "cspell.yaml",
    "cspell.yml",
];

/// Returns true when `p` names a file the validate pipeline cares about.
fn is_ts_relevant_path(p: &str) -> bool {
    let path = Path::new(p);
    if path.extension().is_some_and(|extension| {
        TS_RELEVANT_EXTENSIONS
            .iter()
            .any(|candidate| extension.eq_ignore_ascii_case(candidate))
    }) {
        return true;
    }
    if path
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| {
            TS_RELEVANT_FILENAMES
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
                || ts_relevant_pattern_filename(name)
        })
    {
        return true;
    }
    false
}

/// Returns true for config filename families that are defined by tool
/// conventions rather than one exact filename.
fn ts_relevant_pattern_filename(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    (name.starts_with("tsconfig")
        && Path::new(&name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json")))
        || (name.starts_with("prettier.config.") && name.len() > "prettier.config.".len())
        || name == ".prettierrc"
        || name.starts_with(".prettierrc.")
        || (name.starts_with("cspell.config.") && name.len() > "cspell.config.".len())
}

/// Returns the absolute repo root reported by `git rev-parse --show-toplevel`
/// when run inside `start`. Returns `None` when the spawn fails, the command
/// exits non-zero, or the stdout is empty.
fn git_root(start: &Path) -> Option<PathBuf> {
    let output = run_git(&["rev-parse", "--show-toplevel"], start).ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if text.is_empty() {
        return None;
    }
    Some(PathBuf::from(text))
}
