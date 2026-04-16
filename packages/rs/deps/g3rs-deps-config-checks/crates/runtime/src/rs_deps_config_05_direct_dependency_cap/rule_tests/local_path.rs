use g3rs_deps_config_checks_assertions::rs_deps_config_05_direct_dependency_cap::rule as assertions;

use super::helpers::{run_check, runtime_dependency};

#[test]
fn duplicate_normalized_package_name_stays_at_cap() {
    let mut dependencies = (0..24)
        .map(|idx| runtime_dependency(&format!("dep_{idx}")))
        .collect::<Vec<_>>();
    dependencies.push(runtime_dependency("serde"));
    dependencies.push(runtime_dependency("serde"));

    let results = run_check(&dependencies);

    assertions::assert_no_findings(&results);
}
