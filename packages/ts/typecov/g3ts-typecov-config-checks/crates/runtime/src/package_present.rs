use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;

const TYPE_COVERAGE_PACKAGE: &str = "type-coverage";

pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-typecov/package-present",
            "type-coverage package cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `type-coverage` is installed.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        );
    };
    if crate::common::package_has_dependency(package, TYPE_COVERAGE_PACKAGE) {
        crate::common::info(
            "g3ts-typecov/package-present",
            "type-coverage package is installed",
            format!("`{}` directly installs `type-coverage`.", package.rel_path),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-typecov/package-present",
            "type-coverage package is missing",
            format!(
                "`{}` must install `type-coverage` directly so typecov is reproducible for this app/package root.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "package_present_tests/mod.rs"]
mod package_present_tests;
