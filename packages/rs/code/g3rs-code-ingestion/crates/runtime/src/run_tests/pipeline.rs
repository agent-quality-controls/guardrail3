#![expect(
    clippy::disallowed_methods,
    clippy::too_many_lines,
    clippy::format_collect,
    reason = "test fixtures need direct filesystem and process access to build temp workspaces; long pipeline assertions enumerate every emitted finding for a given fixture; format!-from-iterator is the simplest fixture-content builder for synthetic large source files"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_code_ingestion_assertions::run::{
    assert_config_pipeline_ignores_foreign_nested_repo_findings,
    assert_config_pipeline_reports_deny_through_full_lane,
    assert_config_pipeline_reports_exact_exception_comment_counts,
    assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints,
    assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests,
    assert_pipeline_classifies_custom_target_paths_before_checks_run,
    assert_pipeline_emits_explicit_input_failure_for_parse_error,
    assert_pipeline_keeps_other_findings_when_one_file_fails_to_parse,
    assert_pipeline_preserves_current_test_owned_rule_behavior,
    assert_pipeline_reports_effective_line_and_dispatch_boundaries,
    assert_pipeline_reports_expected_findings_on_real_source_files,
    assert_pipeline_reports_include_str_traversal,
    assert_pipeline_reports_new_single_file_ast_rules,
    assert_pipeline_reports_trait_and_public_error_boundaries,
};
use g3rs_code_ingestion_assertions::run::{
    assert_single_parse_failed_error, assert_single_unreadable_error,
};
use g3rs_code_types::G3RsCodeSourceChecksInput;
use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

const HAS_TODO: &str = "pub fn run() {\n    todo!(\"finish this\");\n}\n";
const DIRECT_STD_FS: &str =
    "pub fn load() {\n    let _ = std::fs::read_to_string(\"/tmp/demo\");\n}\n";
const CLEAN_FILE: &str = "pub fn clean() {\n    let value = 1 + 1;\n    let _ = value;\n}\n";
const COMMENT_USE_STD_FS: &str = "// use std::fs::read_to_string in docs only\npub fn clean() {}\n";
const STRING_TODO: &str = "pub fn clean() {\n    let _ = \"TODO: literal only\";\n}\n";

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
    fs::write(path.join("Cargo.toml"), "[workspace]\nmembers = []\n")
        .expect("write default workspace manifest");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::run::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    flatten_results(&inputs)
}

fn flatten_results(inputs: &[G3RsCodeSourceChecksInput]) -> Vec<G3CheckResult> {
    inputs
        .iter()
        .flat_map(g3rs_code_source_checks::check)
        .collect::<Vec<_>>()
}

fn run_config_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_config_checks(&crawl).expect("config ingestion should succeed");
    g3rs_code_config_checks::check(&input)
}

#[test]
fn pipeline_reports_expected_findings_on_real_source_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/has_todo.rs"), HAS_TODO);
    write(root.join("src/direct_std_fs.rs"), DIRECT_STD_FS);
    write(
        root.join("src/panic_probe.rs"),
        "pub fn run() { panic!(\"boom\"); }\n",
    );
    write(root.join("src/clean_file.rs"), CLEAN_FILE);

    let results = run_pipeline(root);
    assert_pipeline_reports_expected_findings_on_real_source_files(&results);
}

#[test]
fn config_pipeline_reports_exception_comments_and_unsafe_code_lints() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/core\"]\n\
\n\
[workspace.lints.rust]\n\
unsafe_code = \"forbid\"\n\
\n\
# EXCEPTION: temporary workspace suppression\n",
    );
    write(
        root.join("deny.toml"),
        "advisories = { ignore = [] }\n# EXCEPTION: temporary advisory hold\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let results = run_config_pipeline(root);
    assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints(&results);
}

#[test]
fn config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/core\"]\n\
\n\
[workspace.lints.rust]\n\
unsafe_code = \"warn\"\n\
\n\
quoted = \"# EXCEPTION: not real\"\n\
# note: harmless\n",
    );
    write(
        root.join("deny.toml"),
        "value = \"// EXCEPTION: still not real\"\n# temporary note only\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "\
[package]\n\
name = \"core\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
# EXCEPTION: package inventory still counts\n",
    );

    let results = run_config_pipeline(root);
    assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests(&results);
}

