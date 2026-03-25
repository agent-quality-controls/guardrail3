use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    collected_facts, config_input, published_library_package_root_tree, remove_ban_path,
};
use super::super::check;

#[test]
fn inventories_library_only_global_state_type_bans_when_library_profile_baseline_is_present() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-05"));
    assert!(results.iter().any(|result| {
        result.severity == Severity::Info
            && result.inventory
            && result.message == "`std::sync::LazyLock` is banned."
    }));
    assert!(results.iter().any(|result| {
        result.severity == Severity::Info
            && result.inventory
            && result.message == "`once_cell::sync::OnceCell` is banned."
    }));
}

#[test]
fn errors_when_library_profile_is_missing_global_state_type_bans() {
    let mut clippy = build_clippy_toml("library", false, true, "", "");
    for path in ["std::sync::LazyLock", "once_cell::sync::OnceCell"] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = published_library_package_root_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.iter().any(|result| {
        result.severity == Severity::Error
            && result.message == "`std::sync::LazyLock` is not present in `disallowed-types`."
    }));
    assert!(results.iter().any(|result| {
        result.severity == Severity::Error
            && result.message == "`once_cell::sync::OnceCell` is not present in `disallowed-types`."
    }));
}
