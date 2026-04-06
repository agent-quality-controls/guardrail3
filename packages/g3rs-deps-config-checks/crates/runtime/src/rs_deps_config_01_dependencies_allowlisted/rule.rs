use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{DependencySectionKind, allowlist_present, allowlisted, dependency_entries_from_policy_input, error, info};

const ID: &str = "RS-DEPS-CONFIG-01";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !allowlist_present(input) {
        return;
    }

    for entry in dependency_entries_from_policy_input(input) {
        if entry.section_kind != DependencySectionKind::Dependencies {
            continue;
        }

        if allowlisted(input, &entry.dep_package_name) {
            results.push(info(
                ID,
                "dependency allowlisted",
                format!(
                    "Dependency `{}` in `{}` is allowlisted for crate `{}`.",
                    entry.dep_package_name, entry.table_label, entry.crate_name
                ),
                entry.cargo_rel_path,
            ));
            continue;
        }

        results.push(error(
            ID,
            "unauthorized dependency",
            format!(
                "Dependency `{}` in `{}` is not allowlisted for crate `{}`. Add it to the dependency allowlist or remove the dependency.",
                entry.dep_package_name, entry.table_label, entry.crate_name
            ),
            entry.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
