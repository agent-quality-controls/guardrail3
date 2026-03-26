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
    fs::write(path, body).expect("write file");
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

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert!(
        report.sections[0].results.is_empty(),
        "clean app root should not emit arch findings: {report:#?}"
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

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert_eq!(report.sections[0].results.len(), 1, "{report:#?}");
    assert_eq!(report.sections[0].results[0].id, "RS-ARCH-05");
    assert_eq!(
        report.sections[0].results[0].file.as_deref(),
        Some("guardrail3.toml")
    );

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

    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, "arch");
    assert_eq!(report.sections[0].results.len(), 1, "{report:#?}");
    assert_eq!(report.sections[0].results[0].id, "RS-ARCH-05");
    assert_eq!(
        report.sections[0].results[0].file.as_deref(),
        Some("guardrail3.toml")
    );

    fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = temp_root("arch-runtime-malformed-config");
    write_file(&root, "guardrail3.toml", "[rust.checks]\nhexarch = \"nope\"\n");
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
fn hexarch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = temp_root("hexarch-runtime-malformed-config");
    write_file(&root, "guardrail3.toml", "[rust.checks]\nhexarch = \"nope\"\n");
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
