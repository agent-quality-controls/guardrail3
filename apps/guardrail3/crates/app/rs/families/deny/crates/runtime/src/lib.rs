mod deny_support;
pub mod facts;
mod facts_support;
mod inputs;
#[path = "coverage/rs_deny_01_coverage.rs"]
mod rs_deny_01_coverage;
#[path = "coverage/rs_deny_02_allowed_locations.rs"]
mod rs_deny_02_allowed_locations;
#[path = "coverage/rs_deny_03_shadowing.rs"]
mod rs_deny_03_shadowing;
#[path = "advisories/rs_deny_04_deprecated_advisories.rs"]
mod rs_deny_04_deprecated_advisories;
#[path = "advisories/rs_deny_05_advisories_baseline.rs"]
mod rs_deny_05_advisories_baseline;
#[path = "advisories/rs_deny_06_stricter_advisories_inventory.rs"]
mod rs_deny_06_stricter_advisories_inventory;
#[path = "advisories/rs_deny_07_graph_all_features.rs"]
mod rs_deny_07_graph_all_features;
#[path = "advisories/rs_deny_08_graph_no_default_features.rs"]
mod rs_deny_08_graph_no_default_features;
#[path = "bans/rs_deny_09_ban_baseline_complete.rs"]
mod rs_deny_09_ban_baseline_complete;
#[path = "bans/rs_deny_10_multiple_versions_floor.rs"]
mod rs_deny_10_multiple_versions_floor;
#[path = "bans/rs_deny_11_highlight_inventory.rs"]
mod rs_deny_11_highlight_inventory;
#[path = "bans/rs_deny_12_allow_wildcard_paths.rs"]
mod rs_deny_12_allow_wildcard_paths;
#[path = "bans/rs_deny_13_wildcards_inventory.rs"]
mod rs_deny_13_wildcards_inventory;
#[path = "licenses/rs_deny_14_license_allow_baseline.rs"]
mod rs_deny_14_license_allow_baseline;
#[path = "licenses/rs_deny_15_confidence_threshold.rs"]
mod rs_deny_15_confidence_threshold;
#[path = "licenses/rs_deny_16_copyleft_allowlist.rs"]
mod rs_deny_16_copyleft_allowlist;
#[path = "licenses/rs_deny_17_license_exceptions_inventory.rs"]
mod rs_deny_17_license_exceptions_inventory;
#[path = "sources/rs_deny_18_unknown_sources_policy.rs"]
mod rs_deny_18_unknown_sources_policy;
#[path = "sources/rs_deny_19_allow_registry_baseline.rs"]
mod rs_deny_19_allow_registry_baseline;
#[path = "sources/rs_deny_20_allow_git_inventory.rs"]
mod rs_deny_20_allow_git_inventory;
#[path = "bans/rs_deny_21_tokio_full_ban.rs"]
mod rs_deny_21_tokio_full_ban;
#[path = "bans/rs_deny_22_extra_feature_bans_inventory.rs"]
mod rs_deny_22_extra_feature_bans_inventory;
#[path = "sources/rs_deny_23_skip_hygiene.rs"]
mod rs_deny_23_skip_hygiene;
#[path = "sources/rs_deny_24_ignore_hygiene.rs"]
mod rs_deny_24_ignore_hygiene;
#[path = "sources/rs_deny_25_allow_override_channel.rs"]
mod rs_deny_25_allow_override_channel;
#[path = "bans/rs_deny_26_ban_reason_inventory.rs"]
mod rs_deny_26_ban_reason_inventory;
#[path = "bans/rs_deny_27_duplicate_entries.rs"]
mod rs_deny_27_duplicate_entries;
#[path = "sources/rs_deny_28_unknown_keys.rs"]
mod rs_deny_28_unknown_keys;
#[path = "sources/rs_deny_29_ignore_accumulation.rs"]
mod rs_deny_29_ignore_accumulation;
#[path = "sources/rs_deny_30_wrappers.rs"]
mod rs_deny_30_wrappers;

use guardrail3_app_rs_family_mapper::RsDenyRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_deny_assertions as _;

use self::facts::collect;
use self::inputs::{
    ConfigDenyInput, CoveredRustUnitInput, ForbiddenDenyConfigInput, SameRootConflictInput,
    UncoveredRustUnitInput,
};

pub use self::deny_support::expected_ban_names;

