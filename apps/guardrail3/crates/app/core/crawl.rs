//! Filesystem crawler — single walk, collect every file guardrail3 cares about.
//!
//! Uses the `ignore` crate (ripgrep's walker) which respects .gitignore natively.
//! No hardcoded path assumptions. Works with any project layout.

use std::path::{Path, PathBuf};

/// Raw crawl result — every interesting file found, grouped by type.
#[derive(Debug, Default)]
pub struct CrawlResult {
    // ── Rust structure ──
    pub cargo_tomls: Vec<PathBuf>,
    pub cargo_locks: Vec<PathBuf>,

    // ── Rust guardrail configs ──
    pub clippy_tomls: Vec<PathBuf>,
    pub deny_tomls: Vec<PathBuf>,
    pub rustfmt_tomls: Vec<PathBuf>,
    pub rust_toolchains: Vec<PathBuf>,

    // ── TypeScript structure ──
    pub package_jsons: Vec<PathBuf>,
    pub pnpm_workspaces: Vec<PathBuf>,

    // ── TypeScript guardrail configs ──
    /// `tsconfig.json` files — participate in walk-up resolution.
    pub tsconfigs: Vec<PathBuf>,
    /// `tsconfig.base.json` files — only reachable via explicit `extends`, not walk-up.
    pub tsconfig_bases: Vec<PathBuf>,
    pub eslint_configs: Vec<PathBuf>,
    pub stylelint_configs: Vec<PathBuf>,
    pub cspell_configs: Vec<PathBuf>,
    pub jscpd_configs: Vec<PathBuf>,
    pub npmrcs: Vec<PathBuf>,
    pub prettier_configs: Vec<PathBuf>,

    // ── Detection signals (not guardrails, but used for auto-detection) ──
    pub velite_configs: Vec<PathBuf>,
    pub next_configs: Vec<PathBuf>,

    // ── Release ──
    pub release_plz_tomls: Vec<PathBuf>,
    pub cliff_tomls: Vec<PathBuf>,

    // ── CI/CD ──
    pub github_workflows: Vec<PathBuf>,

    // ── Hooks ──
    pub pre_commit_hooks: Vec<PathBuf>,

    // ── Repo-level files (validation checks) ──
    pub license_files: Vec<PathBuf>,
    pub claude_mds: Vec<PathBuf>,
    pub cargo_mutants_tomls: Vec<PathBuf>,

    // ── guardrail3 ──
    pub guardrail3_tomls: Vec<PathBuf>,
    pub guardrail3_overrides: Vec<PathBuf>,

    // ── Source file directories (for coverage maps) ──
    // Each set contains directories that have at least one file of that type.
    // Used to determine which directories need coverage by which tool.
    pub dirs_with_rs: std::collections::BTreeSet<PathBuf>,
    pub dirs_with_ts: std::collections::BTreeSet<PathBuf>,
    pub dirs_with_css: std::collections::BTreeSet<PathBuf>,
}

/// Walk the project tree from `root`, collecting every file guardrail3 cares about.
///
/// Respects .gitignore — `node_modules`/, target/, dist/ etc. are skipped automatically
/// if they're in .gitignore (which they should be in any sane project).
#[allow(clippy::too_many_lines)] // reason: single match on filename — splitting would obscure the classification
pub fn crawl(root: &Path) -> CrawlResult {
    let mut result = CrawlResult::default();

    let walker = ignore::WalkBuilder::new(root)
        .hidden(false) // don't skip hidden dirs — need .guardrail3/, .githooks/, .stylelintrc.*
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker.flatten() {
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }
        let path = entry.into_path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()).map(str::to_owned) else {
            continue;
        };

        // Config files inside test directories are test data, not project configs.
        // Source files still get tracked for dirs_with_rs/dirs_with_ts coverage.
        if is_test_directory(&path) {
            track_source_dir(&path, &mut result);
            continue;
        }

        match name.as_str() {
            // ── Rust structure ──
            "Cargo.toml" => result.cargo_tomls.push(path),
            "Cargo.lock" => result.cargo_locks.push(path),

            // ── Rust guardrail configs ──
            "clippy.toml" | ".clippy.toml" => result.clippy_tomls.push(path),
            "deny.toml" | ".deny.toml" => result.deny_tomls.push(path),
            "rustfmt.toml" | ".rustfmt.toml" => result.rustfmt_tomls.push(path),
            "rust-toolchain.toml" => result.rust_toolchains.push(path),

            // ── TypeScript structure ──
            "package.json" => result.package_jsons.push(path),
            "pnpm-workspace.yaml" => result.pnpm_workspaces.push(path),

            // ── TypeScript guardrail configs ──
            "tsconfig.json" => result.tsconfigs.push(path),
            "tsconfig.base.json" => result.tsconfig_bases.push(path),
            ".npmrc" => result.npmrcs.push(path),
            ".jscpd.json" => result.jscpd_configs.push(path),

            // ── Release ──
            "release-plz.toml" | ".release-plz.toml" => result.release_plz_tomls.push(path),
            "cliff.toml" => result.cliff_tomls.push(path),

            // ── Repo-level files ──
            "CLAUDE.md" => result.claude_mds.push(path),
            "LICENSE" | "LICENSE-MIT" | "LICENSE-APACHE" | "LICENSE.md" => {
                result.license_files.push(path);
            }
            "mutants.toml" => {
                // Only if inside .cargo/
                if path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    == Some(".cargo")
                {
                    result.cargo_mutants_tomls.push(path);
                }
            }

            // ── guardrail3 ──
            "guardrail3.toml" => result.guardrail3_tomls.push(path),

            // ── Multi-name configs — check prefix/suffix ──
            _ => {
                // Track source file directories for coverage maps
                track_source_dir(&path, &mut result);
                classify_by_pattern(&name, path, &mut result);
            }
        }
    }

    // Sort all vectors for deterministic output
    result.cargo_tomls.sort();
    result.cargo_locks.sort();
    result.clippy_tomls.sort();
    result.deny_tomls.sort();
    result.rustfmt_tomls.sort();
    result.package_jsons.sort();
    result.tsconfigs.sort();
    result.tsconfig_bases.sort();
    result.eslint_configs.sort();
    result.stylelint_configs.sort();
    result.cspell_configs.sort();
    result.jscpd_configs.sort();
    result.npmrcs.sort();
    result.prettier_configs.sort();
    result.velite_configs.sort();
    result.next_configs.sort();
    result.github_workflows.sort();
    result.rust_toolchains.sort();
    result.pnpm_workspaces.sort();
    result.release_plz_tomls.sort();
    result.cliff_tomls.sort();
    result.pre_commit_hooks.sort();
    result.license_files.sort();
    result.claude_mds.sort();
    result.cargo_mutants_tomls.sort();
    result.guardrail3_tomls.sort();
    result.guardrail3_overrides.sort();

    result
}

