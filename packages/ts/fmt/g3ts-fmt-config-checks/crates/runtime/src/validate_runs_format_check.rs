use g3ts_fmt_types::G3TsFmtContractInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the corresponding fmt config check.
pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-fmt/validate-runs-format-check",
            "Validate script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `validate` runs formatting fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::validate_runs_format_check(package) {
        crate::common::info(
            "g3ts-fmt/validate-runs-format-check",
            "Validate script runs format check",
            format!(
                "`{}` defines a fail-closed `validate` script that reaches `format:check` or direct `prettier --check`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-fmt/validate-runs-format-check",
            "Validate script does not run format check",
            format!(
                "`{}` must define a fail-closed `validate` script that invokes `format:check` through a package-manager run command or directly invokes `prettier --check`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
