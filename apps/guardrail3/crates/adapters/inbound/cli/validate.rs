use std::collections::BTreeSet;
use std::path::Path;

pub trait ScopedValidateArgs {
    fn staged(&self) -> bool;
    fn dirty(&self) -> bool;
    fn commits(&self) -> Option<usize>;
    fn files(&self) -> &[String];
}

impl ScopedValidateArgs for crate::cli::RsValidateArgs {
    fn staged(&self) -> bool {
        self.staged
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn commits(&self) -> Option<usize> {
        self.commits
    }

    fn files(&self) -> &[String] {
        &self.files
    }
}

impl ScopedValidateArgs for crate::cli::TsValidateArgs {
    fn staged(&self) -> bool {
        self.staged
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn commits(&self) -> Option<usize> {
        self.commits
    }

    fn files(&self) -> &[String] {
        &self.files
    }
}

/// Convert a repo-relative path to an absolute path string.
fn to_abs_path(project_path: &Path, relative: &str) -> String {
    project_path.join(relative).display().to_string()
}

pub fn resolve_scoped_files_pub<T: ScopedValidateArgs>(
    args: &T,
    project_path: &Path,
) -> Option<Vec<String>> {
    resolve_scoped_files(args, project_path)
}

pub fn normalize_scoped_files(
    project_path: &Path,
    scoped_files: Option<&[String]>,
) -> Option<BTreeSet<String>> {
    scoped_files.map(|files| {
        files
            .iter()
            .filter_map(|path| normalize_one_scoped_file(project_path, path))
            .collect()
    })
}

fn normalize_one_scoped_file(project_path: &Path, path: &str) -> Option<String> {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        candidate
            .strip_prefix(project_path)
            .ok()
            .map(|rel| rel.to_string_lossy().trim_start_matches("./").to_owned())
    } else {
        Some(path.trim_start_matches("./").to_owned())
    }
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands for scoped file detection
fn resolve_scoped_files<T: ScopedValidateArgs>(
    args: &T,
    project_path: &Path,
) -> Option<Vec<String>> {
    if !args.files().is_empty() {
        return Some(args.files().to_vec());
    }

    if args.staged() {
        return git_staged_files(project_path);
    }

    if args.dirty() {
        return git_dirty_files(project_path);
    }

    if let Some(n) = args.commits() {
        return git_commit_files(project_path, n);
    }

    None
}

#[allow(clippy::disallowed_methods)] // reason: CLI tool runs git commands
fn git_staged_files(project_path: &Path) -> Option<Vec<String>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| to_abs_path(project_path, line))
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

    let untracked = std::process::Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !staged.status.success() || !unstaged.status.success() || !untracked.status.success() {
        return None;
    }

    let mut files = Vec::new();
    for stdout in [&staged.stdout, &unstaged.stdout, &untracked.stdout] {
        for line in String::from_utf8_lossy(stdout).lines() {
            let full = project_path.join(line).display().to_string();
            if !files.contains(&full) {
                files.push(full);
            }
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
        .filter(|line| !line.is_empty())
        .map(|line| to_abs_path(project_path, line))
        .collect();

    Some(files)
}
