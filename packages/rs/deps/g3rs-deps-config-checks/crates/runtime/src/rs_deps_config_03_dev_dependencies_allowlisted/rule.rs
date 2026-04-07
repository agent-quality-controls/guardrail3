use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::G3RsDepsDependencySection;
use guardrail3_check_types::G3CheckResult;

use crate::support::{allowlist_present, allowlisted, dependencies_in_section, info, warn};

const ID: &str = "RS-DEPS-CONFIG-03";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !allowlist_present(input) {
        return;
    }

    for entry in dependencies_in_section(input, G3RsDepsDependencySection::DevDependencies) {
        if allowlisted(input, &entry.package_name) {
            results.push(info(
                ID,
                "dev dependency allowlisted",
                format!(
                    "Dependency `{}` in `{}` is allowlisted for crate `{}`.",
                    entry.package_name, entry.table_label, input.crate_name
                ),
                &input.crate_cargo_rel_path,
            ));
            continue;
        }

        results.push(warn(
            ID,
            "unauthorized dev dependency",
            format!(
                "Dev dependency `{}` in `{}` is not allowlisted for crate `{}`. Add it to the dependency allowlist or remove the dependency.",
                entry.package_name, entry.table_label, input.crate_name
            ),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
