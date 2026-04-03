use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::DependencySectionKind;
use crate::inputs::DependencyEntryDepsInput;

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
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "build dependency allowlisted".to_owned(),
                format!(
                    "Dependency `{}` in `{}` is allowlisted for crate `{}`.",
                    input.entry.dep_package_name, input.entry.table_label, input.entry.crate_name
                ),
                Some(input.entry.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "unauthorized build dependency".to_owned(),
        format!(
            "Build dependency `{}` in `{}` is not allowlisted for crate `{}`. Add it to the dependency allowlist or remove the dependency.",
            input.entry.dep_package_name, input.entry.table_label, input.entry.crate_name
        ),
        Some(input.entry.cargo_rel_path.clone()),
        None,
        false,
    ));
}

