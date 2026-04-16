use g3rs_deps_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::{
    G3RsDepsConfigInputScope, G3RsDepsDependencySection, G3RsDepsResolvedDependency,
};
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::RustProfile;

use super::super::rule::check;

pub(super) fn run_check(
    allowlist_present: bool,
    allowed_deps: &[&str],
    dependencies: &[G3RsDepsResolvedDependency],
) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigChecksInput {
        scope: G3RsDepsConfigInputScope::CratePolicy,
        crate_cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
        crate_name: "core".to_owned(),
        profile: Some(RustProfile::Library),
        allowlist_present,
        allowed_deps: allowed_deps.iter().map(|dep| (*dep).to_owned()).collect(),
        dependencies: dependencies.to_vec(),
        installed_tools: Vec::new(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn dev_dependency(package_name: &str) -> G3RsDepsResolvedDependency {
    G3RsDepsResolvedDependency {
        package_name: package_name.to_owned(),
        section: G3RsDepsDependencySection::DevDependencies,
        table_label: "[dev-dependencies]".to_owned(),
    }
}
