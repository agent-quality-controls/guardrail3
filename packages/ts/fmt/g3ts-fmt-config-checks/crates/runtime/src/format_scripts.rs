use g3ts_fmt_types::G3TsFmtContractInput;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    let rel_path = crate::common::package_rel_path(&contract.package);
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-fmt/format-scripts",
            "Format scripts cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove formatter scripts exist.",
                rel_path.unwrap_or("package.json")
            ),
            rel_path,
        );
    };
    let has_format = crate::common::script_invokes_prettier(package, "format", "--write");
    let has_check = package
        .script_names
        .iter()
        .any(|script_name| script_name == "format:check")
        && package
            .script_parse_blockers
            .iter()
            .all(|blocker| blocker.script_name != "format:check");
    if has_format && has_check {
        crate::common::info(
            "g3ts-fmt/format-scripts",
            "Format scripts are configured",
            format!(
                "`{}` defines parseable `format` and `format:check` scripts.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-fmt/format-scripts",
            "Format scripts are incomplete",
            format!(
                "`{}` must define `format` as `prettier --write ...` and a parseable `format:check` script.",
                package.rel_path
            ),
            Some(&package.rel_path),
        )
    }
}
