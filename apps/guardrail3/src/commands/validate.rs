use std::path::Path;

use crate::adapters::outbound::fs::RealFileSystem;
use crate::adapters::outbound::tool_runner::RealToolChecker;
use crate::app::discover;
use crate::app::hooks;
use crate::app::rs;
use crate::app::ts;
use crate::cli::ValidateArgs;
use crate::domain::report::{Report, ValidateDomains};
use crate::report;

/// Convert a repo-relative path to an absolute path string.
fn to_abs_path(project_path: &Path, relative: &str) -> String {
    project_path.join(relative).display().to_string()
}

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — stderr output and exit codes
pub fn run(args: &ValidateArgs) {
    let path = Path::new(&args.path);
    let Some(abs_path) = path.canonicalize().ok() else {
        eprintln!("Error: cannot resolve path '{}'", args.path);
        std::process::exit(1);
    };

    let run_all = !args.code && !args.architecture && !args.release && !args.tests;
    let domains = ValidateDomains {
        code: run_all || args.code,
        architecture: run_all || args.architecture,
        release: run_all || args.release,
        tests: run_all || args.tests,
    };

    let fs = RealFileSystem;
    let tc = RealToolChecker;
    let project = discover::detect_project(&fs, &abs_path);

    let scoped_files = resolve_scoped_files(args, &abs_path);
    let scoped_ref = scoped_files.as_deref();

    let mut combined_report = Report::new(abs_path.display().to_string(), {
        let mut stacks = Vec::new();
        if project.has_rust {
            stacks.push("Rust".to_owned());
        }
        if project.has_typescript {
            stacks.push("TypeScript".to_owned());
        }
        if stacks.is_empty() {
            stacks.push("Unknown".to_owned());
        }
        stacks
    });

    if project.has_rust {
        let rust_report = rs::validate::run(
            &fs,
            &abs_path,
            &project,
            scoped_ref,
            &domains,
            args.thorough,
            &tc,
        );
        for section in rust_report.sections {
            combined_report.add_section(section);
        }
    }

    if project.has_typescript {
        let ts_report = ts::validate::run(&fs, &abs_path, scoped_ref, &domains);
        for section in ts_report.sections {
            combined_report.add_section(section);
        }
    }

    // Hook and deployment checks
    let hooks_report = hooks::validate::run(
        &fs,
        &abs_path,
        project.has_rust,
        project.has_typescript,
        &domains,
        &tc,
    );
    for section in hooks_report.sections {
        combined_report.add_section(section);
    }

    match args.format.as_str() {
        "json" => report::json::print_report(&combined_report, args.inventory),
        "md" | "markdown" => report::markdown::print_report(&combined_report, args.inventory, args.verbose),
        _ => report::text::print_report(&combined_report, args.inventory, args.verbose),
    }

    // Exit with error code if errors found
    if combined_report.error_count() > 0 {
        std::process::exit(1);
    }
}

pub fn resolve_scoped_files_pub(args: &ValidateArgs, project_path: &Path) -> Option<Vec<String>> {
    resolve_scoped_files(args, project_path)
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands for scoped file detection
fn resolve_scoped_files(args: &ValidateArgs, project_path: &Path) -> Option<Vec<String>> {
    if !args.files.is_empty() {
        return Some(args.files.clone());
    }

    if args.staged {
        return git_staged_files(project_path);
    }

    if args.dirty {
        return git_dirty_files(project_path);
    }

    if let Some(n) = args.commits {
        return git_commit_files(project_path, n);
    }

    None
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands
fn git_staged_files(project_path: &Path) -> Option<Vec<String>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| to_abs_path(project_path, l))
        .collect();

    Some(files)
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands
fn git_dirty_files(project_path: &Path) -> Option<Vec<String>> {
    let staged = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(project_path)
        .output()
        .ok()?;

    let unstaged = std::process::Command::new("git")
        .args(["diff", "--name-only"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !staged.status.success() || !unstaged.status.success() {
        return None;
    }

    let mut files: Vec<String> = Vec::new();
    for line in String::from_utf8_lossy(&staged.stdout).lines() {
        let full = project_path.join(line).display().to_string();
        if !files.contains(&full) {
            files.push(full);
        }
    }
    for line in String::from_utf8_lossy(&unstaged.stdout).lines() {
        let full = project_path.join(line).display().to_string();
        if !files.contains(&full) {
            files.push(full);
        }
    }

    Some(files)
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands
fn git_commit_files(project_path: &Path, n: usize) -> Option<Vec<String>> {
    let output = std::process::Command::new("git")
        .args([
            "log",
            "--name-only",
            &format!("-{n}"),
            "--diff-filter=ACM",
            "--pretty=format:",
        ])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| to_abs_path(project_path, l))
        .collect();

    Some(files)
}
