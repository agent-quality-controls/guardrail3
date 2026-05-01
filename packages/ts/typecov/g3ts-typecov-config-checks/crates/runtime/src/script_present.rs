use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-typecov/script-present",
            "Typecov script cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove typecov script exists.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        );
    };
    let has_typecov = package
        .script_names
        .iter()
        .any(|script_name| script_name == "typecov")
        && package
            .script_parse_blockers
            .iter()
            .all(|blocker| blocker.script_name != "typecov");
    if has_typecov {
        crate::common::info(
            "g3ts-typecov/script-present",
            "Typecov script is configured",
            format!(
                "`{}` defines a parseable `typecov` script.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-typecov/script-present",
            "Typecov script is missing or unparseable",
            format!(
                "`{}` must define `typecov` as `type-coverage ...`.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "script_present_tests/mod.rs"]
mod script_present_tests;