#[test]
fn config_pipeline_reports_exact_exception_comment_counts() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"demo\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
# EXCEPTION: one\n\
# EXCEPTION: two\n",
    );
    write(root.join("deny.toml"), "# EXCEPTION: three\n");

    let results = run_config_pipeline(root);
    assert_config_pipeline_reports_exact_exception_comment_counts(&results);
}

#[test]
fn config_pipeline_ignores_foreign_nested_repo_findings() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/core\"]\n\
\n\
[workspace.lints.rust]\n\
unsafe_code = \"deny\"\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("vendor/foreign/Cargo.toml"),
        "\
[workspace]\n\
members = []\n\
\n\
[workspace.lints.rust]\n\
unsafe_code = \"deny\"\n\
\n\
# EXCEPTION: foreign workspace suppression\n",
    );
    write(
        root.join("vendor/foreign/deny.toml"),
        "# EXCEPTION: foreign deny suppression\n",
    );

    let results = run_config_pipeline(root);
    assert_config_pipeline_ignores_foreign_nested_repo_findings(&results);
}

#[test]
fn config_pipeline_reports_deny_through_full_lane() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = []\n\
\n\
[workspace.lints.rust]\n\
unsafe_code = \"deny\"\n",
    );

    let results = run_config_pipeline(root);
    assert_config_pipeline_reports_deny_through_full_lane(&results);
}

#[test]
fn config_pipeline_stays_clean_for_root_manifest_without_workspace() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"demo\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n",
    );

    let results = run_config_pipeline(root);
    assert!(results.is_empty(), "unexpected results: {results:#?}");
}

#[test]
fn config_ingestion_fails_closed_for_malformed_owned_root_cargo() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace\nbroken = true");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::run::ingest_for_config_checks(&crawl)
        .expect_err("malformed owned root cargo should fail config ingestion");

    assert_single_parse_failed_error(&error);
}

#[cfg(unix)]
#[test]
fn config_ingestion_fails_closed_for_unreadable_owned_config_file() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let deny_toml = root.join("deny.toml");
    write(&deny_toml, "# EXCEPTION: hidden\n");

    let mut permissions = fs::metadata(&deny_toml)
        .expect("metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&deny_toml, permissions).expect("chmod should succeed");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::run::ingest_for_config_checks(&crawl)
        .expect_err("unreadable owned config should fail config ingestion");

    assert_single_unreadable_error(&error);
}

