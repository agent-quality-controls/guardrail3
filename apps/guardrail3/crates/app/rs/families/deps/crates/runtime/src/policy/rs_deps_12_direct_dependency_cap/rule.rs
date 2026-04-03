use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DirectDependencyCapDepsInput;

const ID: &str = "RS-DEPS-12";
const MAX_UNIQUE_DIRECT_DEPENDENCIES: usize = 25;

pub fn check(input: &DirectDependencyCapDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cap.unique_direct_dependency_count <= MAX_UNIQUE_DIRECT_DEPENDENCIES {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "too many direct dependencies".to_owned(),
        format!(
            "Crate `{}` has {} unique direct dependencies (max {}). Reduce direct dependencies by consolidating or splitting the crate.",
            input.cap.crate_name,
            input.cap.unique_direct_dependency_count,
            MAX_UNIQUE_DIRECT_DEPENDENCIES
        ),
        Some(input.cap.cargo_rel_path.clone()),
        None,
        false,
    ));
}

