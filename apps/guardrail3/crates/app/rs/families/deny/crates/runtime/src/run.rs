use g3_deny_content_checks::G3DenyContentChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
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
        if input.config.parse_error.is_some() {
            continue;
        }

        run_content_checks(&input, &mut results);
        crate::bans::rs_deny_09_ban_baseline_complete::check(&input, &mut results);
        crate::licenses::rs_deny_17_license_exceptions_inventory::check(&input, &mut results);
        crate::sources::rs_deny_25_allow_override_channel::check(&input, &mut results);
        crate::bans::rs_deny_26_ban_reason_inventory::check(&input, &mut results);
        crate::sources::rs_deny_30_wrappers::check(&input, &mut results);
    }

    results
}

fn run_content_checks(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(deny) = input.config.parsed_typed.clone() else {
        return;
    };
    let package_input = G3DenyContentChecksInput {
        deny_rel_path: input.config.rel_path.clone(),
        deny,
    };
    let package_results = g3_deny_content_checks::check(&package_input);
    results.extend(package_results.into_iter().map(convert_check_result));
}

fn convert_check_result(result: G3CheckResult) -> CheckResult {
    CheckResult::from_parts(
        result.id().to_owned(),
        convert_severity(result.severity()),
        result.title().to_owned(),
        result.message().to_owned(),
        result.file().map(str::to_owned),
        result.line(),
        result.inventory(),
    )
}

fn convert_severity(severity: G3Severity) -> guardrail3_domain_report::Severity {
    match severity {
        G3Severity::Error => guardrail3_domain_report::Severity::Error,
        G3Severity::Warn => guardrail3_domain_report::Severity::Warn,
        G3Severity::Info => guardrail3_domain_report::Severity::Info,
    }
}