#[test]
fn pipeline_reports_new_single_file_ast_rules() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("src/crate_allow.rs"),
        "#![allow(dead_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/unused_crate_deps.rs"),
        "#![allow(unused_crate_dependencies)]\nfn probe() {}\n",
    );
    write(
        root.join("src/item_allow_missing_reason.rs"),
        "#[allow(clippy::too_many_lines)]\nfn probe() {}\n",
    );
    write(
        root.join("src/item_allow_with_reason.rs"),
        "#[allow(clippy::too_many_lines)] // reason: generated ffi shim\nfn probe() {}\n",
    );
    write(
        root.join("src/garde_skip.rs"),
        "struct Form {\n    #[garde(skip)] // reason: validated upstream boundary\n    token: String,\n}\n",
    );
    write(
        root.join("src/garde_skip_no_comment.rs"),
        "struct Form {\n    #[garde(skip)]\n    token: String,\n}\n",
    );
    write(
        root.join("src/too_many_lines.rs"),
        &(0..501)
            .map(|i| format!("fn f{i}() {{}}\n"))
            .collect::<String>(),
    );
    write(
        root.join("src/raw_string_payload_only.rs"),
        &format!(
            "const BIG: &str = r#\"\n{}\"#;\n",
            (0..600)
                .map(|i| format!("payload-{i}\n"))
                .collect::<String>()
        ),
    );
    write(
        root.join("src/too_many_uses.rs"),
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19,b20};\nfn probe() {}\n",
    );
    write(
        root.join("src/use_error_boundary_clean.rs"),
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19};\nfn probe() {}\n",
    );
    write(
        root.join("src/many_uses.rs"),
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15};\nfn probe() {}\n",
    );
    write(
        root.join("src/use_warn_boundary_clean.rs"),
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14};\nfn probe() {}\n",
    );
    write(
        root.join("tests/use_exempt.rs"),
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19,b20};\n#[test]\nfn smoke() {}\n",
    );
    write(
        root.join("src/large_struct.rs"),
        "struct Big { f0: u8, f1: u8, f2: u8, f3: u8, f4: u8, f5: u8, f6: u8, f7: u8, f8: u8, f9: u8, f10: u8, f11: u8, f12: u8, f13: u8, f14: u8, f15: u8 }\n",
    );
    write(
        root.join("src/large_enum.rs"),
        "enum Big { V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20 }\n",
    );
    write(
        root.join("src/path_reason.rs"),
        "#[path = \"generated.rs\"] // reason: generated bridge shim\nmod generated;\n",
    );
    write(
        root.join("src/path_missing_reason.rs"),
        "#[path = \"generated.rs\"]\nmod generated;\n",
    );
    write(
        root.join("src/path_weak_reason.rs"),
        "#[path = \"generated.rs\"] // reason: temp\nmod generated;\n",
    );
    write(
        root.join("src/path_escape.rs"),
        "#[path = \"../generated.rs\"]\nmod generated;\n",
    );
    write(
        root.join("src/path_cfg_attr_reason.rs"),
        "#[cfg_attr(feature = \"cli\", path = \"generated.rs\")] // reason: generated bridge shim\nmod generated;\n",
    );
    write(
        root.join("src/path_cfg_attr_known_false.rs"),
        "#[cfg_attr(any(), path = \"generated.rs\")]\nmod generated;\n",
    );
    write(
        root.join("src/path_sidecar_exempt.rs"),
        "#[cfg(test)]\n#[path = \"path_sidecar_exempt_tests/mod.rs\"]\nmod path_sidecar_exempt_tests;\n",
    );
    write(
        root.join("src/cfg_attr_unknown.rs"),
        "#[cfg_attr(feature = \"cli\", allow(dead_code))]\nfn probe() {}\n",
    );
    write(
        root.join("src/deny_without_reason.rs"),
        "#[deny(dead_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/impl_allow.rs"),
        "struct Foo;\n#[allow(clippy::too_many_lines)]\nimpl Foo { fn a(&self) {} fn b(&self) {} fn c(&self) {} fn d(&self) {} }\n",
    );
    write(
        root.join("src/forbid_inventory.rs"),
        "#![forbid(unsafe_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/cfg_attr.rs"),
        "#[cfg_attr(all(), allow(dead_code))]\nfn probe() {}\n",
    );
    write(
        root.join("src/ffi.rs"),
        "#[allow(improper_ctypes)]\nunsafe extern \"C\" { fn puts(s: *const i8); }\n",
    );
    write(
        root.join("src/fs_glob.rs"),
        "use std::fs::*;\nfn probe() {}\n",
    );
    write(
        root.join("src/include_probe.rs"),
        "include!(\"../generated.rs\");\n",
    );
    write(
        root.join("tests/expect_probe.rs"),
        "fn probe() { let _ = Some(1).expect(\"ok\"); }\n",
    );
    write(
        root.join("src/generic_probe.rs"),
        "pub fn build<A, B, C, D, E, F, G>() {}\n",
    );
    write(
        root.join("src/large_trait.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n    fn m8(&self);\n}\n",
    );
    write(
        root.join("src/large_trait_boundary.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n    fn m8(&self);\n    fn m9(&self);\n    fn m10(&self);\n    fn m11(&self);\n}\n",
    );
    write(
        root.join("src/small_trait.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n}\n",
    );
    write(
        root.join("src/public_field_bag.rs"),
        "pub struct User { pub id: String, pub email: String }\n",
    );
    write(
        root.join("src/public_field_warn_boundary.rs"),
        "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8 }\n",
    );
    write(
        root.join("src/public_field_error_boundary.rs"),
        "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8, pub e: u8 }\n",
    );
    write(
        root.join("src/private_field_struct.rs"),
        "pub struct User { id: String, email: String }\n",
    );
    write(
        root.join("src/public_weak_error.rs"),
        "pub fn parse() -> Result<(), String> { Ok(()) }\n",
    );
    write(
        root.join("src/public_trait_weak_error.rs"),
        "pub trait Service { fn parse(&self) -> Result<(), anyhow::Error>; }\n",
    );
    write(
        root.join("src/public_impl_weak_error.rs"),
        "pub struct Gateway;\nimpl Gateway { pub fn boxed(&self) -> Result<(), Box<dyn std::error::Error>> { Ok(()) } }\n",
    );
    write(
        root.join("src/public_str_ref_error.rs"),
        "pub fn label() -> Result<(), &str> { Ok(()) }\n",
    );
    write(
        root.join("src/typed_public_error.rs"),
        "pub fn parse() -> Result<(), ParseError> { Ok(()) }\n",
    );
    write(
        root.join("src/private_weak_error.rs"),
        "fn parse() -> Result<(), String> { Ok(()) }\n",
    );
    write(
        root.join("src/string_dispatch.rs"),
        "pub fn dispatch(value: &str) -> usize { if value == \"v0\" { 0 } else if value == \"v1\" { 1 } else if value == \"v2\" { 2 } else if value == \"v3\" { 3 } else if value == \"v4\" { 4 } else if value == \"v5\" { 5 } else if value == \"v6\" { 6 } else if value == \"v7\" { 7 } else if value == \"v8\" { 8 } else if value == \"v9\" { 9 } else if value == \"v10\" { 10 } else { 0 } }\n",
    );

    let results = run_pipeline(root);
    assert_pipeline_reports_new_single_file_ast_rules(&results);
}

#[test]
fn pipeline_reports_effective_line_and_dispatch_boundaries() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("src/line_cap.rs"),
        &(0..500)
            .map(|i| format!("fn f{i}() {{}}\n"))
            .collect::<String>(),
    );
    write(
        root.join("src/line_over_cap.rs"),
        &(0..501)
            .map(|i| format!("fn g{i}() {{}}\n"))
            .collect::<String>(),
    );
    write(
        root.join("src/string_dispatch_clean.rs"),
        "pub fn dispatch(value: &str) -> usize { match value { \"v0\" => 0, \"v1\" => 1, \"v2\" => 2, \"v3\" => 3, \"v4\" => 4, \"v5\" => 5, \"v6\" => 6, \"v7\" => 7, \"v8\" => 8, \"v9\" => 9, _ => 0 } }",
    );

    let results = run_pipeline(root);
    assert_pipeline_reports_effective_line_and_dispatch_boundaries(&results);
}

