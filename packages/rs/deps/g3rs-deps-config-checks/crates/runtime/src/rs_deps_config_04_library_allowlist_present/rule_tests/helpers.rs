use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::G3RsDepsConfigInputScope;
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::RustProfile;

use crate::rs_deps_config_04_library_allowlist_present::rule::check;

pub(super) fn run_check(
    profile: Option<RustProfile>,
    allowlist_present: bool,
) -> Vec<G3CheckResult> {
    let input = G3RsDepsConfigChecksInput {
        scope: G3RsDepsConfigInputScope::CratePolicy,
        crate_cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
        crate_name: "core".to_owned(),
        profile,
        allowlist_present,
        allowed_deps: if allowlist_present {
            vec!["serde".to_owned()]
        } else {
            Vec::new()
        },
        dependencies: Vec::new(),
        installed_tools: Vec::new(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
