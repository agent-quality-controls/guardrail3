use g3_deps_content_checks::{G3DepsDirectDependencyCapInput, G3DepsPolicyContentChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

use crate::facts::{DepsFacts, collect};
use crate::inputs::{
    InputFailureDepsInput, LockfileDepsInput, ToolDepsInput,
};

pub fn check(
    surface: &FamilyView,
    route: &RsDepsRoute,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route, tc);
    run_with_facts(&facts)
}

pub fn run_with_facts(facts: &DepsFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for tool in &facts.tools {
        let input = ToolDepsInput::new(tool);
        crate::tooling::rs_deps_01_cargo_deny_installed::check(&input, &mut results);
        crate::tooling::rs_deps_02_cargo_machete_installed::check(&input, &mut results);
        crate::tooling::rs_deps_03_cargo_dupes_installed::check(&input, &mut results);
        crate::tooling::rs_deps_04_gitleaks_installed::check(&input, &mut results);
    }

    for input in &facts.policy_content_checks {
        run_policy_content_checks(input, &mut results);
    }

    for lockfile in &facts.lockfiles {
        let input = LockfileDepsInput::new(lockfile);
        crate::policy::rs_deps_09_cargo_lock_present::check(&input, &mut results);
        crate::policy::rs_deps_10_gitignore_not_ignoring_cargo_lock::check(&input, &mut results);
    }

    for failure in &facts.input_failures {
        let input = InputFailureDepsInput::new(failure);
        crate::policy::rs_deps_11_input_failures::check(&input, &mut results);
    }

    for input in &facts.direct_dependency_cap_content_checks {
        run_direct_dependency_cap_check(input, &mut results);
    }
    results
}

fn run_policy_content_checks(
    input: &crate::facts::PolicyContentCheckFacts,
    results: &mut Vec<CheckResult>,
) {
    let package_input = G3DepsPolicyContentChecksInput {
        workspace_cargo_rel_path: input.workspace_cargo_rel_path.clone(),
        workspace_cargo: input.workspace_cargo.clone(),
        crate_cargo_rel_path: input.crate_cargo_rel_path.clone(),
        crate_cargo: input.crate_cargo.clone(),
        guardrail_rel_path: input.guardrail_rel_path.clone(),
        guardrail: toml::from_str(&input.guardrail_content)
            .expect("guardrail3.toml content fact should parse"),
    };
    results.extend(
        g3_deps_content_checks::check_policy(&package_input)
            .into_iter()
            .map(convert_check_result),
    );
}

fn run_direct_dependency_cap_check(
    input: &crate::facts::DirectDependencyCapContentFacts,
    results: &mut Vec<CheckResult>,
) {
    let package_input = G3DepsDirectDependencyCapInput {
        workspace_cargo_rel_path: input.workspace_cargo_rel_path.clone(),
        workspace_cargo: input.workspace_cargo.clone(),
        crate_cargo_rel_path: input.crate_cargo_rel_path.clone(),
        crate_cargo: input.crate_cargo.clone(),
    };
    results.extend(
        g3_deps_content_checks::check_direct_dependency_cap(&package_input)
            .into_iter()
            .map(convert_check_result),
    );
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
