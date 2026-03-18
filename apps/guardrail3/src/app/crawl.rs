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
    pub rust_toolchain: Option<PathBuf>,

    // ── TypeScript structure ──
    pub package_jsons: Vec<PathBuf>,
    pub pnpm_workspace: Option<PathBuf>,

    // ── TypeScript guardrail configs ──
    pub tsconfigs: Vec<PathBuf>,
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
    pub release_plz: Option<PathBuf>,
    pub cliff_toml: Option<PathBuf>,

    // ── CI/CD ──
    pub github_workflows: Vec<PathBuf>,

    // ── Hooks ──
    pub pre_commit_hook: Option<PathBuf>,

    // ── Repo-level files (validation checks) ──
    pub license_file: Option<PathBuf>,
    pub claude_md: Option<PathBuf>,
    pub cargo_mutants_toml: Option<PathBuf>,

    // ── guardrail3 ──
    pub guardrail3_toml: Option<PathBuf>,
    pub guardrail3_overrides: Vec<PathBuf>,
}

/// Walk the project tree from `root`, collecting every file guardrail3 cares about.
///
/// Respects .gitignore — `node_modules`/, target/, dist/ etc. are skipped automatically
/// if they're in .gitignore (which they should be in any sane project).
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

        match name.as_str() {
            // ── Rust structure ──
            "Cargo.toml" => result.cargo_tomls.push(path),
            "Cargo.lock" => result.cargo_locks.push(path),

            // ── Rust guardrail configs ──
            "clippy.toml" | ".clippy.toml" => result.clippy_tomls.push(path),
            "deny.toml" | ".deny.toml" => result.deny_tomls.push(path),
            "rustfmt.toml" | ".rustfmt.toml" => result.rustfmt_tomls.push(path),
            "rust-toolchain.toml" => result.rust_toolchain = Some(path),

            // ── TypeScript structure ──
            "package.json" => result.package_jsons.push(path),
            "pnpm-workspace.yaml" => result.pnpm_workspace = Some(path),

            // ── TypeScript guardrail configs ──
            "tsconfig.json" | "tsconfig.base.json" => result.tsconfigs.push(path),
            ".npmrc" => result.npmrcs.push(path),
            ".jscpd.json" => result.jscpd_configs.push(path),

            // ── Release ──
            "release-plz.toml" | ".release-plz.toml" => result.release_plz = Some(path),
            "cliff.toml" => result.cliff_toml = Some(path),

            // ── Repo-level files ──
            "CLAUDE.md" => result.claude_md = Some(path),
            "LICENSE" | "LICENSE-MIT" | "LICENSE-APACHE" | "LICENSE.md" => {
                result.license_file = Some(path);
            }
            "mutants.toml" => {
                // Only if inside .cargo/
                if path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    == Some(".cargo")
                {
                    result.cargo_mutants_toml = Some(path);
                }
            }

            // ── guardrail3 ──
            "guardrail3.toml" => result.guardrail3_toml = Some(path),

            // ── Multi-name configs — check prefix/suffix ──
            _ => classify_by_pattern(&name, path, &mut result),
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
    result.eslint_configs.sort();
    result.stylelint_configs.sort();
    result.cspell_configs.sort();
    result.jscpd_configs.sort();
    result.npmrcs.sort();
    result.prettier_configs.sort();
    result.velite_configs.sort();
    result.next_configs.sort();
    result.github_workflows.sort();
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
                result.pre_commit_hook = Some(path);
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
