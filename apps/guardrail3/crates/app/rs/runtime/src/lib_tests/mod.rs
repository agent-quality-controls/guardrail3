use std::path::Path;

use guardrail3_app_rs_runtime_assertions::runtime as assertions;
use guardrail3_validation_model::RustValidateFamily;

#[test]
fn filters_disabled_app_results_by_path() {
    let filtered = super::filter_results_for_applicability_for_tests(
        Path::new("/repo"),
        &super::applicability_for_tests(),
        vec![
            super::result_for_tests(Some("apps/enabled/Cargo.toml")),
            super::result_for_tests(Some("apps/disabled/Cargo.toml")),
            super::result_for_tests(Some("packages/lib/Cargo.toml")),
            super::result_for_tests(Some("Cargo.toml")),
        ],
    );

    assertions::assert_filtered_files(
        &filtered,
        &["apps/enabled/Cargo.toml", "packages/lib/Cargo.toml", "Cargo.toml"],
    );
}

#[test]
fn allows_absolute_paths_under_enabled_scope() {
    let result = super::result_for_tests(Some("/repo/apps/enabled/src/lib.rs"));
    let allowed = super::applicability_allows_result_for_tests(
        Path::new("/repo"),
        &super::applicability_for_tests(),
        &result,
    );

    assertions::assert_allowed(allowed);
}

#[test]
fn rootless_results_follow_global_enablement() {
    let result = super::result_for_tests(None);
    let allowed = super::applicability_allows_result_for_tests(
        Path::new("/repo"),
        &super::applicability_for_tests(),
        &result,
    );

    assert!(!allowed);
}

