use g3_deps_content_checks_types::G3DepsContentChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{DependencySectionKind, allowlist_present, allowlisted, dependency_entries, error, info};

const ID: &str = "RS-DEPS-05";

pub(crate) fn check(input: &G3DepsContentChecksInput, results: &mut Vec<G3CheckResult>) {
    if !allowlist_present(input) {
        return;
    }

    for entry in dependency_entries(input) {
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
