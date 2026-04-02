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
    pub(super) use guardrail3_app_rs_family_selection as family_selection;

    #[cfg(feature = "routing")]
    pub(super) use guardrail3_app_rs_legality as legality;

    #[cfg(feature = "routing")]
    pub(super) use guardrail3_app_rs_structure as structure;

    #[cfg(feature = "routing")]
    pub(super) use guardrail3_app_rs_family_mapper::FamilyMapper;
}

use self::context::RustRunContext;
use self::registry::runner_for;

#[derive(Debug)]
pub enum RustRunError {
    ConfigParse(toml::de::Error),
    FamilyNotCompiled(RustValidateFamily),
}

impl std::fmt::Display for RustRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigParse(error) => write!(f, "Error parsing guardrail3.toml: {error}"),
            Self::FamilyNotCompiled(family) => write!(
                f,
                "Rust family `{}` is not compiled into this build.",
                rust_validate_family_cli_name(*family)
            ),
        }
    }
}

impl std::error::Error for RustRunError {}

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
    project_root: &Path,
    validation_scope: Option<&str>,
    scoped_files: Option<&BTreeSet<String>>,
    requested_families: &[RustValidateFamily],
    _thorough: bool,
    _tc: &dyn ToolChecker,
) -> Result<Report, RustRunError> {
    let tree = runtime_deps::project_walker::walk_project(fs, project_root);
    let config = match load_config(&tree) {
        Ok(config) => config,
        Err(_error) if requested_families_allow_config_parse_failure(requested_families) => None,
        Err(error) => return Err(error),
    };
    let selected =
        runtime_deps::family_selection::resolve(&tree, config.as_ref(), requested_families);
    let applicability = collect_family_applicability(config.as_ref());
    #[cfg(feature = "routing")]
    let structure = runtime_deps::structure::collect(tree);  // tree consumed
    #[cfg(feature = "routing")]
    let legality = runtime_deps::legality::collect(structure);  // structure consumed
    #[cfg(feature = "routing")]
    let mapper = runtime_deps::FamilyMapper::from_legality(
        &legality,
        config.as_ref(),
        &selected,
        scoped_files,
    )
    .with_validation_scope(validation_scope);
    #[cfg(not(feature = "routing"))]
    let _ = (scoped_files, validation_scope);
    let ctx = RustRunContext {
        #[cfg(feature = "family-hooks-shared")]
        fs,
        #[cfg(feature = "family-hooks-shared")]
        path: project_root,
        #[cfg(feature = "routing")]
        legality: &legality,
        #[cfg(feature = "routing")]
        mapper: &mapper,
        #[cfg(any(
            feature = "family-deps",
            feature = "family-test",
            feature = "family-release",
            feature = "family-hooks-shared",
            feature = "family-hooks-rs",
        ))]
        tc: _tc,
        #[cfg(feature = "family-release")]
        thorough: _thorough,
    };

    let mut report = Report::new(project_root.display().to_string(), vec!["Rust".to_owned()]);

    for family in selected.iter() {
        let Some(runner) = runner_for(family) else {
            return Err(RustRunError::FamilyNotCompiled(family));
        };
        let results = (runner.run)(&ctx);
        let results = match applicability.get(&family) {
            Some(value) => filter_results_for_applicability(project_root, value, results),
            None => results,
        };
        report.add_section(Section::new(
            rust_validate_family_section_name(family).to_owned(),
            results,
        ));
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
                RustValidateFamily::Topology
                    | RustValidateFamily::Arch
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
                family_applicability(family, config.and_then(GuardrailConfig::rust)),
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
        .and_then(RustConfig::checks)
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
        .and_then(RustConfig::apps)
        .map(|apps| {
            apps.iter()
                .map(|(name, cfg)| {
                    (
                        format!("apps/{name}"),
                        effective_family_flag(cfg.checks(), family, global_enabled),
                    )
                })
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    let packages_enabled = rust
        .and_then(RustConfig::packages)
        .map(|cfg| effective_family_flag(cfg.checks(), family, global_enabled));

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
    let Some(file) = result.file() else {
        return applicability.global_enabled;
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
        RustResultScope::Other => {
            applicability.global_enabled || applicability_has_any_scoped_enable(applicability)
        }
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
    let segments = rel_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut app_paths = Vec::new();
    let mut package_hits = 0usize;

    for window in segments.windows(2) {
        match window {
            ["apps", app_name] => app_paths.push(format!("apps/{app_name}")),
            ["packages", _] => package_hits += 1,
            _ => {}
        }
    }

    match (app_paths.len(), package_hits) {
        (1, 0) => RustResultScope::App(app_paths.remove(0)),
        (0, 1) => RustResultScope::Packages,
        _ => RustResultScope::Other,
    }
}

fn applicability_has_any_scoped_enable(applicability: &RustFamilyApplicability) -> bool {
    applicability
        .app_enabled
        .values()
        .copied()
        .any(|enabled| enabled)
        || applicability.packages_enabled == Some(true)
}

fn load_config(tree: &ProjectTree) -> Result<Option<GuardrailConfig>, RustRunError> {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return Ok(None);
    };
    toml::from_str::<GuardrailConfig>(content)
        .map(Some)
        .map_err(RustRunError::ConfigParse)
}

fn family_uses_global_only(family: RustValidateFamily) -> bool {
    matches!(
        family,
        RustValidateFamily::Topology
            | RustValidateFamily::Fmt
            | RustValidateFamily::Code
            | RustValidateFamily::Test
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
    guardrail3_domain_report::CheckResult::new(
        "TEST".to_owned(),
        guardrail3_domain_report::Severity::Error,
        "test".to_owned(),
        "test".to_owned(),
    )
    .with_optional_file(file.map(str::to_owned))
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
        guardrail3_shared_fs::read_file(path)
    }

    fn read_file_err(&self, path: &std::path::Path) -> Result<String, std::io::Error> {
        guardrail3_shared_fs::read_file_err(path)
    }

    fn list_dir(&self, path: &std::path::Path) -> Vec<guardrail3_outbound_traits::FsDirEntry> {
        guardrail3_shared_fs::list_dir(path)
            .into_iter()
            .map(guardrail3_outbound_traits::FsDirEntry::from_std)
            .collect()
    }

    fn metadata(&self, path: &std::path::Path) -> Option<guardrail3_outbound_traits::FsMetadata> {
        guardrail3_shared_fs::metadata(path).map(guardrail3_outbound_traits::FsMetadata::from_std)
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
    let root =
        std::env::temp_dir().join(format!("guardrail3-{label}-{}-{nonce}", std::process::id()));
    guardrail3_shared_fs::create_dir_all(&root).expect("create temp root");
    root
}

#[cfg(test)]
pub(crate) fn write_file_for_tests(root: &std::path::Path, rel: &str, body: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        guardrail3_shared_fs::create_dir_all(parent).expect("create parent dirs");
    }
    guardrail3_shared_fs::write_file(&path, body).expect("write runtime test fixture file");
}

#[cfg(test)]
pub(crate) fn run_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    families: &[RustValidateFamily],
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(fs, root, None, None, families, false, &StubToolCheckerTest)
}

#[cfg(test)]
pub(crate) fn run_topology_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Topology])
}

