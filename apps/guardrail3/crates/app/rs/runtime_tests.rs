use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::{
    CommandRunResult, FileSystem, FsDirEntry, FsMetadata, ToolChecker,
};
use guardrail3_validation_model::RustValidateFamily;

use super::{
    RustFamilyApplicability, applicability_allows_result, filter_results_for_applicability, run,
};

fn result(file: Option<&str>) -> CheckResult {
    CheckResult {
        id: "TEST".to_owned(),
        severity: Severity::Error,
        title: "test".to_owned(),
        message: "test".to_owned(),
        file: file.map(str::to_owned),
        line: None,
        inventory: false,
    }
}

fn applicability() -> RustFamilyApplicability {
    RustFamilyApplicability {
        global_enabled: false,
        app_enabled: BTreeMap::from([
            ("apps/enabled".to_owned(), true),
            ("apps/disabled".to_owned(), false),
        ]),
        packages_enabled: Some(true),
        global_only: false,
    }
}

#[test]
fn filters_disabled_app_results_by_path() {
    let filtered = filter_results_for_applicability(
        Path::new("/repo"),
        &applicability(),
        vec![
            result(Some("apps/enabled/Cargo.toml")),
            result(Some("apps/disabled/Cargo.toml")),
            result(Some("packages/lib/Cargo.toml")),
            result(Some("Cargo.toml")),
        ],
    );

    let files = filtered
        .iter()
        .map(|item| item.file.as_deref().unwrap_or("<none>"))
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec!["apps/enabled/Cargo.toml", "packages/lib/Cargo.toml"]
    );
}

#[test]
fn allows_absolute_paths_under_enabled_scope() {
    let result = result(Some("/repo/apps/enabled/src/lib.rs"));
    assert!(applicability_allows_result(
        Path::new("/repo"),
        &applicability(),
        &result
    ));
}

#[test]
fn keeps_rootless_results_for_now() {
    let result = result(None);
    assert!(applicability_allows_result(
        Path::new("/repo"),
        &applicability(),
        &result
    ));
}

struct LocalFs;

impl FileSystem for LocalFs {
    fn read_file(&self, path: &Path) -> Option<String> {
        fs::read_to_string(path).ok()
    }

    fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    fn list_dir(&self, path: &Path) -> Vec<FsDirEntry> {
        fs::read_dir(path)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .map(FsDirEntry::from_std)
            .collect()
    }

    fn metadata(&self, path: &Path) -> Option<FsMetadata> {
        fs::metadata(path).ok().map(FsMetadata::from_std)
    }
}

struct StubToolChecker;

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, _tool: &str) -> bool {
        false
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}

fn temp_root(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH")
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("guardrail3-{label}-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&root).expect("create temp root");
    root
}

fn write_file(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, body).expect("write runtime test fixture file");
}

