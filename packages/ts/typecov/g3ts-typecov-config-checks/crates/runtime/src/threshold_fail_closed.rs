use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the typecov `threshold-fail-closed` config check.
pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-typecov/threshold-fail-closed",
            "Typecov script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `typecov` runs `type-coverage --at-least 100` fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::script_invokes_type_coverage(package, "typecov") {
        crate::common::info(
            "g3ts-typecov/threshold-fail-closed",
            "Typecov script is fail-closed",
            format!(
                "`{}` defines `typecov` with a fail-closed `type-coverage --at-least 100` invocation.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-typecov/threshold-fail-closed",
            "Typecov script is not fail-closed",
            format!(
                "`{}` must define `typecov` as a fail-closed `type-coverage --at-least 100` invocation without `||` fallback.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
