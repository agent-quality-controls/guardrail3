use std::path::Path;

use crate::process;

/// Returns true when staged files should trigger TypeScript workspace gates.
pub(crate) fn has_relevant_staged_files(path: &Path) -> bool {
    let Ok(output) = process::run_git(
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

/// Extensions that make a staged file relevant to TypeScript workspace gates.
const TS_RELEVANT_EXTENSIONS: &[&str] =
    &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "css"];

/// Exact filenames that make a staged file relevant to TypeScript workspace gates.
const TS_RELEVANT_FILENAMES: &[&str] = &[
    "package.json",
    "guardrail3-ts.toml",
    ".syncpackrc",
    "pnpm-lock.yaml",
    "pnpm-workspace.yaml",
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
    "prettier.config.js",
    "prettier.config.mjs",
    "cspell.json",
    ".cspell.json",
];

/// Returns true when a staged path is relevant to TypeScript validation.
fn is_ts_relevant_path(path: &str) -> bool {
    let path = path.trim();
    if path.is_empty() {
        return false;
    }
    let file_name = Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if TS_RELEVANT_FILENAMES.contains(&file_name) {
        return true;
    }
    Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| TS_RELEVANT_EXTENSIONS.contains(&extension))
}