#[test]
fn arch_runtime_dispatch_uses_arch_section_name() {
    let root = temp_root("arch-runtime-dispatch");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    let live_results = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
        .collect::<Vec<_>>();

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert!(
        live_results.is_empty(),
        "clean app root should not emit live arch findings: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_scoped_arch_config_violation() {
    let root = temp_root("arch-runtime-scoped-config");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\narch = false\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    let live_results = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
        .collect::<Vec<_>>();

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert_eq!(live_results.len(), 1, "{report:#?}");
    assert_eq!(live_results[0].id, "RS-ARCH-05");
    assert_eq!(live_results[0].file.as_deref(), Some("guardrail3.toml"));

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_still_reports_scoped_arch_config_when_global_arch_is_disabled() {
    let root = temp_root("arch-runtime-scoped-config-global-off");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = false\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\narch = false\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    let live_results = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
        .collect::<Vec<_>>();

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert_eq!(live_results.len(), 1, "{report:#?}");
    assert_eq!(live_results[0].id, "RS-ARCH-05");
    assert_eq!(live_results[0].file.as_deref(), Some("guardrail3.toml"));

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = temp_root("arch-runtime-malformed-config");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\nhexarch = \"nope\"\n",
    );
    write_file(
        &root,
        "tools/worker/Cargo.toml",
        "[package]\nname = \"worker\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    let ids = report.sections[0]
        .results
        .iter()
        .map(|result| result.id.as_str())
        .collect::<Vec<_>>();
    assert!(
        ids.contains(&"RS-ARCH-02"),
        "expected misplaced-root reporting after malformed config parse failure: {report:#?}"
    );
    assert!(
        ids.contains(&"RS-ARCH-07"),
        "expected required-input fail-closed reporting for malformed config: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_fail_closed_results_for_malformed_governed_manifest() {
    let root = temp_root("arch-runtime-malformed-governed-cargo");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = []\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    let ids = report.sections[0]
        .results
        .iter()
        .map(|result| result.id.as_str())
        .collect::<Vec<_>>();
    assert!(
        ids.contains(&"RS-ARCH-07"),
        "expected required-input fail-closed reporting for malformed governed manifest: {report:#?}"
    );
    assert!(
        report.sections[0].results.iter().any(|result| {
            result.id == "RS-ARCH-07" && result.file.as_deref() == Some("apps/backend/Cargo.toml")
        }),
        "expected malformed governed manifest to be attributed to apps/backend/Cargo.toml: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_honors_app_scoped_hexarch_override() {
    let root = temp_root("arch-runtime-app-scoped-hexarch");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\nhexarch = false\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/worker\"]\nresolver = \"2\"\n",
    );
    write_file(
        &root,
        "apps/backend/crates/worker/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    let rs_arch_06_files = report.sections[0]
        .results
        .iter()
        .filter(|result| result.id == "RS-ARCH-06" && !result.inventory)
        .filter_map(|result| result.file.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        rs_arch_06_files,
        vec![
            "apps/backend/Cargo.toml",
            "apps/backend/crates/worker/Cargo.toml",
        ],
        "{report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_governed_auxiliary_metadata_as_fail_closed() {
    let root = temp_root("arch-runtime-governed-auxiliary-metadata");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.metadata.guardrail3]\narch_role = \"auxiliary\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert!(
        report.sections[0].results.iter().any(|result| {
            result.id == "RS-ARCH-07" && result.file.as_deref() == Some("apps/backend/Cargo.toml")
        }),
        "expected governed auxiliary metadata to fail closed: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_explicit_request_reports_inactive_misplaced_root_rule() {
    let root = temp_root("arch-runtime-inactive-misplaced");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = false\nhexarch = true\nlibarch = true\n",
    );
    write_file(
        &root,
        "tools/worker/Cargo.toml",
        "[package]\nname = \"worker\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Arch],
        false,
        &StubToolChecker,
    )
    .expect("arch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert!(
        report.sections[0].results.iter().any(|result| {
            result.id == "RS-ARCH-02"
                && result.inventory
                && result.title == "Misplaced-root reporting is inactive"
        }),
        "expected explicit arch request to surface inactive RS-ARCH-02 inventory: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn hexarch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = temp_root("hexarch-runtime-malformed-config");
    write_file(
        &root,
        "guardrail3.toml",
        "[rust.checks]\nhexarch = \"nope\"\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/domain/types\"]\nresolver = \"2\"\n",
    );
    write_file(
        &root,
        "apps/backend/crates/domain/types/Cargo.toml",
        "[package]\nname = \"backend-domain-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        &root,
        "apps/backend/crates/domain/types/src/lib.rs",
        "pub struct Marker;\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Hexarch],
        false,
        &StubToolChecker,
    )
    .expect("hexarch runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "hexarch");
    let ids = report.sections[0]
        .results
        .iter()
        .map(|result| result.id.as_str())
        .collect::<Vec<_>>();
    assert!(
        ids.contains(&"RS-HEXARCH-15"),
        "expected boundary-config fail-closed reporting for malformed config: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = temp_root("code-runtime-malformed-config");
    write_file(&root, "guardrail3.toml", "[rust.checks]\ncode = \"nope\"\n");
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/domain/types\"]\nresolver = \"2\"\n",
    );
    write_file(
        &root,
        "apps/backend/crates/domain/types/Cargo.toml",
        "[package]\nname = \"backend-domain-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        &root,
        "apps/backend/crates/domain/types/src/lib.rs",
        "pub struct Marker;\n",
    );

    let report = run(
        &LocalFs,
        &root,
        None,
        &[RustValidateFamily::Code],
        false,
        &StubToolChecker,
    )
    .expect("code runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "code");
    let ids = report.sections[0]
        .results
        .iter()
        .map(|result| result.id.as_str())
        .collect::<Vec<_>>();
    assert!(
        ids.contains(&"RS-CODE-30"),
        "expected code-family fail-closed reporting for malformed config: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_scoped_files_limit_config_results_to_active_root() {
    let root = temp_root("code-runtime-scoped-config-results");
    write_file(
        &root,
        "guardrail3.toml",
        "# EXCEPTION: root policy note\n[rust.checks]\ncode = true\n",
    );
    write_file(
        &root,
        "apps/backend/Cargo.toml",
        "# EXCEPTION: backend workspace lint inventory\n[workspace]\nmembers = []\nresolver = \"2\"\n[workspace.lints.rust]\nunsafe_code = \"warn\"\n",
    );
    write_file(
        &root,
        "apps/backend/src/lib.rs",
        "pub struct BackendMarker;\n",
    );
    write_file(
        &root,
        "apps/worker/Cargo.toml",
        "# EXCEPTION: worker workspace lint inventory\n[workspace]\nmembers = []\nresolver = \"2\"\n[workspace.lints.rust]\nunsafe_code = \"warn\"\n",
    );
    write_file(
        &root,
        "apps/worker/src/lib.rs",
        "pub struct WorkerMarker;\n",
    );

    let scoped_files = std::collections::BTreeSet::from(["apps/backend/src/lib.rs".to_owned()]);
    let report = run(
        &LocalFs,
        &root,
        Some(&scoped_files),
        &[RustValidateFamily::Code],
        false,
        &StubToolChecker,
    )
    .expect("code runtime report");

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "code");
    assert!(
        report.sections[0]
            .results
            .iter()
            .all(|result| result.file.as_deref() == Some("apps/backend/Cargo.toml")),
        "scoped code results should stay on the active root only: {report:#?}"
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}
