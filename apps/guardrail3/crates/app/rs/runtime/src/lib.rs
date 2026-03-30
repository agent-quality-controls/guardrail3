use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

mod context;
mod registry;
mod runners;

use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig, RustConfig};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{
    CheckResult, Report, Section, rust_validate_family_cli_name, rust_validate_family_section_name,
};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};
use guardrail3_validation_model::RustValidateFamily;

mod runtime_deps {
    pub(super) use guardrail3_app_core::project_walker;
    pub(super) use guardrail3_app_rs_family_mapper::FamilyMapper;
    pub(super) use guardrail3_app_rs_family_selection as family_selection;
    pub(super) use guardrail3_app_rs_placement as placement;
}

use self::context::RustRunContext;
use self::registry::runner_for;

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
    let tree = runtime_deps::project_walker::walk_project(fs, path);
    let config = match load_config(&tree) {
        Ok(config) => config,
        Err(_error) if requested_families_allow_config_parse_failure(requested_families) => None,
        Err(error) => return Err(error),
    };
    let scope = runtime_deps::placement::collect(&tree);
    let selected =
        runtime_deps::family_selection::resolve(&tree, config.as_ref(), requested_families);
    let applicability = collect_family_applicability(config.as_ref());
    let mapper =
        runtime_deps::FamilyMapper::new(&tree, &scope, config.as_ref(), &selected, scoped_files);
    let ctx = RustRunContext {
        #[cfg(feature = "family-hooks-shared")]
        fs,
        #[cfg(feature = "family-hooks-shared")]
        path,
        tree: &tree,
        mapper: &mapper,
        tc,
        thorough,
    };

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    for family in selected.iter() {
        let Some(runner) = runner_for(family) else {
            return Err(format!(
                "Rust family `{}` is not compiled into this build.",
                rust_validate_family_cli_name(family)
            ));
        };
        let results = (runner.run)(&ctx);
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

fn requested_families_allow_config_parse_failure(
    requested_families: &[RustValidateFamily],
) -> bool {
    !requested_families.is_empty()
        && requested_families.iter().all(|family| {
            matches!(
                family,
                RustValidateFamily::Arch
                    | RustValidateFamily::Hexarch
                    | RustValidateFamily::Libarch
                    | RustValidateFamily::Code
            )
        })
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
pub(crate) fn result_for_tests(file: Option<&str>) -> guardrail3_domain_report::CheckResult {
    guardrail3_domain_report::CheckResult {
        id: "TEST".to_owned(),
        severity: guardrail3_domain_report::Severity::Error,
        title: "test".to_owned(),
        message: "test".to_owned(),
        file: file.map(str::to_owned),
        line: None,
        inventory: false,
    }
}

#[cfg(test)]
pub(crate) fn applicability_for_tests() -> RustFamilyApplicability {
    RustFamilyApplicability {
        global_enabled: false,
        app_enabled: std::collections::BTreeMap::from([
            ("apps/enabled".to_owned(), true),
            ("apps/disabled".to_owned(), false),
        ]),
        packages_enabled: Some(true),
        global_only: false,
    }
}

#[cfg(test)]
pub(crate) fn filter_results_for_applicability_for_tests(
    repo_root: &std::path::Path,
    applicability: &RustFamilyApplicability,
    results: Vec<guardrail3_domain_report::CheckResult>,
) -> Vec<guardrail3_domain_report::CheckResult> {
    filter_results_for_applicability(repo_root, applicability, results)
}

#[cfg(test)]
pub(crate) fn applicability_allows_result_for_tests(
    repo_root: &std::path::Path,
    applicability: &RustFamilyApplicability,
    result: &guardrail3_domain_report::CheckResult,
) -> bool {
    applicability_allows_result(repo_root, applicability, result)
}

#[cfg(test)]
pub(crate) struct LocalFsTest;

#[cfg(test)]
impl guardrail3_outbound_traits::FileSystem for LocalFsTest {
    fn read_file(&self, path: &std::path::Path) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn read_file_err(&self, path: &std::path::Path) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn list_dir(&self, path: &std::path::Path) -> Vec<guardrail3_outbound_traits::FsDirEntry> {
        std::fs::read_dir(path)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .map(guardrail3_outbound_traits::FsDirEntry::from_std)
            .collect()
    }

    fn metadata(&self, path: &std::path::Path) -> Option<guardrail3_outbound_traits::FsMetadata> {
        std::fs::metadata(path)
            .ok()
            .map(guardrail3_outbound_traits::FsMetadata::from_std)
    }
}

#[cfg(test)]
pub(crate) struct StubToolCheckerTest;

#[cfg(test)]
impl guardrail3_outbound_traits::ToolChecker for StubToolCheckerTest {
    fn is_installed(&self, _tool: &str) -> bool {
        false
    }

    fn run_cargo_publish_dry_run_outcome(
        &self,
        _path: &std::path::Path,
    ) -> Option<guardrail3_outbound_traits::CommandRunResult> {
        None
    }
}

#[cfg(test)]
pub(crate) fn temp_root_for_tests(label: &str) -> std::path::PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir()
        .join(format!("guardrail3-{label}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&root).expect("create temp root");
    root
}

#[cfg(test)]
pub(crate) fn write_file_for_tests(root: &std::path::Path, rel: &str, body: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(path, body).expect("write runtime test fixture file");
}

#[cfg(test)]
pub(crate) fn run_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    families: &[RustValidateFamily],
) -> guardrail3_app_core::Result<guardrail3_domain_report::Report> {
    run(fs, root, None, families, false, &StubToolCheckerTest)
}

#[cfg(test)]
#[path = "runtime_tests/mod.rs"] // reason: test-only sidecar module wiring
mod runtime_tests;
