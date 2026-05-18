use g3ts_fmt_types::G3TsFmtContractInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the corresponding fmt config check.
pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-fmt/format-check-script",
            "Format check script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `format:check` runs Prettier fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::script_invokes_prettier(package, "format:check", "--check") {
        crate::common::info(
            "g3ts-fmt/format-check-script",
            "Format check script is configured",
            format!(
                "`{}` defines `format:check` with a fail-closed `prettier --check` invocation.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-fmt/format-check-script",
            "Format check script is missing or not fail-closed",
            format!(
                "`{}` must define `format:check` as a fail-closed `prettier --check ...` invocation without `||` fallback.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
