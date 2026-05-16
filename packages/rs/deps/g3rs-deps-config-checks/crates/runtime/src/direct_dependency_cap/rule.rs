use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, unique_direct_dependency_names};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deps/direct-dependency-cap";
/// Constant value used by the surrounding module.
const MAX_UNIQUE_DIRECT_DEPENDENCIES: usize = 25;

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let unique_direct_dependency_count = unique_direct_dependency_names(input).len();
    if unique_direct_dependency_count <= MAX_UNIQUE_DIRECT_DEPENDENCIES {
        return;
    }

    results.push(error(
        ID,
        "too many direct dependencies",
        format!(
            "Crate `{}` has {unique_direct_dependency_count} unique direct dependencies (max {MAX_UNIQUE_DIRECT_DEPENDENCIES}). Reduce direct dependencies by consolidating or splitting the crate.",
            input.crate_name
        ),
        &input.crate_cargo_rel_path,
    ));
}
