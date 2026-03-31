use std::path::Path;

use guardrail3_app_rs_runtime_assertions::runtime as assertions;
use guardrail3_domain_report::{Report, Section};
use guardrail3_validation_model::RustValidateFamily;

fn section<'a>(report: &'a Report, name: &str) -> &'a Section {
    report
        .sections()
        .iter()
        .find(|section| section.name() == name)
        .unwrap_or_else(|| panic!("missing section `{name}`: {report:#?}"))
}

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
        &[
            "apps/enabled/Cargo.toml",
            "packages/lib/Cargo.toml",
            "Cargo.toml",
        ],
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
fn topology_runtime_dispatch_uses_topology_section_name() {
    let root = super::temp_root_for_tests("topology-runtime-dispatch");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_clean_section(&report, "topology");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_reports_scoped_topology_config_violation() {
    let root = super::temp_root_for_tests("topology-runtime-scoped-config");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\ntopology = false\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_scoped_config_violation(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_still_reports_scoped_topology_config_when_global_topology_is_disabled() {
    let root = super::temp_root_for_tests("topology-runtime-scoped-config-global-off");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = false\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\ntopology = false\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_scoped_config_violation(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_reports_fail_closed_results_for_malformed_guardrail_config() {
    let root = super::temp_root_for_tests("topology-runtime-malformed-config");
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
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_fail_closed_malformed_config(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_reports_fail_closed_results_for_malformed_governed_manifest() {
    let root = super::temp_root_for_tests("topology-runtime-malformed-governed-cargo");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = []\n",
    );

    let report =
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_fail_closed_malformed_governed_manifest(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_honors_app_scoped_hexarch_override() {
    let root = super::temp_root_for_tests("topology-runtime-app-scoped-hexarch");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\nhexarch = false\n",
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
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_app_scoped_hexarch_override(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_reports_governed_auxiliary_metadata_as_fail_closed() {
    let root = super::temp_root_for_tests("topology-runtime-governed-auxiliary-metadata");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.metadata.guardrail3]\ntopology_role = \"auxiliary\"\n",
    );

    let report =
        super::run_topology_for_tests(&super::LocalFsTest, &root).expect("topology runtime report");

    assertions::assert_topology_fail_closed_malformed_governed_manifest(&report);

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn topology_runtime_stays_absent_when_only_other_family_is_requested() {
    let root = super::temp_root_for_tests("topology-runtime-not-auto-selected");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = false\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "tools/worker/Cargo.toml",
        "[package]\nname = \"worker\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );

    let report = super::run_for_tests(&super::LocalFsTest, &root, &[RustValidateFamily::Toolchain])
        .expect("toolchain runtime report");

    assertions::assert_section_absent(&report, "topology");
    assertions::assert_ids_absent(&report, &["RS-TOPOLOGY-02"]);

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
fn hexarch_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("hexarch-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/app\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/app/Cargo.toml",
        "[package]\nname = \"backend-app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/app/src/lib.rs",
        "pub fn backend() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report =
        super::run_hexarch_for_tests(&super::LocalFsTest, &root).expect("hexarch runtime report");

    assertions::assert_result_present(
        &report,
        "hexarch",
        "RS-HEXARCH-01",
        Some("apps/backend"),
        Some(true),
        Some("Service `backend` has crates/ directory"),
    );
    assertions::assert_result_present(
        &report,
        "hexarch",
        "RS-HEXARCH-01",
        Some("apps/other"),
        Some(false),
        Some("Service `other` missing crates/ directory"),
    );
    assert_eq!(
        section(&report, "hexarch")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-HEXARCH-01"
                    && matches!(result.file(), Some("apps/backend" | "apps/other"))
            })
            .count(),
        2,
        "expected one hexarch crates/ coverage result per legal app workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn hexarch_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("hexarch-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/app\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/app/Cargo.toml",
        "[package]\nname = \"backend-app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/app/src/lib.rs",
        "pub fn backend() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = [\"crates/app\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/app/Cargo.toml",
        "[package]\nname = \"other-app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/app/src/lib.rs",
        "pub fn other() {}\n",
    );

    let report = super::run_hexarch_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/crates/app/src",
    )
    .expect("hexarch runtime report");

    assertions::assert_result_present(
        &report,
        "hexarch",
        "RS-HEXARCH-01",
        Some("apps/backend"),
        Some(true),
        None,
    );
    assert!(
        !section(&report, "hexarch")
            .results()
            .iter()
            .filter_map(|result| result.file())
            .any(|file| file.starts_with("apps/other")),
        "scoped hexarch run leaked into sibling app workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn hexarch_runtime_stays_decoupled_from_topology_exactness() {
    let root = super::temp_root_for_tests("hexarch-runtime-decoupled-topology-exactness");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/app/Cargo.toml",
        "[package]\nname = \"backend-app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report =
        super::run_hexarch_for_tests(&super::LocalFsTest, &root).expect("hexarch runtime report");

    assertions::assert_section_absent(&report, "topology");
    assertions::assert_ids_absent(
        &report,
        &[
            "RS-TOPOLOGY-12",
            "RS-HEXARCH-07",
            "RS-HEXARCH-09",
            "RS-LIBARCH-05",
            "RS-LIBARCH-06",
        ],
    );

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
fn code_runtime_scoped_files_do_not_narrow_global_family() {
    let root = super::temp_root_for_tests("code-runtime-global-scoped-files");
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
        "apps/backend/src/lib.rs",
        "pub fn backend() { todo!() }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/src/lib.rs",
        "pub fn other() { todo!() }\n",
    );

    let report = super::run_code_with_scoped_files_for_tests(
        &super::LocalFsTest,
        &root,
        vec!["apps/backend/src/lib.rs".to_owned()],
    )
    .expect("code runtime report");

    assertions::assert_result_present(
        &report,
        "code",
        "RS-CODE-13",
        Some("apps/backend/src/lib.rs"),
        Some(false),
        Some("todo! macro"),
    );
    assertions::assert_result_present(
        &report,
        "code",
        "RS-CODE-13",
        Some("apps/other/src/lib.rs"),
        Some(false),
        Some("todo! macro"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_validation_scope_does_not_narrow_global_family() {
    let root = super::temp_root_for_tests("code-runtime-global-validation-scope");
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
        "apps/backend/src/lib.rs",
        "pub fn backend() { todo!() }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/src/lib.rs",
        "pub fn other() { todo!() }\n",
    );

    let report = super::run_code_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("code runtime report");

    assertions::assert_result_present(
        &report,
        "code",
        "RS-CODE-13",
        Some("apps/backend/src/lib.rs"),
        Some(false),
        Some("todo! macro"),
    );
    assertions::assert_result_present(
        &report,
        "code",
        "RS-CODE-13",
        Some("apps/other/src/lib.rs"),
        Some(false),
        Some("todo! macro"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn code_runtime_repo_global_surface_includes_rust_files_outside_cargo_roots() {
    let root = super::temp_root_for_tests("code-runtime-rootless-rust-file");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\ncode = true\n");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(&root, "tools/stray.rs", "pub fn stray() { todo!() }\n");

    let report =
        super::run_code_for_tests(&super::LocalFsTest, &root).expect("code runtime report");

    assertions::assert_result_present(
        &report,
        "code",
        "RS-CODE-13",
        Some("tools/stray.rs"),
        Some(false),
        Some("todo! macro"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn fmt_runtime_validation_scope_does_not_narrow_global_family() {
    let root = super::temp_root_for_tests("fmt-runtime-global-validation-scope");
    super::write_file_for_tests(&root, "guardrail3.toml", "[rust.checks]\nfmt = true\n");
    super::write_file_for_tests(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/backend\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_fmt_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("fmt runtime report");

    assertions::assert_result_present(
        &report,
        "fmt",
        "RS-FMT-01",
        Some(""),
        Some(false),
        Some("rustfmt config missing"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn fmt_runtime_validation_scope_keeps_root_toolchain_state() {
    let root = super::temp_root_for_tests("fmt-runtime-root-toolchain-state");
    super::write_file_for_tests(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/backend\"]\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "rustfmt.toml",
        "edition = \"2024\"\ngroup_imports = \"StdExternalCrate\"\n",
    );
    super::write_file_for_tests(
        &root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_fmt_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("fmt runtime report");

    assertions::assert_result_present(
        &report,
        "fmt",
        "RS-FMT-04",
        Some("rustfmt.toml"),
        Some(false),
        Some("nightly-only rustfmt setting `group_imports` on stable"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn fmt_runtime_validation_scope_keeps_root_cargo_state() {
    let root = super::temp_root_for_tests("fmt-runtime-root-cargo-state");
    super::write_file_for_tests(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/backend\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "rustfmt.toml", "edition = \"2024\"\n");
    super::write_file_for_tests(
        &root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_fmt_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("fmt runtime report");

    assertions::assert_result_present(
        &report,
        "fmt",
        "RS-FMT-06",
        Some("Cargo.toml"),
        Some(false),
        Some("Cargo.toml edition missing"),
    );

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
fn toolchain_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("toolchain-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntoolchain = true\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );

    let report = super::run_toolchain_for_tests(&super::LocalFsTest, &root)
        .expect("toolchain runtime report");

    assertions::assert_result_present(
        &report,
        "toolchain",
        "RS-TOOLCHAIN-01",
        Some("apps/backend/rust-toolchain.toml"),
        Some(true),
        Some("rust-toolchain.toml exists"),
    );
    assertions::assert_result_present(
        &report,
        "toolchain",
        "RS-TOOLCHAIN-01",
        Some("apps/other/rust-toolchain.toml"),
        Some(false),
        Some("rust-toolchain.toml missing"),
    );
    assert_eq!(
        section(&report, "toolchain")
            .results()
            .iter()
            .filter(|result| result.id() == "RS-TOOLCHAIN-01")
            .count(),
        2,
        "expected exactly one toolchain coverage result per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn clippy_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("clippy-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");

    let report =
        super::run_clippy_for_tests(&super::LocalFsTest, &root).expect("clippy runtime report");

    assertions::assert_result_present(
        &report,
        "clippy",
        "RS-CLIPPY-01",
        Some("apps/backend"),
        Some(false),
        Some("Rust unit uncovered by clippy.toml"),
    );
    assertions::assert_result_present(
        &report,
        "clippy",
        "RS-CLIPPY-01",
        Some("apps/other"),
        Some(false),
        Some("Rust unit uncovered by clippy.toml"),
    );
    assert_eq!(
        section(&report, "clippy")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-CLIPPY-01"
                    && !result.inventory()
                    && matches!(result.file(), Some("apps/backend" | "apps/other"))
            })
            .count(),
        2,
        "expected exactly one clippy coverage failure per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deny_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("deny-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(&root, "apps/backend/deny.toml", "[bans");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");
    super::write_file_for_tests(&root, "apps/other/deny.toml", "[sources");

    let report =
        super::run_deny_for_tests(&super::LocalFsTest, &root).expect("deny runtime report");

    assertions::assert_result_present(
        &report,
        "deny",
        "RS-DENY-01",
        Some("apps/backend/deny.toml"),
        Some(false),
        Some("deny config parse error"),
    );
    assertions::assert_result_present(
        &report,
        "deny",
        "RS-DENY-01",
        Some("apps/other/deny.toml"),
        Some(false),
        Some("deny config parse error"),
    );
    assert_eq!(
        section(&report, "deny")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-DENY-01"
                    && !result.inventory()
                    && matches!(
                        result.file(),
                        Some("apps/backend/deny.toml" | "apps/other/deny.toml")
                    )
            })
            .count(),
        2,
        "expected exactly one deny parse failure per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn garde_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("garde-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/Cargo.toml",
        "[package]\nname = \"backend-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ngarde = { workspace = true }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/src/lib.rs",
        "use garde::Validate;\n#[derive(Debug, Validate)]\npub struct BackendInput { #[garde(length(min = 1))] pub name: String }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/Cargo.toml",
        "[package]\nname = \"other-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/src/lib.rs",
        "use garde::Validate;\n#[derive(Debug, Validate)]\npub struct OtherInput { #[garde(length(min = 1))] pub name: String }\n",
    );

    let report =
        super::run_garde_for_tests(&super::LocalFsTest, &root).expect("garde runtime report");

    assertions::assert_result_present(
        &report,
        "garde",
        "RS-GARDE-01",
        Some("apps/backend/Cargo.toml"),
        Some(true),
        Some("garde dependency found"),
    );
    assertions::assert_result_present(
        &report,
        "garde",
        "RS-GARDE-01",
        Some("apps/other/Cargo.toml"),
        Some(false),
        Some("garde dependency missing"),
    );
    assert_eq!(
        section(&report, "garde")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-GARDE-01"
                    && matches!(
                        result.file(),
                        Some("apps/backend/Cargo.toml" | "apps/other/Cargo.toml")
                    )
            })
            .count(),
        2,
        "expected one garde root-coverage result per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn cargo_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("cargo-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");

    let report =
        super::run_cargo_for_tests(&super::LocalFsTest, &root).expect("cargo runtime report");

    assertions::assert_result_present(
        &report,
        "cargo",
        "RS-CARGO-01",
        Some("apps/backend/Cargo.toml"),
        Some(false),
        Some("rust lint table missing"),
    );
    assertions::assert_result_present(
        &report,
        "cargo",
        "RS-CARGO-01",
        Some("apps/other/Cargo.toml"),
        Some(false),
        Some("rust lint table missing"),
    );
    assert_eq!(
        section(&report, "cargo")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-CARGO-01"
                    && result.title() == "rust lint table missing"
                    && !result.inventory()
                    && matches!(
                        result.file(),
                        Some("apps/backend/Cargo.toml" | "apps/other/Cargo.toml")
                    )
            })
            .count(),
        2,
        "expected one cargo root-lint failure per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn fmt_runtime_validation_scope_keeps_repo_global_surface() {
    let root = super::temp_root_for_tests("fmt-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/backend\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");

    let report = super::run_fmt_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("fmt runtime report");

    assertions::assert_result_present(
        &report,
        "fmt",
        "RS-FMT-01",
        Some(""),
        Some(false),
        Some("rustfmt config missing"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn clippy_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("clippy-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");

    let report = super::run_clippy_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("clippy runtime report");

    assertions::assert_result_present(
        &report,
        "clippy",
        "RS-CLIPPY-01",
        Some("apps/backend"),
        Some(false),
        Some("Rust unit uncovered by clippy.toml"),
    );
    assertions::assert_absent_file(&report, "clippy", "apps/other");
    assert_eq!(
        section(&report, "clippy")
            .results()
            .iter()
            .filter(|result| !result.inventory())
            .filter_map(|result| result.file().map(str::to_owned))
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from(["apps/backend".to_owned()]),
        "expected scoped clippy run to stay inside one workspace surface: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deny_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("deny-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(&root, "apps/backend/deny.toml", "[bans");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");
    super::write_file_for_tests(&root, "apps/other/deny.toml", "[sources");

    let report = super::run_deny_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("deny runtime report");

    assertions::assert_result_present(
        &report,
        "deny",
        "RS-DENY-01",
        Some("apps/backend/deny.toml"),
        Some(false),
        Some("deny config parse error"),
    );
    assertions::assert_absent_file(&report, "deny", "apps/other/deny.toml");
    assert_eq!(
        section(&report, "deny")
            .results()
            .iter()
            .filter(|result| !result.inventory())
            .filter_map(|result| result.file().map(str::to_owned))
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from(["apps/backend/deny.toml".to_owned()]),
        "expected scoped deny run to stay inside one workspace surface: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn cargo_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("cargo-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");

    let report = super::run_cargo_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("cargo runtime report");

    assertions::assert_result_present(
        &report,
        "cargo",
        "RS-CARGO-01",
        Some("apps/backend/Cargo.toml"),
        Some(false),
        None,
    );
    assertions::assert_absent_file(&report, "cargo", "apps/other/Cargo.toml");
    assert_eq!(
        section(&report, "cargo")
            .results()
            .iter()
            .filter(|result| !result.inventory())
            .filter_map(|result| result.file().map(str::to_owned))
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from(["apps/backend/Cargo.toml".to_owned()]),
        "expected scoped cargo run to stay inside one workspace surface: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn garde_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("garde-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/Cargo.toml",
        "[package]\nname = \"backend-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/src/lib.rs",
        "use garde::Validate;\n#[derive(Debug, Validate)]\npub struct BackendInput { #[garde(length(min = 1))] pub name: String }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/Cargo.toml",
        "[package]\nname = \"other-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/src/lib.rs",
        "use garde::Validate;\n#[derive(Debug, Validate)]\npub struct OtherInput { #[garde(length(min = 1))] pub name: String }\n",
    );

    let report = super::run_garde_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/crates/api/src",
    )
    .expect("garde runtime report");

    assertions::assert_result_present(
        &report,
        "garde",
        "RS-GARDE-01",
        Some("apps/backend/Cargo.toml"),
        Some(false),
        Some("garde dependency missing"),
    );
    assert!(
        !section(&report, "garde")
            .results()
            .iter()
            .filter_map(|result| result.file())
            .any(|file| file.starts_with("apps/other")),
        "scoped garde run leaked into sibling workspace: {report:#?}"
    );

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
        "[package]\nname = \"api\"\n\n[dependencies]\ntokio = \"1\"\n",
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

    assertions::assert_result_present(
        &report,
        "deps",
        "RS-DEPS-09",
        Some("apps/backend/Cargo.lock"),
        Some(true),
        Some("Cargo.lock committed"),
    );
    assert!(
        !section(&report, "deps")
            .results()
            .iter()
            .any(|result| result.file() == Some("apps/backend/crates/other/Cargo.toml")),
        "sibling workspace member leaked into subtree deps run: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn deps_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("deps-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/backend/Cargo.lock", "");
    super::write_file_for_tests(&root, "apps/backend/src/lib.rs", "pub fn backend() {}\n");
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(&root, "apps/other/Cargo.lock", "");
    super::write_file_for_tests(&root, "apps/other/src/lib.rs", "pub fn other() {}\n");

    let report =
        super::run_deps_for_tests(&super::LocalFsTest, &root).expect("deps runtime report");

    assertions::assert_result_present(
        &report,
        "deps",
        "RS-DEPS-09",
        Some("apps/backend/Cargo.lock"),
        Some(true),
        Some("Cargo.lock committed"),
    );
    assertions::assert_result_present(
        &report,
        "deps",
        "RS-DEPS-09",
        Some("apps/other/Cargo.lock"),
        Some(true),
        Some("Cargo.lock committed"),
    );
    assert_eq!(
        section(&report, "deps")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-DEPS-09"
                    && result.inventory()
                    && matches!(
                        result.file(),
                        Some("apps/backend/Cargo.lock" | "apps/other/Cargo.lock")
                    )
            })
            .count(),
        2,
        "expected one deps lockfile result per legal workspace: {report:#?}"
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

    assertions::assert_absent_file(&report, "release", "apps/backend/crates/other/Cargo.toml");
    assertions::assert_result_present(
        &report,
        "release",
        "RS-PUB-02",
        Some("apps/backend/crates/api/Cargo.toml"),
        Some(false),
        Some("api: missing license"),
    );
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
fn release_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("release-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/README.md",
        &format!("# Backend\n\n{}\n", "x".repeat(240)),
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/src/lib.rs",
        "pub fn backend() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/crates/api/Cargo.toml",
        r#"
            [package]
            name = "backend"
            version = "0.1.0"
            description = "backend crate"
            repository = "https://example.com/backend"
            readme = "README.md"
            keywords = ["backend"]
            categories = ["development-tools"]
        "#,
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/README.md",
        &format!("# Other\n\n{}\n", "x".repeat(240)),
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/src/lib.rs",
        "pub fn other() {}\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/crates/api/Cargo.toml",
        r#"
            [package]
            name = "other"
            version = "0.1.0"
            description = "other crate"
            repository = "https://example.com/other"
            readme = "README.md"
            keywords = ["other"]
            categories = ["development-tools"]
        "#,
    );

    let report =
        super::run_release_for_tests(&super::LocalFsTest, &root).expect("release runtime report");

    assertions::assert_result_present(
        &report,
        "release",
        "RS-PUB-02",
        Some("apps/backend/crates/api/Cargo.toml"),
        Some(false),
        Some("backend: missing license"),
    );
    assertions::assert_result_present(
        &report,
        "release",
        "RS-PUB-02",
        Some("apps/other/crates/api/Cargo.toml"),
        Some(false),
        Some("other: missing license"),
    );
    assert_eq!(
        section(&report, "release")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-PUB-02"
                    && !result.inventory()
                    && matches!(
                        result.file(),
                        Some(
                            "apps/backend/crates/api/Cargo.toml"
                                | "apps/other/crates/api/Cargo.toml"
                        )
                    )
            })
            .count(),
        2,
        "expected one release crate-policy result per legal workspace: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn libarch_runtime_runs_each_legal_workspace_once() {
    let root = super::temp_root_for_tests("libarch-runtime-multi-workspace");
    super::write_file_for_tests(
        &root,
        "packages/shared/Cargo.toml",
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\"]\nresolver = \"2\"\n\n[workspace.dependencies]\nshared-api = { path = \"crates/api\" }\nshared-core = { path = \"crates/core\" }\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/src/lib.rs",
        "pub use shared_api::*;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/api/Cargo.toml",
        "[package]\nname = \"shared-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/api/src/lib.rs",
        "pub struct Api;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/core/Cargo.toml",
        "[package]\nname = \"shared-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/core/src/lib.rs",
        "pub struct Core;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/Cargo.toml",
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/src/lib.rs",
        "pub use other_api::*;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/api/Cargo.toml",
        "[package]\nname = \"other-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/api/src/lib.rs",
        "pub struct Api;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/core/Cargo.toml",
        "[package]\nname = \"other-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nother-api = { path = \"../api\" }\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/core/src/lib.rs",
        "pub struct Core;\n",
    );

    let report =
        super::run_libarch_for_tests(&super::LocalFsTest, &root).expect("libarch runtime report");

    assertions::assert_result_present(
        &report,
        "libarch",
        "RS-LIBARCH-07",
        Some("packages/shared/crates/core/Cargo.toml"),
        Some(true),
        Some("core does not depend on api"),
    );
    assertions::assert_result_present(
        &report,
        "libarch",
        "RS-LIBARCH-07",
        Some("packages/other/crates/core/Cargo.toml"),
        Some(false),
        Some("core must not depend on api"),
    );
    assert_eq!(
        section(&report, "libarch")
            .results()
            .iter()
            .filter(|result| {
                result.id() == "RS-LIBARCH-07"
                    && matches!(
                        result.file(),
                        Some(
                            "packages/shared/crates/core/Cargo.toml"
                                | "packages/other/crates/core/Cargo.toml"
                        )
                    )
            })
            .count(),
        2,
        "expected one libarch core-direction result per legal package root: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn libarch_runtime_validation_scope_stays_inside_owning_workspace() {
    let root = super::temp_root_for_tests("libarch-runtime-validation-scope");
    super::write_file_for_tests(
        &root,
        "packages/shared/Cargo.toml",
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/src/lib.rs",
        "pub use shared_api::*;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/api/Cargo.toml",
        "[package]\nname = \"shared-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/api/src/lib.rs",
        "pub struct Api;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/core/Cargo.toml",
        "[package]\nname = \"shared-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/shared/crates/core/src/lib.rs",
        "pub struct Core;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/Cargo.toml",
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/src/lib.rs",
        "pub use other_api::*;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/api/Cargo.toml",
        "[package]\nname = \"other-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/api/src/lib.rs",
        "pub struct Api;\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/core/Cargo.toml",
        "[package]\nname = \"other-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nother-api = { path = \"../api\" }\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/other/crates/core/src/lib.rs",
        "pub struct Core;\n",
    );

    let report = super::run_libarch_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "packages/shared/src",
    )
    .expect("libarch runtime report");

    assertions::assert_result_present(
        &report,
        "libarch",
        "RS-LIBARCH-07",
        Some("packages/shared/crates/core/Cargo.toml"),
        Some(true),
        None,
    );
    assert!(
        !section(&report, "libarch")
            .results()
            .iter()
            .filter_map(|result| result.file())
            .any(|file| file.starts_with("packages/other")),
        "scoped libarch run leaked into sibling package root: {report:#?}"
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn libarch_runtime_stays_decoupled_from_topology_exactness() {
    let root = super::temp_root_for_tests("libarch-runtime-decoupled-topology-exactness");
    super::write_file_for_tests(
        &root,
        "guardrail3.toml",
        "[rust.checks]\ntopology = true\nhexarch = true\nlibarch = true\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/reason-policy/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\nresolver = \"2\"\n",
    );
    super::write_file_for_tests(
        &root,
        "packages/reason-policy/crates/api/Cargo.toml",
        "[package]\nname = \"reason-policy-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let report =
        super::run_libarch_for_tests(&super::LocalFsTest, &root).expect("libarch runtime report");

    assertions::assert_section_absent(&report, "topology");
    assertions::assert_ids_absent(
        &report,
        &[
            "RS-TOPOLOGY-12",
            "RS-HEXARCH-07",
            "RS-HEXARCH-09",
            "RS-LIBARCH-05",
            "RS-LIBARCH-06",
        ],
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn test_runtime_validation_scope_does_not_narrow_global_family() {
    let root = super::temp_root_for_tests("test-runtime-global-validation-scope");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/src/lib.rs",
        "#[cfg(test)]\nmod tests { #[test] fn backend() {} }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/Cargo.toml",
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/other/src/lib.rs",
        "#[cfg(test)]\nmod tests { #[test] fn other() {} }\n",
    );

    let report = super::run_test_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src",
    )
    .expect("test runtime report");

    assertions::assert_result_present(
        &report,
        "test",
        "RS-TEST-01",
        Some("apps/backend/src/lib.rs"),
        Some(false),
        Some("inline cfg(test) body in src"),
    );
    assertions::assert_result_present(
        &report,
        "test",
        "RS-TEST-01",
        Some("apps/other/src/lib.rs"),
        Some(false),
        Some("inline cfg(test) body in src"),
    );

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}

#[test]
fn test_runtime_validation_scope_keeps_workspace_root_policy_files() {
    let root = super::temp_root_for_tests("test-runtime-root-policy-surface");
    super::write_file_for_tests(
        &root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ntokio = \"1\"\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/src/lib.rs",
        "#[cfg(test)]\nmod tests { #[tokio::test] async fn backend() {} }\n",
    );
    super::write_file_for_tests(
        &root,
        "apps/backend/.config/nextest.toml",
        "[profile.default]\nslow-timeout = \"60s\"\n",
    );

    let report = super::run_test_with_validation_scope_for_tests(
        &super::LocalFsTest,
        &root,
        "apps/backend/src/lib.rs",
    )
    .expect("test runtime report");

    assertions::assert_result_present(
        &report,
        "test",
        "RS-TEST-09",
        Some("apps/backend/.config/nextest.toml"),
        Some(false),
        Some("nextest timeouts incomplete"),
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

    let live_results = section(&report, "deps")
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

    assertions::assert_clean_section(&report, "deps");

    std::fs::remove_dir_all(&root).expect("cleanup temp root");
}
