use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;

/// `CSPELL_PACKAGE` constant.
const CSPELL_PACKAGE: &str = "cspell";

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-spelling/cspell-package-present",
            "cspell package cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `cspell` is installed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::package_has_dependency(package, CSPELL_PACKAGE) {
        crate::common::info(
            "g3ts-spelling/cspell-package-present",
            "cspell package is installed",
            format!("`{}` directly installs `cspell`.", package.rel_path),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-spelling/cspell-package-present",
            "cspell package is missing",
            format!(
                "`{}` must install `cspell` directly so spelling is reproducible for this app/package root.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "cspell_package_present_tests/mod.rs"]
mod cspell_package_present_tests;
