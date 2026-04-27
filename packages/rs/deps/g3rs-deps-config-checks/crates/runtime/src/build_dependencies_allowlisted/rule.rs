use g3rs_deps_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::G3RsDepsDependencySection;
use guardrail3_check_types::G3CheckResult;

use crate::support::{allowlist_present, allowlisted, dependencies_in_section, error, info};

const ID: &str = "g3rs-deps/build-dependencies-allowlisted";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !allowlist_present(input) {
        return;
    }

    for entry in dependencies_in_section(input, G3RsDepsDependencySection::BuildDependencies) {
        if allowlisted(input, &entry.package_name) {
            results.push(info(
                ID,
                "build dependency allowlisted",
                format!(
                    "Dependency `{}` in `{}` is allowlisted for crate `{}`.",
                    entry.package_name, entry.table_label, input.crate_name
                ),
                &input.crate_cargo_rel_path,
            ));
            continue;
        }

        results.push(error(
            ID,
            "unauthorized build dependency",
            format!(
                "Build dependency `{}` in `{}` is not allowlisted for crate `{}`. Add it to the dependency allowlist or remove the dependency.",
                entry.package_name, entry.table_label, input.crate_name
            ),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
