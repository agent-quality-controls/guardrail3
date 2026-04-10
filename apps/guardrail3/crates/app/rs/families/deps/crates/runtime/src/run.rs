use std::collections::BTreeMap;

use g3rs_deps_config_checks::G3RsDepsConfigChecksInput;
use g3rs_deps_types::{G3RsDepsDependencySection, G3RsDepsResolvedDependency};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;
use guardrail3_rs_toml_parser::RustProfile;

use crate::facts::{DepsFacts, DependencySectionKind, collect};
use crate::inputs::{InputFailureDepsInput, LockfileDepsInput, ToolDepsInput};

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

    run_config_checks(facts, &mut results);

    for lockfile in &facts.lockfiles {
        let input = LockfileDepsInput::new(lockfile);
        crate::policy::rs_deps_09_cargo_lock_present::check(&input, &mut results);
        crate::policy::rs_deps_10_gitignore_not_ignoring_cargo_lock::check(&input, &mut results);
    }

    for failure in &facts.input_failures {
        let input = InputFailureDepsInput::new(failure);
        crate::policy::rs_deps_11_input_failures::check(&input, &mut results);
    }

    results
}

fn run_config_checks(facts: &DepsFacts, results: &mut Vec<CheckResult>) {
    let mut policy_by_crate = BTreeMap::new();
    for input in &facts.policy_content_checks {
        let (profile, allowlist_present, allowed_deps) =
            deps_policy_for_crate(&input.crate_cargo_rel_path, &input.guardrail_content);
        let _ = policy_by_crate.insert(
            input.crate_cargo_rel_path.clone(),
            (profile, allowlist_present, allowed_deps),
        );
    }

    let mut crate_names = BTreeMap::new();
    for entry in &facts.dependency_entries {
        let _ = crate_names
            .entry(entry.cargo_rel_path.clone())
            .or_insert_with(|| entry.crate_name.clone());
    }
    for entry in &facts.allowlist_coverage {
        let _ = crate_names
            .entry(entry.cargo_rel_path.clone())
            .or_insert_with(|| entry.crate_name.clone());
    }
    for entry in &facts.direct_dependency_caps {
        let _ = crate_names
            .entry(entry.cargo_rel_path.clone())
            .or_insert_with(|| entry.crate_name.clone());
    }

    for (crate_cargo_rel_path, crate_name) in crate_names {
        let dependencies = facts
            .dependency_entries
            .iter()
            .filter(|entry| entry.cargo_rel_path == crate_cargo_rel_path)
            .map(|entry| G3RsDepsResolvedDependency {
                package_name: entry.dep_package_name.clone(),
                section: map_section_kind(entry.section_kind),
                table_label: entry.table_label.clone(),
            })
            .collect();
        let (profile, allowlist_present, allowed_deps) = policy_by_crate
            .get(&crate_cargo_rel_path)
            .cloned()
            .unwrap_or((None, false, Vec::new()));
        let package_input = G3RsDepsConfigChecksInput {
            crate_cargo_rel_path,
            crate_name,
            profile,
            allowlist_present,
            allowed_deps,
            dependencies,
        };
        results.extend(
            g3rs_deps_config_checks::check(&package_input)
                .into_iter()
                .map(convert_check_result),
        );
    }
}

fn map_section_kind(section_kind: DependencySectionKind) -> G3RsDepsDependencySection {
    match section_kind {
        DependencySectionKind::Dependencies => G3RsDepsDependencySection::Dependencies,
        DependencySectionKind::BuildDependencies => G3RsDepsDependencySection::BuildDependencies,
        DependencySectionKind::DevDependencies => G3RsDepsDependencySection::DevDependencies,
    }
}

fn deps_policy_for_crate(
    crate_cargo_rel_path: &str,
    guardrail_content: &str,
) -> (Option<RustProfile>, bool, Vec<String>) {
    let Ok(config) = toml::from_str::<GuardrailConfig>(guardrail_content) else {
        return (None, false, Vec::new());
    };
    let rel_dir = crate_cargo_rel_path
        .rsplit_once('/')
        .map(|(prefix, _)| prefix)
        .unwrap_or("");
    let Some(rust) = config.rust() else {
        return (None, false, Vec::new());
    };

    let crate_config = match governed_zone_scope(rel_dir) {
        Some(GovernedZoneScope::App(app_name)) => rust.apps().and_then(|apps| apps.get(app_name)),
        Some(GovernedZoneScope::Packages) => rust.packages(),
        None => None,
    };

    let profile = crate_config
        .and_then(|cfg| cfg.profile().or_else(|| cfg.type_()))
        .and_then(to_rust_profile)
        .or_else(|| config.profile().and_then(|cfg| to_rust_profile(cfg.name())));
    let allowed_deps = crate_config
        .and_then(|cfg| cfg.allowed_deps())
        .map(|deps| deps.to_vec())
        .unwrap_or_default();
    let allowlist_present = crate_config.and_then(|cfg| cfg.allowed_deps()).is_some();

    (profile, allowlist_present, allowed_deps)
}

fn to_rust_profile(value: &str) -> Option<RustProfile> {
    match value {
        "library" => Some(RustProfile::Library),
        "service" => Some(RustProfile::Service),
        _ => None,
    }
}

enum GovernedZoneScope<'a> {
    App(&'a str),
    Packages,
}

fn governed_zone_scope(rel_dir: &str) -> Option<GovernedZoneScope<'_>> {
    let segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() < 2 {
        return None;
    }

    let mut app_names = Vec::new();
    let mut package_hits = 0usize;
    for window in segments.windows(2) {
        match window {
            ["apps", app_name] => app_names.push(*app_name),
            ["packages", _] => package_hits += 1,
            _ => {}
        }
    }

    match (app_names.len(), package_hits) {
        (1, 0) => Some(GovernedZoneScope::App(app_names[0])),
        (0, 1) => Some(GovernedZoneScope::Packages),
        _ => None,
    }
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
