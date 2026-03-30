use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

pub fn check_duplication_tools(
    content: &str,
    file_path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    let has_cargo_dupes = content.contains("cargo dupes") || content.contains("cargo-dupes");
    let has_jscpd = content.contains("jscpd");
    let file = file_path.display().to_string();

    emit_duplication_warnings(
        results,
        &file,
        has_rust,
        has_typescript,
        has_cargo_dupes,
        has_jscpd,
    );
    emit_duplication_inventory(
        results,
        &file,
        has_rust,
        has_typescript,
        has_cargo_dupes,
        has_jscpd,
    );
}

pub fn check_required_tools(tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    let tools = [
        ("gitleaks", Severity::Error),
        ("cargo-deny", Severity::Error),
        ("cargo-machete", Severity::Error),
    ];

    for (tool, severity) in &tools {
        if tc.is_installed(tool) {
            results.push(CheckResult::new(
                "H8".to_owned(),
                Severity::Info,
                format!("{tool} installed"),
                "Found on PATH".to_owned(),
            ));
        } else {
            results.push(CheckResult::new(
                "H8".to_owned(),
                *severity,
                format!("{tool} not installed"),
                format!("{tool} not found on PATH"),
            ));
        }
    }
}

fn emit_duplication_warnings(
    results: &mut Vec<CheckResult>,
    file: &str,
    has_rust: bool,
    has_typescript: bool,
    has_cargo_dupes: bool,
    has_jscpd: bool,
) {
    if has_rust && !has_cargo_dupes {
        results.push(h12_warn(
            "Missing cargo-dupes for Rust",
            "Rust project should use cargo-dupes for copy-paste detection",
            file,
        ));
    }

    if has_typescript && !has_jscpd {
        results.push(h12_warn(
            "Missing jscpd for TypeScript",
            "TypeScript project should use jscpd for copy-paste detection",
            file,
        ));
    }

    if has_rust && has_jscpd && !has_cargo_dupes {
        results.push(h12_warn(
            "Using jscpd for Rust",
            "Using jscpd for Rust -- consider cargo-dupes (AST-aware, no Node.js dependency)",
            file,
        ));
    }

    if has_rust && !has_typescript && has_jscpd {
        results.push(h12_warn(
            "Rust-only project running jscpd",
            "Rust-only project running jscpd requires Node.js -- use cargo-dupes instead",
            file,
        ));
    }
}

fn emit_duplication_inventory(
    results: &mut Vec<CheckResult>,
    file: &str,
    has_rust: bool,
    has_typescript: bool,
    has_cargo_dupes: bool,
    has_jscpd: bool,
) {
    if has_rust && has_cargo_dupes {
        results.push(
            h12_info(
                "cargo-dupes configured for Rust",
                "Rust copy-paste detection using cargo-dupes",
                file,
            )
            .as_inventory(),
        );
    }

    if has_typescript && has_jscpd {
        results.push(
            h12_info(
                "jscpd configured for TypeScript",
                "TypeScript copy-paste detection using jscpd",
                file,
            )
            .as_inventory(),
        );
    }
}

fn h12_warn(title: &str, message: &str, file: &str) -> CheckResult {
    CheckResult::new(
        "H12".to_owned(),
        Severity::Warn,
        title.to_owned(),
        message.to_owned(),
    )
    .with_file(file.to_owned())
}

fn h12_info(title: &str, message: &str, file: &str) -> CheckResult {
    CheckResult::new(
        "H12".to_owned(),
        Severity::Info,
        title.to_owned(),
        message.to_owned(),
    )
    .with_file(file.to_owned())
}
