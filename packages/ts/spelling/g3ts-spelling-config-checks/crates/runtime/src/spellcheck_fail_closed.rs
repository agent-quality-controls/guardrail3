use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-spelling/spellcheck-fail-closed",
            "Spellcheck script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `spellcheck` runs cspell fail-closed."
            ),
            Some(rel_path),
        );
    };
    if crate::common::script_invokes_cspell(package, "spellcheck") {
        crate::common::info(
            "g3ts-spelling/spellcheck-fail-closed",
            "Spellcheck script is fail-closed",
            format!(
                "`{}` defines `spellcheck` with a fail-closed `cspell` invocation.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-spelling/spellcheck-fail-closed",
            "Spellcheck script is not fail-closed",
            format!(
                "`{}` must define `spellcheck` as a fail-closed `cspell ...` invocation without `||` fallback.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "spellcheck_fail_closed_tests/mod.rs"]
mod spellcheck_fail_closed_tests;