/// Classify files that have variable names (eslint.config.*, .stylelintrc.*, cspell.config.*, etc.)
fn classify_by_pattern(name: &str, path: PathBuf, result: &mut CrawlResult) {
    // ESLint: eslint.config.{js,mjs,cjs,ts,mts,cts}
    if name.starts_with("eslint.config.") {
        result.eslint_configs.push(path);
        return;
    }

    // Stylelint: .stylelintrc.* or stylelint.config.*
    if name.starts_with(".stylelintrc") || name.starts_with("stylelint.config.") {
        result.stylelint_configs.push(path);
        return;
    }

    // cspell: cspell.json, .cspell.json, cspell.config.*, .cspell.config.*
    if name.starts_with("cspell.config.") || name.starts_with(".cspell") {
        result.cspell_configs.push(path);
        return;
    }

    // Prettier: .prettierrc.* or prettier.config.*
    if name.starts_with(".prettierrc") || name.starts_with("prettier.config.") {
        result.prettier_configs.push(path);
        return;
    }

    // Velite: velite.config.{ts,mjs,js}
    if name.starts_with("velite.config.") {
        result.velite_configs.push(path);
        return;
    }

    // Next.js: next.config.{ts,mjs,js}
    if name.starts_with("next.config.") {
        result.next_configs.push(path);
        return;
    }

    // GitHub workflows: .github/workflows/*.yml
    let ext_matches = std::path::Path::new(name)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml"));
    if ext_matches
        && path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            == Some("workflows")
    {
        result.github_workflows.push(path);
        return;
    }

    // Pre-commit hook
    if name == "pre-commit" {
        if let Some(parent) = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        {
            if parent == ".githooks" || parent == "hooks" {
                result.pre_commit_hooks.push(path);
                return;
            }
        }
    }

    // guardrail3 override files
    if let Some(parent) = path.parent() {
        if parent.ends_with(".guardrail3/overrides")
            || parent
                .parent()
                .is_some_and(|pp| pp.ends_with(".guardrail3/overrides"))
        {
            result.guardrail3_overrides.push(path);
        }
    }
}

/// Check if a file is inside a test/fixture directory.
/// Config files in these dirs are test data, not project configs.
fn is_test_directory(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_str().unwrap_or("");
        matches!(
            s,
            "tests"
                | "test"
                | "__tests__"
                | "__mocks__"
                | "fixtures"
                | "test-data"
                | "golden-tests"
        )
    })
}

/// Track which directories contain source files, by file extension.
fn track_source_dir(path: &Path, result: &mut CrawlResult) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let Some(dir) = path.parent() else {
        return;
    };
    match ext {
        "rs" => {
            let _ = result.dirs_with_rs.insert(dir.to_path_buf());
        }
        "ts" | "tsx" | "mts" | "js" | "jsx" | "mjs" => {
            let _ = result.dirs_with_ts.insert(dir.to_path_buf());
        }
        "css" => {
            let _ = result.dirs_with_css.insert(dir.to_path_buf());
        }
        _ => {}
    }
}
