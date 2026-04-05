use g3_deps_content_checks_types::G3DepsPolicyContentChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{DependencySectionKind, allowlist_present, allowlisted, dependency_entries_from_policy_input, error, info};

const ID: &str = "RS-DEPS-06";

pub(crate) fn check(input: &G3DepsPolicyContentChecksInput, results: &mut Vec<G3CheckResult>) {
    if !allowlist_present(input) {
        return;
    }

    for entry in dependency_entries_from_policy_input(input) {
        if entry.section_kind != DependencySectionKind::BuildDependencies {
            continue;
        }

        if allowlisted(input, &entry.dep_package_name) {
            results.push(info(
                ID,
                "build dependency allowlisted",
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
            "unauthorized build dependency",
            format!(
                "Build dependency `{}` in `{}` is not allowlisted for crate `{}`. Add it to the dependency allowlist or remove the dependency.",
                entry.dep_package_name, entry.table_label, entry.crate_name
            ),
            entry.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