#[test]
fn pipeline_reports_trait_and_public_error_boundaries() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("src/trait_clean.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n}\n",
    );
    write(
        root.join("src/trait_warn.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n    fn m8(&self);\n}\n",
    );
    write(
        root.join("src/trait_error.rs"),
        "pub trait Service {\n    fn m0(&self);\n    fn m1(&self);\n    fn m2(&self);\n    fn m3(&self);\n    fn m4(&self);\n    fn m5(&self);\n    fn m6(&self);\n    fn m7(&self);\n    fn m8(&self);\n    fn m9(&self);\n    fn m10(&self);\n    fn m11(&self);\n    fn m12(&self);\n}\n",
    );
    write(
        root.join("src/public_string_error.rs"),
        "pub fn parse() -> Result<(), String> { Ok(()) }\n",
    );
    write(
        root.join("src/public_str_error.rs"),
        "pub fn label() -> Result<(), &str> { Ok(()) }\n",
    );
    write(
        root.join("src/public_anyhow_error.rs"),
        "pub fn parse() -> Result<(), anyhow::Error> { Ok(()) }\n",
    );
    write(
        root.join("src/public_box_error.rs"),
        "pub fn parse() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }\n",
    );
    write(
        root.join("src/private_string_error.rs"),
        "fn parse() -> Result<(), String> { Ok(()) }\n",
    );
    write(
        root.join("src/public_trait_error.rs"),
        "pub trait Api {\n    fn parse(&self) -> Result<(), String>;\n    fn typed(&self) -> Result<(), ParseError>;\n}\n",
    );
    write(
        root.join("src/public_impl_error.rs"),
        "pub struct Api;\nimpl Api {\n    pub fn parse(&self) -> Result<(), String> { Ok(()) }\n    pub fn typed(&self) -> Result<(), ParseError> { Ok(()) }\n}\n",
    );

    let results = run_pipeline(root);
    assert_pipeline_reports_trait_and_public_error_boundaries(&results);
}

