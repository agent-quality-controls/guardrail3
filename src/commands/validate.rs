use std::path::Path;
use std::process::Command;

use crate::cli::ValidateArgs;
use crate::discover;
use crate::hooks;
use crate::report;
use crate::report::types::Report;
use crate::rs;
use crate::ts;

pub fn run(args: &ValidateArgs) {
    let path = Path::new(&args.path);
    let abs_path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: cannot resolve path '{}': {e}", args.path);
            std::process::exit(1);
        }
    };

    let project = discover::detect_project(&abs_path);

    let scoped_files = resolve_scoped_files(args, &abs_path);
    let scoped_ref = scoped_files.as_deref();

    let mut combined_report = Report::new(
        abs_path.display().to_string(),
        {
            let mut stacks = Vec::new();
            if project.has_rust {
                stacks.push("Rust".to_string());
            }
            if project.has_typescript {
                stacks.push("TypeScript".to_string());
            }
            if stacks.is_empty() {
                stacks.push("Unknown".to_string());
            }
            stacks
        },
    );

    if project.has_rust {
        let rust_report = rs::validate::run(&abs_path, &project, scoped_ref);
        for section in rust_report.sections {
            combined_report.add_section(section);
        }
    }

    if project.has_typescript {
        let ts_report = ts::validate::run(&abs_path, scoped_ref);
        for section in ts_report.sections {
            combined_report.add_section(section);
        }
    }

    // Hook and deployment checks
    let hooks_report = hooks::validate::run(
        &abs_path,
        project.has_rust,
        project.has_typescript,
    );
    for section in hooks_report.sections {
        combined_report.add_section(section);
    }

    match args.format.as_str() {
        "json" => report::json::print_report(&combined_report),
        "md" | "markdown" => report::markdown::print_report(&combined_report),
        _ => report::text::print_report(&combined_report),
    };

    // Exit with error code if errors found
    if combined_report.error_count() > 0 {
        std::process::exit(1);
    }
}

pub fn resolve_scoped_files_pub(args: &ValidateArgs, project_path: &Path) -> Option<Vec<String>> {
    resolve_scoped_files(args, project_path)
}

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

fn git_staged_files(project_path: &Path) -> Option<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| project_path.join(l).display().to_string())
        .collect();

    Some(files)
}

fn git_dirty_files(project_path: &Path) -> Option<Vec<String>> {
    let staged = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(project_path)
        .output()
        .ok()?;

    let unstaged = Command::new("git")
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

fn git_commit_files(project_path: &Path, n: usize) -> Option<Vec<String>> {
    let output = Command::new("git")
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
        .map(|l| project_path.join(l).display().to_string())
        .collect();

    Some(files)
}