#[test]
fn arch_runtime_dispatch_uses_arch_section_name() {
    let root = super::temp_root_for_tests("arch-runtime-dispatch");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_clean_section(&report, "arch");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_scoped_arch_config_violation() {
    let root = super::temp_root_for_tests("arch-runtime-scoped-config");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\narch = false\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_scoped_config_violation(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_still_reports_scoped_arch_config_when_global_arch_is_disabled() {
    let root = super::temp_root_for_tests("arch-runtime-scoped-config-global-off");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = false\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\narch = false\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_scoped_config_violation(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = super::temp_root_for_tests("arch-runtime-malformed-config");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\nhexarch = \"nope\"\n",
    );
    super::write_file_for_tests(
        &root,
        "tools/worker/Cargo.toml",
        "[package]\nname = \"worker\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_fail_closed_malformed_config(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_fail_closed_results_for_malformed_governed_manifest() {
    let root = super::temp_root_for_tests("arch-runtime-malformed-governed-cargo");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = []\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_fail_closed_malformed_governed_manifest(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_honors_app_scoped_hexarch_override() {
    let root = super::temp_root_for_tests("arch-runtime-app-scoped-hexarch");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\nhexarch = false\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/worker\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/worker/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_app_scoped_hexarch_override(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_reports_governed_auxiliary_metadata_as_fail_closed() {
    let root = super::temp_root_for_tests("arch-runtime-governed-auxiliary-metadata");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.metadata.guardrail3]\narch_role = \"auxiliary\"\n",
    );

    let report =
        super::run_arch_for_tests(&super::LocalFsTest, &root).expect("arch runtime report");

    assertions::assert_arch_fail_closed_malformed_governed_manifest(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn arch_runtime_explicit_request_reports_inactive_misplaced_root_rule() {
    let root = super::temp_root_for_tests("arch-runtime-inactive-misplaced");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\narch = false\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "tools/worker/Cargo.toml",
        "[package]\nname = \"worker\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report = super::run_for_tests(&super::LocalFsTest, &root, &[RustValidateFamily::Arch])
        .expect("arch runtime report");

    assertions::assert_arch_inactive_misplaced_root_inventory(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn hexarch_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = super::temp_root_for_tests("hexarch-runtime-malformed-config");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\nhexarch = \"nope\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/domain/types\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/domain/types/Cargo.toml",
        "[package]\nname = \"backend-domain-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/domain/types/src/lib.rs",
        "pub struct Marker;\n",
    );

    let report =
        super::run_hexarch_for_tests(&super::LocalFsTest, &root).expect("hexarch runtime report");

    assertions::assert_hexarch_fail_closed_malformed_config(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn cargo_runtime_rejects_malformed_repo_root_guardrail_config() {
    let root = super::temp_root_for_tests("cargo-runtime-malformed-config");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ncargo = \"nope\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/guardrail3/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let result = super::run_cargo_for_tests(&super::LocalFsTest, &root);

    assert!(
        matches!(result, Err(super::RustRunError::ConfigParse(_))),
        "expected config parse error, got: {result:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = super::temp_root_for_tests("code-runtime-malformed-config");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\ncode = \"nope\"\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn ok() {}\n");

    let report =
        super::run_code_for_tests(&super::LocalFsTest, &root).expect("code runtime report");

    assertions::assert_code_fail_closed_malformed_config(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_scoped_files_limit_config_results_to_active_root() {
    let root = super::temp_root_for_tests("code-runtime-scoped-files");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\ncode = true\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/rustfmt.toml",
        "max_width = 100 # EXCEPTION: backend only\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/rustfmt.toml",
        "max_width = 100 # EXCEPTION: other only\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_code_with_scoped_files_for_tests(
        &super::LocalFsTest,
        &root,
        vec!["apps/backend/src/lib.rs".to_owned()],
    )
    .expect("code runtime report");

    assertions::assert_code_scoped_files_config_result(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_validation_scope_limits_config_results_to_active_root() {
    let root = super::temp_root_for_tests("code-runtime-validation-scope");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\ncode = true\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/rustfmt.toml",
        "max_width = 100 # EXCEPTION: backend only\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/rustfmt.toml",
        "max_width = 100 # EXCEPTION: other only\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_code_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("code runtime report");

    assertions::assert_code_scoped_files_config_result(&report);
    assertions::assert_absent_file(&report, "code", "apps/other/rustfmt.toml");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn toolchain_runtime_accepts_local_nested_workspace_toolchain() {
    let root = super::temp_root_for_tests("toolchain-runtime-local-nested-workspace");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntoolchain = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/guardrail3/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/guardrail3/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );

    let report = super::run_toolchain_for_tests(&super::LocalFsTest, &root)
        .expect("toolchain runtime report");

    assertions::assert_clean_section(&report, "toolchain");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn toolchain_runtime_requires_workspace_local_toolchain_when_repo_root_file_is_not_routed() {
    let root = super::temp_root_for_tests("toolchain-runtime-local-workspace");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntoolchain = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/guardrail3/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );

    let report = super::run_toolchain_for_tests(&super::LocalFsTest, &root)
        .expect("toolchain runtime report");

    assertions::assert_toolchain_requires_local_workspace_toolchain(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deps_runtime_validation_scope_does_not_spill_into_sibling_workspace_members() {
    let root = super::temp_root_for_tests("deps-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        r#"
            [rust.checks]
            deps = false

            [rust.apps.backend]
            profile = "service"
            allowed_deps = ["serde"]

            [rust.apps.backend.checks]
            deps = true
        "#,
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/Cargo.lock", "");
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/Cargo.toml",
        "[package]\nname = \"api\"\n\n[dependencies]\nserde = \"1\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/src/lib.rs",
        "pub fn api() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/other/Cargo.toml",
        "[package]\nname = \"other\"\n\n[dependencies]\ntokio = \"1\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/other/src/lib.rs",
        "pub fn other() {}\n",
    );

    let report = super::run_deps_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/crates/api/src",
    )
    .expect("deps runtime report");

    assert_eq!(report.sections().len(), 1, "{report:#?}");
    assert_eq!(report.sections()[0].name(), "deps");
    assert!(
        !report.sections()[0]
            .results()
            .iter()
            .any(|result| result.file() == Some("apps/backend/crates/other/Cargo.toml")),
        "sibling workspace member leaked into subtree deps run: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn release_runtime_validation_scope_keeps_repo_policy_global_and_excludes_sibling_crates() {
    let root = super::temp_root_for_tests("release-runtime-validation-scope");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\nrelease = true\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/Cargo.toml",
        r#"
            [package]
            name = "api"
            version = "0.1.0"
            description = "api crate"
            license = "MIT"
            repository = "https://example.com/api"
            readme = "README.md"
            keywords = ["api"]
            categories = ["development-tools"]
        "#,
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/README.md",
        &format!("# API\n\n{}\n", "x".repeat(240)),
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/src/lib.rs",
        "pub fn api() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/other/Cargo.toml",
        r#"
            [package]
            name = "other"
            version = "0.1.0"
        "#,
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/other/src/lib.rs",
        "pub fn other() {}\n",
    );

    let report = super::run_release_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/crates/api/src",
    )
    .expect("release runtime report");

    assert_eq!(report.sections().len(), 1, "{report:#?}");
    assert_eq!(report.sections()[0].name(), "release");
    assertions::assert_absent_file(&report, "release", "apps/backend/crates/other/Cargo.toml");
    assertions::assert_result_present(
        &report,
        "release",
        "RS-RELEASE-02",
        Some("release-plz.toml"),
        Some(false),
        None,
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deps_runtime_scoped_opt_in_does_not_emit_global_tool_results() {
    let root = super::temp_root_for_tests("deps-runtime-global-off-tools");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        r#"
            [rust.checks]
            deps = false

            [rust.apps.backend]
            profile = "service"
            allowed_deps = ["serde"]

            [rust.apps.backend.checks]
            deps = true
        "#,
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/Cargo.lock", "");
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/src/deps_probe.rs",
        "pub fn deps_probe() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[package]\nname = \"backend\"\n\n[dependencies]\ntokio = \"1\"\n",
    );

    let report =
        super::run_deps_for_tests(&super::LocalFsTest, &root).expect("deps runtime report");

    assert_eq!(report.sections().len(), 1, "{report:#?}");
    assert_eq!(report.sections()[0].name(), "deps");
    let live_results = report.sections()[0]
        .results()
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert!(
        live_results.iter().any(|result| result.id() == "RS-DEPS-05"
            && result.file() == Some("apps/backend/Cargo.toml")),
        "expected crate-local allowlist violation: {report:#?}"
    );
    assert!(
        !live_results.iter().any(|result| {
            matches!(
                result.id(),
                "RS-DEPS-01" | "RS-DEPS-02" | "RS-DEPS-03" | "RS-DEPS-04"
            )
        }),
        "global tool results leaked through scoped opt-in: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deps_runtime_ignores_repo_workspace_root_when_enabled_descendant_app_is_not_a_workspace() {
    let root = super::temp_root_for_tests("deps-runtime-repo-workspace-root");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        r#"
            [rust.checks]
            deps = false

            [rust.apps.api]
            profile = "service"
            allowed_deps = ["serde"]

            [rust.apps.api.checks]
            deps = true
        "#,
    );
    super::write_file_for_tests(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/*\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "Cargo.lock", "");
    super::write_file_for_tests(
        &root,
        "apps/api/Cargo.toml",
        "[package]\nname = \"api\"\n\n[dependencies]\nserde = \"1\"\n",
    );
    super::write_file_for_tests(&root, "apps/api/src/lib.rs", "pub fn api() {}\n");

    let report =
        super::run_deps_for_tests(&super::LocalFsTest, &root).expect("deps runtime report");

    assert_eq!(report.sections().len(), 1, "{report:#?}");
    assert_eq!(report.sections()[0].name(), "deps");
    assertions::assert_clean_section(&report, "deps");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}