#[test]
fn pipeline_reports_include_str_traversal() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("src/include_str_escape.rs"),
        "const TEMPLATE: &str = include_str!(\"../templates/payload.txt\");\n",
    );

    let results = run_pipeline(root);
    assert_pipeline_reports_include_str_traversal(&results);
}

#[test]
fn pipeline_rejects_known_false_positive_fixture_patterns() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/comment_use_std_fs.rs"), COMMENT_USE_STD_FS);
    write(root.join("src/string_todo.rs"), STRING_TODO);

    let results = run_pipeline(root);

    assert!(
        results.is_empty(),
        "comment/string fixtures should stay clean under source pipeline: {results:#?}"
    );
}

#[test]
fn pipeline_preserves_current_test_owned_rule_behavior() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("tests/smoke.rs"),
        "#[test]\nfn smoke() { todo!(); panic!(\"boom\"); let _ = std::fs::read_to_string(\"f\"); }\n",
    );
    write(
        root.join("src/helpers_tests.rs"),
        "pub fn helper() { todo!(); panic!(\"boom\"); let _ = std::fs::read_to_string(\"f\"); }\n",
    );

    let results = run_pipeline(root);
    assert_pipeline_preserves_current_test_owned_rule_behavior(&results);
}

#[test]
fn pipeline_emits_explicit_input_failure_for_parse_error() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/broken.rs"), "fn broken( {");

    let results = run_pipeline(root);
    assert_pipeline_emits_explicit_input_failure_for_parse_error(&results);
}

#[test]
fn pipeline_stays_clean_on_small_workspace_baseline() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/clean_file.rs"), CLEAN_FILE);
    write(root.join("src/string_todo.rs"), STRING_TODO);
    write(root.join("src/comment_use_std_fs.rs"), COMMENT_USE_STD_FS);

    let results = run_pipeline(root);

    assert!(
        results.is_empty(),
        "clean baseline workspace should stay clean: {results:#?}"
    );
}

#[test]
fn pipeline_keeps_other_findings_when_one_file_fails_to_parse() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/broken.rs"), "fn broken( {");
    write(root.join("src/has_todo.rs"), HAS_TODO);

    let results = run_pipeline(root);
    assert_pipeline_keeps_other_findings_when_one_file_fails_to_parse(&results);
}

#[test]
fn pipeline_classifies_custom_target_paths_before_checks_run() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"demo\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
\n\
[lib]\n\
path = \"lib/api.rs\"\n\
\n\
[[bin]]\n\
name = \"worker\"\n\
path = \"cmd/worker.rs\"\n",
    );
    write(
        root.join("lib/api.rs"),
        "pub trait Service { fn m0(&self); fn m1(&self); fn m2(&self); fn m3(&self); fn m4(&self); fn m5(&self); fn m6(&self); fn m7(&self); fn m8(&self); }\n",
    );
    write(
        root.join("cmd/worker.rs"),
        "pub trait Service { fn m0(&self); fn m1(&self); fn m2(&self); fn m3(&self); fn m4(&self); fn m5(&self); fn m6(&self); fn m7(&self); fn m8(&self); fn m9(&self); fn m10(&self); fn m11(&self); fn m12(&self); }\n",
    );

    let results = run_pipeline(root);
    assert_pipeline_classifies_custom_target_paths_before_checks_run(&results);
}
