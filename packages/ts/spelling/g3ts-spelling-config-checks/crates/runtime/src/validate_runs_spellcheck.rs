use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-spelling/validate-runs-spellcheck",
            "Validate script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `validate` runs spelling fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::validate_runs_spellcheck(package) {
        crate::common::info(
            "g3ts-spelling/validate-runs-spellcheck",
            "Validate script runs spellcheck",
            format!(
                "`{}` defines a fail-closed `validate` script that reaches `spellcheck` or direct `cspell`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-spelling/validate-runs-spellcheck",
            "Validate script does not run spellcheck",
            format!(
                "`{}` must define a fail-closed `validate` script that invokes `spellcheck` through a package-manager run command or directly invokes `cspell`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
