use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::ToolChecker;

#[allow(clippy::too_many_lines)] // reason: check function validates multiple tools sequentially, splitting would fragment the logic
pub fn check_duplication_tools(
    content: &str,
    file_path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    let has_cargo_dupes = content.contains("cargo dupes") || content.contains("cargo-dupes");
    let has_jscpd = content.contains("jscpd");

    if has_rust && !has_cargo_dupes {
        results.push(CheckResult {
            id: "H12".to_owned(),
            severity: Severity::Warn,
            title: "Missing cargo-dupes for Rust".to_owned(),
            message: "Rust project should use cargo-dupes for copy-paste detection".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    if has_typescript && !has_jscpd {
        results.push(CheckResult {
            id: "H12".to_owned(),
            severity: Severity::Warn,
            title: "Missing jscpd for TypeScript".to_owned(),
            message: "TypeScript project should use jscpd for copy-paste detection".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    if has_rust && has_jscpd && !has_cargo_dupes {
        results.push(CheckResult {
            id: "H12".to_owned(),
            severity: Severity::Warn,
            title: "Using jscpd for Rust".to_owned(),
            message:
                "Using jscpd for Rust -- consider cargo-dupes (AST-aware, no Node.js dependency)"
                    .to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    if has_rust && !has_typescript && content.contains("pnpm exec jscpd") {
        results.push(CheckResult {
            id: "H12".to_owned(),
            severity: Severity::Warn,
            title: "Rust-only project running jscpd".to_owned(),
            message: "Rust-only project running jscpd requires Node.js -- use cargo-dupes instead"
                .to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    if has_rust && has_cargo_dupes {
        results.push(
            CheckResult {
                id: "H12".to_owned(),
                severity: Severity::Info,
                title: "cargo-dupes configured for Rust".to_owned(),
                message: "Rust copy-paste detection using cargo-dupes".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }

    if has_typescript && has_jscpd {
        results.push(
            CheckResult {
                id: "H12".to_owned(),
                severity: Severity::Info,
                title: "jscpd configured for TypeScript".to_owned(),
                message: "TypeScript copy-paste detection using jscpd".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

pub fn check_required_tools(tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    let tools = [
        ("gitleaks", Severity::Error),
        ("cargo-deny", Severity::Error),
        ("cargo-machete", Severity::Error),
    ];

    for (tool, severity) in &tools {
        if tc.is_installed(tool) {
            results.push(CheckResult {
                id: "H8".to_owned(),
                severity: Severity::Info,
                title: format!("{tool} installed"),
                message: "Found on PATH".to_owned(),
                file: None,
                line: None,
                inventory: false,
            });
        } else {
            results.push(CheckResult {
                id: "H8".to_owned(),
                severity: *severity,
                title: format!("{tool} not installed"),
                message: format!("{tool} not found on PATH"),
                file: None,
                line: None,
                inventory: false,
            });
        }
    }
}
