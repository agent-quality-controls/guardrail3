use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

const ID: &str = "HOOK-RS-06";

pub fn check(rel_path: &str, tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    for tool in ["gitleaks", "cargo-deny", "cargo-machete"] {
        if tc.is_installed(tool) {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    format!("{tool} installed"),
                    format!("{tool} is available on PATH for Rust hook execution."),
                    Some(rel_path.to_owned()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("{tool} missing"),
                format!("{tool} is required by the Rust hook but is not on PATH."),
                Some(rel_path.to_owned()),
                None,
                false,
            ));
        }
    }
}

#[cfg(test)]
pub(crate) fn run_case(installed: &[&'static str]) -> Vec<CheckResult> {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        &test_support::StubToolChecker::new(installed),
        &mut results,
    );
    results
}

#[cfg(test)]

mod tests;
