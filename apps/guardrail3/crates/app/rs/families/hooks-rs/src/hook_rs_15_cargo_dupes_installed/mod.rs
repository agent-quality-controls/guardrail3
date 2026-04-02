use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

const ID: &str = "HOOK-RS-15";

pub fn check(
    rel_path: &str,
    cargo_dupes_required: bool,
    cargo_dupes_path_qualified: bool,
    tc: &dyn ToolChecker,
    results: &mut Vec<CheckResult>,
) {
    if !cargo_dupes_required {
        return;
    }

    if cargo_dupes_path_qualified || tc.is_installed("cargo-dupes") {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "cargo-dupes installed".to_owned(),
                "cargo-dupes is available for Rust duplication checks.".to_owned(),
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
            "cargo-dupes missing".to_owned(),
            "Hook requires cargo-dupes, but it is not available on PATH.".to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_case(
    cargo_dupes_required: bool,
    cargo_dupes_path_qualified: bool,
    installed: &[&'static str],
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        cargo_dupes_required,
        cargo_dupes_path_qualified,
        &test_support::StubToolChecker::new(installed),
        &mut results,
    );
    results
}

#[cfg(test)]

mod tests;
