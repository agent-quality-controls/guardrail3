use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::{G3RsDepsDependencySection, G3RsDepsResolvedDependency};
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::RustProfile;

use crate::rs_deps_config_05_direct_dependency_cap::rule::check;

pub(super) fn run_check(dependencies: &[G3RsDepsResolvedDependency]) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigChecksInput {
        crate_cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
        crate_name: "api".to_owned(),
        profile: Some(RustProfile::Service),
        allowlist_present: false,
        allowed_deps: Vec::new(),
        dependencies: dependencies.to_vec(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn runtime_dependency(package_name: &str) -> G3RsDepsResolvedDependency {
    G3RsDepsResolvedDependency {
        package_name: package_name.to_owned(),
        section: G3RsDepsDependencySection::Dependencies,
        table_label: "[dependencies]".to_owned(),
    }
}

pub(super) fn build_dependency(package_name: &str) -> G3RsDepsResolvedDependency {
    G3RsDepsResolvedDependency {
        package_name: package_name.to_owned(),
        section: G3RsDepsDependencySection::BuildDependencies,
        table_label: "[build-dependencies]".to_owned(),
    }
}

pub(super) fn dev_dependency(package_name: &str) -> G3RsDepsResolvedDependency {
    G3RsDepsResolvedDependency {
        package_name: package_name.to_owned(),
        section: G3RsDepsDependencySection::DevDependencies,
        table_label: "[dev-dependencies]".to_owned(),
    }
}

pub(super) fn target_runtime_dependency(
    package_name: &str,
    target_key: &str,
) -> G3RsDepsResolvedDependency {
    G3RsDepsResolvedDependency {
        package_name: package_name.to_owned(),
        section: G3RsDepsDependencySection::Dependencies,
        table_label: format!("[target.'{target_key}'.dependencies]"),
    }
}
