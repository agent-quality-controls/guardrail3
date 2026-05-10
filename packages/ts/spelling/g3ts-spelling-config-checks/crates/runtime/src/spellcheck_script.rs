use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-spelling/spellcheck-script",
            "Spellcheck script cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove spelling script exists."
            ),
            Some(rel_path),
        );
    };
    let has_spellcheck = package
        .script_names
        .iter()
        .any(|script_name| script_name == "spellcheck")
        && package
            .script_parse_blockers
            .iter()
            .all(|blocker| blocker.script_name != "spellcheck");
    if has_spellcheck {
        crate::common::info(
            "g3ts-spelling/spellcheck-script",
            "Spellcheck script is configured",
            format!(
                "`{}` defines a parseable `spellcheck` script.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-spelling/spellcheck-script",
            "Spellcheck script is missing or unparseable",
            format!(
                "`{}` must define `spellcheck` as `cspell ...`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "spellcheck_script_tests/mod.rs"]
mod spellcheck_script_tests;
