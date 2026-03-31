use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

pub fn check(tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R45: cargo-deny installed
    check_tool_installed(tc, "cargo-deny", "R45", Severity::Error, &mut results);

    // R46: cargo-machete installed
    check_tool_installed(tc, "cargo-machete", "R46", Severity::Error, &mut results);

    // R47: cargo-dupes installed (recommended, not required)
    check_tool_installed(tc, "cargo-dupes", "R47", Severity::Warn, &mut results);

    // R48: gitleaks installed
    check_tool_installed(tc, "gitleaks", "R48", Severity::Error, &mut results);

    // R50 REMOVED — banned crate detection is cargo-deny's job.
    // guardrail3 verifies: deny.toml configured (R8-R20), cargo-deny installed (R45),
    // pre-commit hook runs cargo-deny (H-checks). cargo-deny catches actual violations.

    results
}

fn check_tool_installed(
    tc: &dyn ToolChecker,
    tool: &str,
    check_id: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    if tc.is_installed(tool) {
        results.push(CheckResult::from_parts(
    check_id.to_owned(),
    Severity::Info,
    format!("{tool} installed"),
    format!("`{tool}` found on PATH. Required tool for guardrail enforcement is available. No action needed."),
    None,
    None,
    false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: missing_severity,
            title: format!("{tool} not installed"),
            message: format!("`{tool}` not found on PATH. This tool is required for guardrail enforcement in pre-commit hooks. Install with: `cargo install {tool}`"),
            file: None,
            line: None,
            inventory: false,
        ));
    }
