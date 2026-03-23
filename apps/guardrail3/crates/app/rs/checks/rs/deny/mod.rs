mod deny_support;
mod facts;
mod inputs;
mod rs_deny_01_coverage;
mod rs_deny_02_allowed_locations;
mod rs_deny_03_shadowing;
mod rs_deny_04_deprecated_advisories;
mod rs_deny_05_advisories_baseline;
mod rs_deny_06_stricter_advisories_inventory;
mod rs_deny_07_graph_all_features;
mod rs_deny_08_graph_no_default_features;
mod rs_deny_09_ban_baseline_complete;
mod rs_deny_10_multiple_versions_floor;
mod rs_deny_11_highlight_inventory;
mod rs_deny_12_allow_wildcard_paths;
mod rs_deny_13_wildcards_inventory;
mod rs_deny_14_license_allow_baseline;
mod rs_deny_15_confidence_threshold;
mod rs_deny_16_copyleft_allowlist;
mod rs_deny_17_license_exceptions_inventory;
mod rs_deny_18_unknown_sources_policy;
mod rs_deny_19_allow_registry_baseline;
mod rs_deny_20_allow_git_inventory;
mod rs_deny_21_tokio_full_ban;
mod rs_deny_22_extra_feature_bans_inventory;
mod rs_deny_23_skip_hygiene;
mod rs_deny_24_ignore_hygiene;
mod rs_deny_25_allow_override_channel;
mod rs_deny_26_ban_reason_inventory;
mod rs_deny_27_duplicate_entries;
mod rs_deny_28_unknown_keys;
mod rs_deny_29_ignore_accumulation;
mod rs_deny_30_wrappers;

#[cfg(test)]
mod test_support;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::collect;
use self::inputs::{
    ConfigDenyInput, CoveredRustUnitInput, ForbiddenDenyConfigInput, SameRootConflictInput,
    UncoveredRustUnitInput,
};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

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
