use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use guardrail3_app_core::project_walker;
use guardrail3_app_rs_family_arch as arch;
use guardrail3_app_rs_family_cargo as cargo;
use guardrail3_app_rs_family_clippy as clippy;
use guardrail3_app_rs_family_code as code;
use guardrail3_app_rs_family_deny as deny;
use guardrail3_app_rs_family_deps as deps;
use guardrail3_app_rs_family_fmt as fmt;
use guardrail3_app_rs_family_garde as garde;
use guardrail3_app_rs_family_hexarch as hexarch;
use guardrail3_app_rs_family_hooks_shared as hooks_shared;
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_family_release as release;
use guardrail3_app_rs_family_selection as family_selection;
use guardrail3_app_rs_family_test as test;
use guardrail3_app_rs_family_toolchain as toolchain;
use guardrail3_app_rs_placement as placement;
use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig, RustConfig};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Report, Section, rust_validate_family_section_name};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};
use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone)]
struct RustFamilyApplicability {
    global_enabled: bool,
    app_enabled: BTreeMap<String, bool>,
    packages_enabled: Option<bool>,
    global_only: bool,
}

enum RustResultScope {
    App(String),
    Packages,
    Other,
}

pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&BTreeSet<String>>,
    requested_families: &[RustValidateFamily],
    thorough: bool,
    tc: &dyn ToolChecker,
) -> Result<Report, String> {
    let tree = project_walker::walk_project(fs, path);
    let config = load_config(&tree)?;
    let scope = placement::collect(&tree);
    let selected = family_selection::resolve(&tree, config.as_ref(), requested_families);
    let applicability = collect_family_applicability(config.as_ref());
    let mapper = FamilyMapper::new(&tree, &scope, config.as_ref(), &selected, scoped_files);

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    for family in selected.iter() {
        let results = match family {
            RustValidateFamily::Arch => arch::check(&tree, &mapper.map_rs_arch()),
            RustValidateFamily::Fmt => fmt::check(&tree),
            RustValidateFamily::Toolchain => toolchain::check(&tree),
            RustValidateFamily::Clippy => clippy::check(&tree),
            RustValidateFamily::Deny => deny::check(&tree),
            RustValidateFamily::Cargo => cargo::check(&tree),
            RustValidateFamily::Code => code::check(&tree, scoped_files),
            RustValidateFamily::Hexarch => hexarch::check(&tree, &mapper.map_rs_hexarch()),
            RustValidateFamily::Deps => deps::check(&tree, tc),
            RustValidateFamily::Garde => garde::check(&tree, scoped_files),
            RustValidateFamily::Test => test::check(&tree, &mapper.map_rs_test(), tc),
            RustValidateFamily::Release => release::check(&tree, tc, thorough),
            RustValidateFamily::HooksShared => hooks_shared::check(fs, path, &tree, tc),
            RustValidateFamily::HooksRs => Vec::new(),
        };
        let results = match applicability.get(&family) {
            Some(value) => filter_results_for_applicability(path, value, results),
            None => results,
        };
        report.add_section(Section {
            name: rust_validate_family_section_name(family).to_owned(),
            results,
        });
    }

    Ok(report)
}

fn collect_family_applicability(
    config: Option<&GuardrailConfig>,
) -> BTreeMap<RustValidateFamily, RustFamilyApplicability> {
    RustValidateFamily::all()
        .iter()
        .copied()
        .map(|family| {
            (
                family,
                family_applicability(family, config.and_then(|value| value.rust.as_ref())),
            )
        })
        .collect()
}

fn family_applicability(
    family: RustValidateFamily,
    rust: Option<&RustConfig>,
) -> RustFamilyApplicability {
    let global_only = family_uses_global_only(family);
    let global_enabled = rust
        .and_then(|value| value.checks.as_ref())
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);

    if global_only {
        return RustFamilyApplicability {
            global_enabled,
            app_enabled: BTreeMap::new(),
            packages_enabled: None,
            global_only: true,
        };
    }

    let app_enabled = rust
        .and_then(|value| value.apps.as_ref())
        .map(|apps| {
            apps.iter()
                .map(|(name, cfg)| {
                    (
                        format!("apps/{name}"),
                        effective_family_flag(cfg.checks.as_ref(), family, global_enabled),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let packages_enabled = rust
        .and_then(|value| value.packages.as_ref())
        .map(|cfg| effective_family_flag(cfg.checks.as_ref(), family, global_enabled));

    RustFamilyApplicability {
        global_enabled,
        app_enabled,
        packages_enabled,
        global_only: false,
    }
}

fn filter_results_for_applicability(
    project_root: &Path,
    applicability: &RustFamilyApplicability,
    results: Vec<CheckResult>,
) -> Vec<CheckResult> {
    if applicability.global_only {
        return results;
    }

    results
        .into_iter()
        .filter(|result| applicability_allows_result(project_root, applicability, result))
        .collect()
}

fn applicability_allows_result(
    project_root: &Path,
    applicability: &RustFamilyApplicability,
    result: &CheckResult,
) -> bool {
    let Some(file) = result.file.as_deref() else {
        return true;
    };
    let Some(rel_path) = normalize_result_path(project_root, file) else {
        return applicability.global_enabled;
    };

    match scope_for_result_path(&rel_path) {
        RustResultScope::App(app_path) => applicability
            .app_enabled
            .get(&app_path)
            .copied()
            .unwrap_or(applicability.global_enabled),
        RustResultScope::Packages => applicability
            .packages_enabled
            .unwrap_or(applicability.global_enabled),
        RustResultScope::Other => applicability.global_enabled,
    }
}

fn normalize_result_path(project_root: &Path, file: &str) -> Option<String> {
    let candidate = Path::new(file);
    if candidate.is_absolute() {
        candidate
            .strip_prefix(project_root)
            .ok()
            .map(|value| value.to_string_lossy().replace('\\', "/"))
    } else {
        Some(file.trim_start_matches("./").replace('\\', "/"))
    }
}

fn scope_for_result_path(rel_path: &str) -> RustResultScope {
    let mut segments = rel_path.split('/').filter(|segment| !segment.is_empty());
    match (segments.next(), segments.next()) {
        (Some("apps"), Some(app_name)) => RustResultScope::App(format!("apps/{app_name}")),
        (Some("packages"), _) => RustResultScope::Packages,
        _ => RustResultScope::Other,
    }
}

fn load_config(tree: &ProjectTree) -> Result<Option<GuardrailConfig>, String> {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return Ok(None);
    };
    toml::from_str::<GuardrailConfig>(content)
        .map(Some)
        .map_err(|error| format!("Error parsing guardrail3.toml: {error}"))
}

fn family_uses_global_only(family: RustValidateFamily) -> bool {
    matches!(
        family,
        RustValidateFamily::Arch
            | RustValidateFamily::Fmt
            | RustValidateFamily::Toolchain
            | RustValidateFamily::HooksShared
            | RustValidateFamily::HooksRs
    )
}

fn effective_family_flag(
    checks: Option<&RustChecksConfig>,
    family: RustValidateFamily,
    global: bool,
) -> bool {
    checks
        .and_then(|value| value.family_enabled(family))
        .unwrap_or(global)
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
