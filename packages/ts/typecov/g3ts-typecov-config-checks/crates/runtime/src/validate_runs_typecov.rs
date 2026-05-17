use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the typecov `validate-runs-typecov` config check.
pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-typecov/validate-runs-typecov",
            "Validate script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `validate` runs typecov fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::validate_runs_typecov(package) {
        crate::common::info(
            "g3ts-typecov/validate-runs-typecov",
            "Validate script runs typecov",
            format!(
                "`{}` defines a fail-closed `validate` script that reaches `typecov` or direct `type-coverage --at-least 100`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-typecov/validate-runs-typecov",
            "Validate script does not run typecov",
            format!(
                "`{}` must define a fail-closed `validate` script that invokes `typecov` through a package-manager run command or directly invokes `type-coverage --at-least 100`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
