use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// H4: Check dispatcher pattern in pre-commit script
pub(super) fn check_dispatcher_pattern(
    pre_commit_path: &Path,
    is_modular: bool,
    pre_commit_content: &str,
    results: &mut Vec<CheckResult>,
) {
    if is_modular {
        let has_dispatcher = pre_commit_content.contains("pre-commit.d")
            && (pre_commit_content.contains("source ")
                || pre_commit_content.contains(". ")
                || pre_commit_content.contains("for ")
                || pre_commit_content.contains("run-parts"));
        if has_dispatcher {
            results.push(
                CheckResult::new(
                    "H4".to_owned(),
                    Severity::Info,
                    "Dispatcher pattern found".to_owned(),
                    "pre-commit sources scripts from pre-commit.d/".to_owned(),
                )
                .with_optional_location(Some(pre_commit_path.display().to_string()), None)
                .with_inventory(false)
                .as_inventory(),
            );
        } else {
            results.push(
                CheckResult::new(
                    "H4".to_owned(),
                    Severity::Error,
                    "Dispatcher pattern missing".to_owned(),
                    "pre-commit.d/ exists but pre-commit doesn't dispatch to it".to_owned(),
                )
                .with_optional_location(Some(pre_commit_path.display().to_string()), None)
                .with_inventory(false),
            );
        }
    } else {
        results.push(
            CheckResult::new(
                "H4".to_owned(),
                Severity::Info,
                "Monolithic script (no dispatcher needed)".to_owned(),
                "No pre-commit.d/, so no dispatcher check".to_owned(),
            )
            .with_optional_location(Some(pre_commit_path.display().to_string()), None)
            .with_inventory(false)
            .as_inventory(),
        );
    }
}

/// H6: emit script stats. Returns (`line_count`, `size`).
pub(super) fn emit_script_stats(
    fs: &dyn FileSystem,
    pre_commit_path: &Path,
    pre_commit_content: &str,
    results: &mut Vec<CheckResult>,
) -> (usize, u64) {
    let line_count = pre_commit_content.lines().count();
    let metadata = fs.metadata(pre_commit_path);
    let modified = metadata.as_ref().and_then(|m| m.modified().ok()).map(|t| {
        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    });
    let size = match metadata.as_ref() {
        Some(m) => m.len(),
        None => 0,
    };

    results.push(
        CheckResult::new(
            "H6".to_owned(),
            Severity::Info,
            "Pre-commit script stats".to_owned(),
            format!(
                "{line_count} lines, {size} bytes{}",
                modified.map_or(String::new(), |t| format!(", mtime unix {t}"))
            ),
        )
        .with_optional_location(Some(pre_commit_path.display().to_string()), None)
        .with_inventory(false)
        .as_inventory(),
    );

    (line_count, size)
}

