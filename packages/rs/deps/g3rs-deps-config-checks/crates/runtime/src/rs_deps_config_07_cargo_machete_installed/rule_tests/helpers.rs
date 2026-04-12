use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::G3RsDepsConfigInputScope;
use guardrail3_check_types::G3CheckResult;

use crate::rs_deps_config_07_cargo_machete_installed::rule::check;

pub(super) fn run_check(installed_tools: &[&str]) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigChecksInput {
        scope: G3RsDepsConfigInputScope::WorkspaceTooling,
        crate_cargo_rel_path: "Cargo.toml".to_owned(),
        crate_name: "workspace".to_owned(),
        profile: None,
        allowlist_present: false,
        allowed_deps: Vec::new(),
        dependencies: Vec::new(),
        installed_tools: installed_tools.iter().map(|tool| (*tool).to_owned()).collect(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
