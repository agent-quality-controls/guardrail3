use guardrail3_app_rs_family_mapper::RsDenyRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::facts::collect;
use crate::inputs::{
    ConfigDenyInput, CoveredRustUnitInput, SameRootConflictInput, UncoveredRustUnitInput,
};

pub fn check(surface: &FamilyView, route: &RsDenyRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
        crate::coverage::rs_deny_01_coverage::check_policy_context_error(parse_error, &mut results);
    }

    for covered in &facts.covered_units {
        crate::coverage::rs_deny_01_coverage::check_covered(&CoveredRustUnitInput::new(covered), &mut results);
    }
    for uncovered in &facts.uncovered_units {
        crate::coverage::rs_deny_01_coverage::check_uncovered(&UncoveredRustUnitInput::new(uncovered), &mut results);
    }
    for conflict in &facts.same_root_conflicts {
        crate::coverage::rs_deny_03_shadowing::check_same_root_conflict(
            &SameRootConflictInput::new(conflict),
            &mut results,
        );
    }

    for input in ConfigDenyInput::from_facts(&facts) {
        crate::coverage::rs_deny_01_coverage::check_parse_error(&input, &mut results);
        crate::advisories::rs_deny_04_deprecated_advisories::check(&input, &mut results);
        crate::advisories::rs_deny_05_advisories_baseline::check(&input, &mut results);
        crate::advisories::rs_deny_06_stricter_advisories_inventory::check(&input, &mut results);
        crate::advisories::rs_deny_07_graph_all_features::check(&input, &mut results);
        crate::advisories::rs_deny_08_graph_no_default_features::check(&input, &mut results);
        crate::bans::rs_deny_09_ban_baseline_complete::check(&input, &mut results);
        crate::bans::rs_deny_10_multiple_versions_floor::check(&input, &mut results);
        crate::bans::rs_deny_11_highlight_inventory::check(&input, &mut results);
        crate::bans::rs_deny_12_allow_wildcard_paths::check(&input, &mut results);
        crate::bans::rs_deny_13_wildcards_inventory::check(&input, &mut results);
        crate::licenses::rs_deny_14_license_allow_baseline::check(&input, &mut results);
        crate::licenses::rs_deny_15_confidence_threshold::check(&input, &mut results);
        crate::licenses::rs_deny_16_copyleft_allowlist::check(&input, &mut results);
        crate::licenses::rs_deny_17_license_exceptions_inventory::check(&input, &mut results);
        crate::sources::rs_deny_18_unknown_sources_policy::check(&input, &mut results);
        crate::sources::rs_deny_19_allow_registry_baseline::check(&input, &mut results);
        crate::sources::rs_deny_20_allow_git_inventory::check(&input, &mut results);
        crate::bans::rs_deny_21_tokio_full_ban::check(&input, &mut results);
        crate::bans::rs_deny_22_extra_feature_bans_inventory::check(&input, &mut results);
        crate::sources::rs_deny_23_skip_hygiene::check(&input, &mut results);
        crate::sources::rs_deny_24_ignore_hygiene::check(&input, &mut results);
        crate::sources::rs_deny_25_allow_override_channel::check(&input, &mut results);
        crate::bans::rs_deny_26_ban_reason_inventory::check(&input, &mut results);
        crate::bans::rs_deny_27_duplicate_entries::check(&input, &mut results);
        crate::sources::rs_deny_28_unknown_keys::check(&input, &mut results);
        crate::sources::rs_deny_29_ignore_accumulation::check(&input, &mut results);
        crate::sources::rs_deny_30_wrappers::check(&input, &mut results);
    }

    results
}