pub(super) fn check_local_scripts(
    fs: &dyn FileSystem,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let overrides_d = path.join(".guardrail3/overrides/pre-commit.d");
    if overrides_d.is_dir() {
        inventory_scripts(fs, &overrides_d, "H11", "Local pre-commit scripts", results);
    } else {
        results.push(
            CheckResult::new(
                "H11".to_owned(),
                Severity::Info,
                "No .guardrail3/overrides/pre-commit.d/ directory".to_owned(),
                "No local hook overrides found".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    }
}

struct PatternCheck {
    pattern: &'static [&'static str],
    label: &'static str,
    severity_if_missing: Severity,
    requires_rust: bool,
    requires_ts: bool,
}

const HOOK_PATTERN_CHECKS: &[PatternCheck] = &[
    PatternCheck {
        pattern: &["gitleaks"],
        label: "gitleaks",
        severity_if_missing: Severity::Error,
        requires_rust: false,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo fmt", "rustfmt"],
        label: "cargo fmt / rustfmt",
        severity_if_missing: Severity::Error,
        requires_rust: true,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo clippy", "clippy"],
        label: "cargo clippy",
        severity_if_missing: Severity::Error,
        requires_rust: true,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo deny", "cargo-deny"],
        label: "cargo deny",
        severity_if_missing: Severity::Error,
        requires_rust: true,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo test"],
        label: "cargo test",
        severity_if_missing: Severity::Warn,
        requires_rust: true,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo machete", "cargo-machete"],
        label: "cargo machete",
        severity_if_missing: Severity::Warn,
        requires_rust: true,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["tsc", "--noEmit"],
        label: "tsc / --noEmit",
        severity_if_missing: Severity::Warn,
        requires_rust: false,
        requires_ts: true,
    },
    PatternCheck {
        pattern: &["eslint"],
        label: "eslint",
        severity_if_missing: Severity::Warn,
        requires_rust: false,
        requires_ts: true,
    },
    PatternCheck {
        pattern: &["jscpd"],
        label: "jscpd",
        severity_if_missing: Severity::Warn,
        requires_rust: false,
        requires_ts: false,
    },
    PatternCheck {
        pattern: &["cargo dupes", "cargo-dupes"],
        label: "cargo dupes",
        severity_if_missing: Severity::Info,
        requires_rust: true,
        requires_ts: false,
    },
];

pub(super) fn check_monolithic_patterns(
    content: &str,
    file_path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    for check in HOOK_PATTERN_CHECKS {
        if check.requires_rust && !has_rust {
            continue;
        }
        if check.requires_ts && !has_typescript {
            continue;
        }

        let found = check.pattern.iter().any(|p| content.contains(p));
        if found {
            results.push(
                CheckResult::new(
                    "H5".to_owned(),
                    Severity::Info,
                    format!("{} found in pre-commit", check.label),
                    "Pattern present in monolithic script".to_owned(),
                )
                .with_optional_location(Some(file_path.display().to_string()), None)
                .with_inventory(false),
            );
        } else {
            results.push(
                CheckResult::new(
                    "H5".to_owned(),
                    check.severity_if_missing,
                    format!("{} not found in pre-commit", check.label),
                    "Pattern missing from monolithic script".to_owned(),
                )
                .with_optional_location(Some(file_path.display().to_string()), None)
                .with_inventory(false),
            );
        }
    }
}

pub(super) fn check_modular_scripts(
    fs: &dyn FileSystem,
    pre_commit_d: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    // Read all script contents to search for patterns
    let mut all_content = String::new();
    for entry in fs.list_dir(pre_commit_d) {
        if let Some(content) = fs.read_file(&entry.path()) {
            all_content.push_str(&content);
            all_content.push('\n');
        }
    }

    // Reuse the same pattern checks against the combined script content
    check_monolithic_patterns(
        &all_content,
        pre_commit_d,
        has_rust,
        has_typescript,
        results,
    );
}

/// H-CSS-01: Check that pre-commit hook runs stylelint on CSS files.
pub(super) fn check_stylelint_hook(pre_commit_content: &str, results: &mut Vec<CheckResult>) {
    let has_stylelint = pre_commit_content.contains("stylelint");
    let has_css_detection = pre_commit_content.contains(".css");

    if has_stylelint && has_css_detection {
        results.push(
            CheckResult::new("H-CSS-01".to_owned(), Severity::Info, "Stylelint configured in pre-commit hook".to_owned(), "Pre-commit hook runs stylelint on staged CSS files for quality and accessibility checking.".to_owned()).with_optional_location(None, None).with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::new("H-CSS-01".to_owned(), Severity::Warn, "No stylelint in pre-commit hook".to_owned(), "Pre-commit hook does not run stylelint on CSS files. CSS quality and accessibility issues won't be caught before commit. Add a stylelint step to .githooks/pre-commit that runs on staged .css files.".to_owned()).with_optional_location(None, None).with_inventory(false));
    }
}

/// H-TOOL-01: cspell in pre-commit hook
pub(super) fn check_cspell_hook(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("cspell") {
        results.push(
            CheckResult::new(
                "H-TOOL-01".to_owned(),
                Severity::Info,
                "cspell configured in hook".to_owned(),
                "Pre-commit hook runs cspell.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult::new(
                "H-TOOL-01".to_owned(),
                Severity::Warn,
                "No cspell in hook".to_owned(),
                "Pre-commit hook does not run cspell. Add spell checking step.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false),
        );
    }
}

/// H-TOOL-02: merge conflict markers in hook
pub(super) fn check_conflict_marker_hook(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("conflict marker") || content.contains("<{7}") || content.contains("<<<") {
        results.push(
            CheckResult::new(
                "H-TOOL-02".to_owned(),
                Severity::Info,
                "Conflict marker check in hook".to_owned(),
                "Pre-commit hook checks for merge conflict markers.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::new("H-TOOL-02".to_owned(), Severity::Warn, "No conflict marker check in hook".to_owned(), "Pre-commit hook does not check for merge conflict markers. Add grep for <<<<<<< ======= >>>>>>>.".to_owned()).with_optional_location(None, None).with_inventory(false));
    }
}

/// H-TOOL-03: lockfile integrity in hook
pub(super) fn check_lockfile_hook(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("frozen-lockfile") || content.contains("lockfile") {
        results.push(
            CheckResult::new(
                "H-TOOL-03".to_owned(),
                Severity::Info,
                "Lockfile integrity check in hook".to_owned(),
                "Pre-commit hook verifies lockfile integrity.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::new("H-TOOL-03".to_owned(), Severity::Warn, "No lockfile check in hook".to_owned(), "Pre-commit hook does not check lockfile integrity. Add pnpm install --frozen-lockfile.".to_owned()).with_optional_location(None, None).with_inventory(false));
    }
}

/// H-TOOL-04: prettier format check in hook
pub(super) fn check_prettier_hook(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("prettier") && content.contains("--check") {
        results.push(
            CheckResult::new(
                "H-TOOL-04".to_owned(),
                Severity::Info,
                "Prettier format check in hook".to_owned(),
                "Pre-commit hook runs prettier --check.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult::new(
                "H-TOOL-04".to_owned(),
                Severity::Warn,
                "No prettier in hook".to_owned(),
                "Pre-commit hook does not run prettier --check. Add formatting verification."
                    .to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false),
        );
    }
}

/// H-TOOL-05: pnpm audit in hook
pub(super) fn check_audit_hook(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("pnpm audit") {
        results.push(
            CheckResult::new(
                "H-TOOL-05".to_owned(),
                Severity::Info,
                "Dependency audit in hook".to_owned(),
                "Pre-commit hook runs pnpm audit.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::new("H-TOOL-05".to_owned(), Severity::Warn, "No dependency audit in hook".to_owned(), "Pre-commit hook does not run pnpm audit. Add informational dependency vulnerability scan.".to_owned()).with_optional_location(None, None).with_inventory(false));
    }
}

/// H-SAFE-01: pre-commit hook should use `set -e` or `set -euo pipefail`
pub(super) fn check_set_e_safety(content: &str, results: &mut Vec<CheckResult>) {
    if content.contains("set -e") || content.contains("set -euo pipefail") {
        results.push(
            CheckResult::new(
                "H-SAFE-01".to_owned(),
                Severity::Info,
                "Pre-commit hook has shell error handling".to_owned(),
                "Hook script uses `set -e` or `set -euo pipefail`.".to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult::new(
                "H-SAFE-01".to_owned(),
                Severity::Warn,
                "Pre-commit hook missing shell error handling".to_owned(),
                "Pre-commit hook missing `set -e` or `set -euo pipefail` \u{2014} commands that \
                      fail may not abort the hook."
                    .to_owned(),
            )
            .with_optional_location(None, None)
            .with_inventory(false),
        );
    }
}

pub(super) fn inventory_scripts(
    fs: &dyn FileSystem,
    dir: &Path,
    id: &str,
    title_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    if !dir.exists() {
        results.push(
            CheckResult::new(
                id.to_owned(),
                Severity::Warn,
                format!("{title_prefix}: unreadable"),
                "Directory does not exist".to_owned(),
            )
            .with_optional_location(Some(dir.display().to_string()), None)
            .with_inventory(false),
        );
        return;
    }

    let entries = fs.list_dir(dir);
    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_owned());
        }
    }
    names.sort();

    if names.is_empty() {
        results.push(
            CheckResult::new(
                id.to_owned(),
                Severity::Info,
                format!("{title_prefix}: empty"),
                "No scripts found".to_owned(),
            )
            .with_optional_location(Some(dir.display().to_string()), None)
            .with_inventory(false)
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult::new(
                id.to_owned(),
                Severity::Info,
                format!("{title_prefix}: {} scripts", names.len()),
                names.join(", "),
            )
            .with_optional_location(Some(dir.display().to_string()), None)
            .with_inventory(false)
            .as_inventory(),
        );
    }
}