#[cfg(test)]
pub(crate) fn run_hexarch_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Hexarch])
}

#[cfg(test)]
pub(crate) fn run_hexarch_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Hexarch],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_code_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Code])
}

#[cfg(test)]
pub(crate) fn run_toolchain_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Toolchain])
}

#[cfg(test)]
pub(crate) fn run_clippy_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Clippy])
}

#[cfg(test)]
pub(crate) fn run_fmt_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Fmt],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_clippy_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Clippy],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_deny_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Deny])
}

#[cfg(test)]
pub(crate) fn run_deny_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Deny],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_deps_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Deps])
}

#[cfg(test)]
pub(crate) fn run_garde_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Garde])
}

#[cfg(test)]
pub(crate) fn run_garde_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Garde],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_cargo_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Cargo])
}

#[cfg(test)]
pub(crate) fn run_cargo_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Cargo],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_deps_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Deps],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_release_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Release],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_libarch_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Libarch])
}

#[cfg(test)]
pub(crate) fn run_libarch_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Libarch],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_release_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run_for_tests(fs, root, &[RustValidateFamily::Release])
}

#[cfg(test)]
pub(crate) fn run_test_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Test],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_code_with_scoped_files_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    scoped_files: Vec<String>,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    let scoped_files = scoped_files.into_iter().collect::<BTreeSet<_>>();
    run(
        fs,
        root,
        None,
        Some(&scoped_files),
        &[RustValidateFamily::Code],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
pub(crate) fn run_code_with_validation_scope_for_tests(
    fs: &dyn guardrail3_outbound_traits::FileSystem,
    root: &std::path::Path,
    validation_scope: &str,
) -> Result<guardrail3_domain_report::Report, RustRunError> {
    run(
        fs,
        root,
        Some(validation_scope),
        None,
        &[RustValidateFamily::Code],
        false,
        &StubToolCheckerTest,
    )
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: test-only sidecar module wiring
mod lib_tests;
