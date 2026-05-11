use g3rs_deps_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::G3RsDepsConfigInputScope;
use g3rs_toml_parser::types::RustProfile;
use guardrail3_check_types::G3CheckResult;

use super::super::check;

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
