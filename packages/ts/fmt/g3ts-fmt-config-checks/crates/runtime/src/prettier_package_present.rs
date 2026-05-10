use g3ts_fmt_types::G3TsFmtContractInput;
use guardrail3_check_types::G3CheckResult;

/// npm package name for the Prettier CLI.
const PRETTIER_PACKAGE: &str = "prettier";

/// Runs the corresponding fmt config check.
pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-fmt/prettier-package-present",
            "Prettier package cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `prettier` is installed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::package_has_dependency(package, PRETTIER_PACKAGE) {
        crate::common::info(
            "g3ts-fmt/prettier-package-present",
            "Prettier package is installed",
            format!("`{}` directly installs `prettier`.", package.rel_path),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-fmt/prettier-package-present",
            "Prettier package is missing",
            format!(
                "`{}` must install `prettier` directly so formatting is reproducible for this app/package root.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "prettier_package_present_tests/mod.rs"]
mod prettier_package_present_tests;
