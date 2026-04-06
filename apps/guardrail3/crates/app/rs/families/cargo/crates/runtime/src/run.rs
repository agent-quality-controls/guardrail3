use g3rs_cargo_config_checks::G3RsCargoConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::discover::collect;
use crate::inputs::{
    InputFailureCargoInput, InputFailureInventoryCargoInput, MissingMemberCargoInput,
    MissingMemberInventoryCargoInput, PolicyRootCargoInput, WorkspaceMemberCargoInput,
};

pub fn check(surface: &FamilyView, route: &RsCargoRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in InputFailureCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_14_input_failures::check(&input, &mut results);
    }
    for input in InputFailureInventoryCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_14_input_failures::check_inventory(&input, &mut results);
    }

    for input in PolicyRootCargoInput::from_facts(&facts) {
        run_content_checks(&input, &mut results);
        if input.root.parsed_typed.is_some() {
            crate::workspace_policy::rs_cargo_03_allow_inventory::check(&input, &mut results);
            crate::workspace_policy::rs_cargo_12_unapproved_allow_entries::check(&input, &mut results);
            crate::workspace_policy::rs_cargo_15_rust_version_policy::check(&input, &mut results);
        }
    }

    for input in WorkspaceMemberCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_04_lint_inheritance::check(&input, &mut results);
        crate::member_policy::rs_cargo_06_no_weakened_overrides::check(&input, &mut results);
        crate::member_policy::rs_cargo_09_member_edition_drift::check(&input, &mut results);
        crate::member_policy::rs_cargo_13_member_local_allows_forbidden::check(&input, &mut results);
    }

    for input in MissingMemberCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_10_missing_member_cargo::check(&input, &mut results);
    }
    for input in MissingMemberInventoryCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_10_missing_member_cargo::check_inventory(&input, &mut results);
    }

    results
}

fn run_content_checks(
    input: &PolicyRootCargoInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    let Some(cargo) = input.root.parsed_typed.clone() else {
        return;
    };

    let package_input = G3RsCargoConfigChecksInput {
        cargo_rel_path: input.root.cargo_rel_path.clone(),
        cargo,
    };
    let package_results = g3rs_cargo_config_checks::check(&package_input);
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

fn convert_severity(severity: G3Severity) -> Severity {
    match severity {
        G3Severity::Error => Severity::Error,
        G3Severity::Warn => Severity::Warn,
        G3Severity::Info => Severity::Info,
    }
}
