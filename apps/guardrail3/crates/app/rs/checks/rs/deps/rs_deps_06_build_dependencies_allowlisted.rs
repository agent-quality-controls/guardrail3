use crate::domain::report::{CheckResult, Severity};

use super::facts::DependencySectionKind;
use super::inputs::DependencyEntryDepsInput;

const ID: &str = "RS-DEPS-06";

pub fn check(input: &DependencyEntryDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.entry.section_kind != DependencySectionKind::BuildDependencies {
        return;
    }
    if !input.entry.allowlist_present {
        return;
    }

    if input.entry.allowlisted {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "build dependency allowlisted".to_owned(),
                message: format!(
                    "Dependency `{}` in `[build-dependencies]` is allowlisted for crate `{}`.",
                    input.entry.dep_package_name, input.entry.crate_name
                ),
                file: Some(input.entry.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "unauthorized build dependency".to_owned(),
        message: format!(
            "Dependency `{}` in `[build-dependencies]` is not allowlisted for crate `{}`.",
            input.entry.dep_package_name, input.entry.crate_name
        ),
        file: Some(input.entry.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_deps_06_build_dependencies_allowlisted_tests/mod.rs"]
mod tests;
