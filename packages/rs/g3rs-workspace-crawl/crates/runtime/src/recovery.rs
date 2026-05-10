//! Recovery list and banned-tree predicates used during crawl phase 2.

use std::path::Path;

/// Directory names that are never walked during recovery.
///
/// These directories are excluded from the recovery phase to avoid
/// descending into build artifacts, dependency trees, and git internals.
const BANNED_DIR_NAMES: &[&str] = &[".git", "target", "node_modules"];

/// Root-relative subtrees that are never walked during recovery.
const BANNED_ROOTS: &[&str] = &[".claude/worktrees"];

/// Config file names recovered from ignored space (exact match).
const RECOVER_EXACT: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    ".gitignore",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "rustfmt.toml",
    ".rustfmt.toml",
    "rust-toolchain.toml",
    "rust-toolchain",
    "package.json",
    "pnpm-workspace.yaml",
    "tsconfig.json",
    "tsconfig.base.json",
    ".npmrc",
    ".jscpd.json",
    "cspell.json",
    ".cspell.json",
    "guardrail3-rs.toml",
    "release-plz.toml",
    ".release-plz.toml",
    "cliff.toml",
    "lefthook.yml",
    "lefthook.yaml",
    ".lefthook.yml",
    ".lefthook.yaml",
    "stryker.config.json",
];

/// Config file prefixes recovered from ignored space.
const RECOVER_PREFIX: &[&str] = &[
    "eslint.config.",
    ".stylelintrc",
    "stylelint.config.",
    "cspell.config.",
    ".cspell.config.",
    "prettier.config.",
    ".prettierrc",
    "velite.config.",
    "contentlayer.config.",
    "next.config.",
    "stryker.config.",
    "vitest.config.",
    "jest.config.",
    "playwright.config.",
];

/// Directory names recovered as ignored sentinels because their mere presence
/// is a guardrail-relevant fact.
const RECOVER_DIR_NAMES: &[&str] = &[".next", ".velite", ".contentlayer"];

/// Whether this file should be recovered from ignored space.
pub(crate) fn should_recover(name: &str, rel_path: &str) -> bool {
    if RECOVER_EXACT.contains(&name) {
        return true;
    }
    if RECOVER_PREFIX.iter().any(|p| name.starts_with(p)) {
        return true;
    }
    if name.starts_with("tsconfig")
        && Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        return true;
    }
    if rel_path.ends_with(".cargo/config.toml") || rel_path.ends_with(".cargo/config") {
        return true;
    }
    if name == "mutants.toml" && rel_path.contains(".cargo/") {
        return true;
    }
    if name == "nextest.toml" && rel_path.contains(".config/") {
        return true;
    }
    if rel_path.contains(".config/")
        && (name.starts_with("cspell.") || name.starts_with(".cspell."))
    {
        return true;
    }
    if rel_path.contains(".github/workflows/") {
        let ext = Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml") {
            return true;
        }
    }
    if name == "pre-commit"
        && (rel_path == ".githooks/pre-commit"
            || rel_path == "hooks/pre-commit"
            || rel_path == ".husky/pre-commit")
    {
        return true;
    }
    if rel_path.starts_with(".githooks/pre-commit.d/")
        || rel_path.starts_with(".guardrail3/overrides/pre-commit.d/")
    {
        return true;
    }
    false
}

/// Whether this directory should be recovered from ignored space.
pub(crate) fn should_recover_dir(name: &str) -> bool {
    RECOVER_DIR_NAMES.contains(&name)
}

/// Whether a relative path falls under a banned subtree.
pub(crate) fn is_banned(rel_path: &str) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    if BANNED_ROOTS
        .iter()
        .any(|root| rel_path == *root || rel_path.starts_with(&format!("{root}/")))
    {
        return true;
    }
    rel_path
        .split('/')
        .any(|component| BANNED_DIR_NAMES.contains(&component))
}
