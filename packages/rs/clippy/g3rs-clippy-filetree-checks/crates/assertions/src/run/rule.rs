pub fn assert_clean_root_coverage(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::coverage_exists::rule::assert_findings(
        results,
        &[crate::coverage_exists::rule::info(
            "workspace root covered by clippy config",
            "Workspace root is covered by `.clippy.toml`.",
            ".clippy.toml",
            true,
        )],
    );
    crate::same_root_conflict::rule::assert_no_findings(results);
}

pub fn assert_plain_root_coverage(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::coverage_exists::rule::assert_findings(
        results,
        &[crate::coverage_exists::rule::info(
            "workspace root covered by clippy config",
            "Workspace root is covered by `clippy.toml`.",
            "clippy.toml",
            true,
        )],
    );
    crate::same_root_conflict::rule::assert_no_findings(results);
}

pub fn assert_missing_root(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::coverage_exists::rule::assert_findings(
        results,
        &[crate::coverage_exists::rule::error(
            "workspace root uncovered by clippy config",
            "Add `clippy.toml` or `.clippy.toml` at the workspace root so clippy policy is not left to defaults.",
            "clippy.toml",
            false,
        )],
    );
    crate::same_root_conflict::rule::assert_no_findings(results);
}

pub fn assert_same_root_conflict(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::coverage_exists::rule::assert_findings(
        results,
        &[crate::coverage_exists::rule::info(
            "workspace root covered by clippy config",
            "Workspace root is covered by `.clippy.toml`.",
            ".clippy.toml",
            true,
        )],
    );
    crate::same_root_conflict::rule::assert_findings(
        results,
        &[crate::same_root_conflict::rule::error(
            "same-root clippy config conflict",
            "`clippy.toml` conflicts with `.clippy.toml` at the same policy root. Keep only the highest-precedence clippy config file.",
            "clippy.toml",
            false,
        )],
    );
}
