use g3rs_deps_config_checks_types::G3RsDepsConfigDirectDependencyCapInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, unique_direct_dependency_names};

const ID: &str = "RS-DEPS-CONFIG-05";
const MAX_UNIQUE_DIRECT_DEPENDENCIES: usize = 25;

pub(crate) fn check(input: &G3RsDepsConfigDirectDependencyCapInput, results: &mut Vec<G3CheckResult>) {
    let unique_direct_dependency_count = unique_direct_dependency_names(input).len();
    if unique_direct_dependency_count <= MAX_UNIQUE_DIRECT_DEPENDENCIES {
        return;
    }

    let crate_name = crate_name(&input.crate_cargo_rel_path, &input.crate_cargo);
    results.push(error(
        ID,
        "too many direct dependencies",
        format!(
            "Crate `{crate_name}` has {unique_direct_dependency_count} unique direct dependencies (max {MAX_UNIQUE_DIRECT_DEPENDENCIES}). Reduce direct dependencies by consolidating or splitting the crate."
        ),
        &input.crate_cargo_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