pub fn check(tree: &ProjectTree, route: &RsDenyRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
        rs_deny_01_coverage::check_policy_context_error(parse_error, &mut results);
    }

    for covered in &facts.covered_units {
        rs_deny_01_coverage::check_covered(&CoveredRustUnitInput::new(covered), &mut results);
    }
    for uncovered in &facts.uncovered_units {
        rs_deny_01_coverage::check_uncovered(&UncoveredRustUnitInput::new(uncovered), &mut results);
    }
    for forbidden in &facts.forbidden_configs {
        rs_deny_02_allowed_locations::check(
            &ForbiddenDenyConfigInput::new(forbidden),
            &mut results,
        );
        rs_deny_03_shadowing::check_forbidden(
            &ForbiddenDenyConfigInput::new(forbidden),
            &mut results,
        );
    }
    for conflict in &facts.same_root_conflicts {
        rs_deny_03_shadowing::check_same_root_conflict(
            &SameRootConflictInput::new(conflict),
            &mut results,
        );
    }

    for input in ConfigDenyInput::from_facts(&facts) {
        rs_deny_01_coverage::check_parse_error(&input, &mut results);
        rs_deny_04_deprecated_advisories::check(&input, &mut results);
        rs_deny_05_advisories_baseline::check(&input, &mut results);
        rs_deny_06_stricter_advisories_inventory::check(&input, &mut results);
        rs_deny_07_graph_all_features::check(&input, &mut results);
        rs_deny_08_graph_no_default_features::check(&input, &mut results);
        rs_deny_09_ban_baseline_complete::check(&input, &mut results);
        rs_deny_10_multiple_versions_floor::check(&input, &mut results);
        rs_deny_11_highlight_inventory::check(&input, &mut results);
        rs_deny_12_allow_wildcard_paths::check(&input, &mut results);
        rs_deny_13_wildcards_inventory::check(&input, &mut results);
        rs_deny_14_license_allow_baseline::check(&input, &mut results);
        rs_deny_15_confidence_threshold::check(&input, &mut results);
        rs_deny_16_copyleft_allowlist::check(&input, &mut results);
        rs_deny_17_license_exceptions_inventory::check(&input, &mut results);
        rs_deny_18_unknown_sources_policy::check(&input, &mut results);
        rs_deny_19_allow_registry_baseline::check(&input, &mut results);
        rs_deny_20_allow_git_inventory::check(&input, &mut results);
        rs_deny_21_tokio_full_ban::check(&input, &mut results);
        rs_deny_22_extra_feature_bans_inventory::check(&input, &mut results);
        rs_deny_23_skip_hygiene::check(&input, &mut results);
        rs_deny_24_ignore_hygiene::check(&input, &mut results);
        rs_deny_25_allow_override_channel::check(&input, &mut results);
        rs_deny_26_ban_reason_inventory::check(&input, &mut results);
        rs_deny_27_duplicate_entries::check(&input, &mut results);
        rs_deny_28_unknown_keys::check(&input, &mut results);
        rs_deny_29_ignore_accumulation::check(&input, &mut results);
        rs_deny_30_wrappers::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
pub(crate) fn config_facts(deny_toml: &str) -> facts::DenyConfigFacts {
    config_facts_for_test(deny_toml, None)
}

#[cfg(test)]
pub(crate) fn config_facts_with_profile(
    deny_toml: &str,
    profile_name: &str,
) -> facts::DenyConfigFacts {
    config_facts_for_test(deny_toml, Some(profile_name))
}

#[cfg(test)]
pub(crate) fn collected_facts(tree: &ProjectTree) -> facts::DenyFacts {
    collect_facts_for_test(tree)
}

#[cfg(test)]
pub(crate) fn forbidden_input<'a>(
    facts: &'a facts::DenyFacts,
    rel_path: &str,
) -> ForbiddenDenyConfigInput<'a> {
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected forbidden deny config facts");
    ForbiddenDenyConfigInput::new(forbidden)
}

#[cfg(test)]
pub(crate) fn same_root_conflict_input<'a>(
    facts: &'a facts::DenyFacts,
    policy_root_rel: &str,
) -> SameRootConflictInput<'a> {
    let conflict = facts
        .same_root_conflicts
        .iter()
        .find(|conflict| conflict.policy_root_rel == policy_root_rel)
        .expect("expected same-root deny conflict facts");
    SameRootConflictInput::new(conflict)
}

#[cfg(test)]
pub(crate) fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    use guardrail3_adapters_outbound_fs::RealFileSystem;
    use guardrail3_app_core::project_walker::walk_project;
    use guardrail3_app_rs_family_mapper::FamilyMapper;
    use guardrail3_app_rs_placement::collect as collect_scope;
    use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

    let tree = walk_project(&RealFileSystem, root);
    let scope = collect_scope(&tree);
    let selected =
        RustFamilySelection::new(std::collections::BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deny();
    check(&tree, &route)
}

#[cfg(test)]
pub(crate) fn config_facts_for_test(
    deny_toml: &str,
    profile_name: Option<&str>,
) -> facts::DenyConfigFacts {
    let (parsed, parse_error) = match toml::from_str::<toml::Value>(deny_toml) {
        Ok(parsed) => (Some(parsed), None),
        Err(err) => (None, Some(err.to_string())),
    };
    facts::DenyConfigFacts {
        policy_root_rel: String::new(),
        rel_path: "deny.toml".to_owned(),
        file_kind: "deny.toml".to_owned(),
        parsed,
        parse_error,
        profile_name: Some(profile_name.unwrap_or("service").to_owned()),
        policy_context_valid: true,
    }
}

#[cfg(test)]
pub(crate) fn run_config_rule_for_test(
    deny_toml: &str,
    profile_name: Option<&str>,
    rule: fn(&ConfigDenyInput<'_>, &mut Vec<CheckResult>),
) -> Vec<CheckResult> {
    let config = config_facts_for_test(deny_toml, profile_name);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn collect_facts_for_test(tree: &ProjectTree) -> facts::DenyFacts {
    use guardrail3_app_rs_family_mapper::FamilyMapper;
    use guardrail3_app_rs_placement::collect as collect_scope;
    use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

    let scope = collect_scope(tree);
    let selected =
        RustFamilySelection::new(std::collections::BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_deny();
    collect(tree, &route)
}
