use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_code_ingestion_types::G3RsCodeSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use tempfile::tempdir;

const HAS_TODO: &str =
    include_str!("../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/has_todo.rs");
const DIRECT_STD_FS: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/direct_std_fs.rs"
);
const CLEAN_FILE: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/clean_file.rs"
);
const COMMENT_USE_STD_FS: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/grep-attacks/rust-structural/comment_use_std_fs.rs"
);
const STRING_TODO: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/grep-attacks/rust-code-quality/string_todo.rs"
);

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    flatten_results(&inputs)
}

fn flatten_results(inputs: &[G3RsCodeSourceChecksInput]) -> Vec<G3CheckResult> {
    inputs
        .iter()
        .flat_map(g3rs_code_source_checks::check)
        .collect::<Vec<_>>()
}

fn run_config_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("config ingestion should succeed");
    g3rs_code_config_checks::check(&input)
}

fn findings_by_file(results: &[G3CheckResult]) -> BTreeMap<String, Vec<&G3CheckResult>> {
    let mut by_file = BTreeMap::<String, Vec<&G3CheckResult>>::new();
    for result in results {
        let key = result.file().unwrap_or("<none>").to_owned();
        by_file.entry(key).or_default().push(result);
    }
    by_file
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
    let by_file = findings_by_file(&results);

    assert!(
        by_file["src/has_todo.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-13"),
        "todo fixture should trigger RS-CODE-SOURCE-13: {results:#?}"
    );
    assert_eq!(
        by_file["src/has_todo.rs"].len(),
        1,
        "todo fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        by_file["src/direct_std_fs.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-15"),
        "direct std::fs fixture should trigger RS-CODE-SOURCE-15: {results:#?}"
    );
    assert_eq!(
        by_file["src/direct_std_fs.rs"].len(),
        1,
        "direct std::fs fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        by_file["src/panic_probe.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-16"),
        "panic fixture should trigger RS-CODE-SOURCE-16: {results:#?}"
    );
    assert_eq!(
        by_file["src/panic_probe.rs"].len(),
        1,
        "panic fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        !by_file.contains_key("src/clean_file.rs"),
        "clean source should not produce findings: {results:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-CODE-CONFIG-07"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-CODE-CONFIG-12" && result.severity() == G3Severity::Info),
        "{results:#?}"
    );
    assert!(
        by_file["deny.toml"]
            .iter()
            .any(|result| result.id() == "RS-CODE-CONFIG-07"),
        "{results:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(by_file["crates/core/Cargo.toml"].len(), 1, "{results:#?}");
    assert_eq!(by_file["crates/core/Cargo.toml"][0].id(), "RS-CODE-CONFIG-07");
    assert!(
        !by_file.contains_key("Cargo.toml"),
        "root workspace warn plus harmless comments should stay clean: {results:#?}"
    );
    assert!(
        !by_file.contains_key("deny.toml"),
        "harmless deny comments should stay clean: {results:#?}"
    );
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
    write(
        root.join("deny.toml"),
        "# EXCEPTION: three\n",
    );

    let results = run_config_pipeline(root);
    let by_file = findings_by_file(&results);

    assert_eq!(results.len(), 3, "{results:#?}");
    assert_eq!(by_file["Cargo.toml"].len(), 2, "{results:#?}");
    assert_eq!(by_file["deny.toml"].len(), 1, "{results:#?}");
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
    let by_file = findings_by_file(&results);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(by_file["Cargo.toml"].len(), 1, "{results:#?}");
    assert_eq!(by_file["Cargo.toml"][0].id(), "RS-CODE-CONFIG-12");
    assert!(
        !by_file.contains_key("vendor/foreign/Cargo.toml"),
        "{results:#?}"
    );
    assert!(
        !by_file.contains_key("vendor/foreign/deny.toml"),
        "{results:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(by_file["Cargo.toml"].len(), 1, "{results:#?}");
    assert_eq!(by_file["Cargo.toml"][0].id(), "RS-CODE-CONFIG-12");
    assert_eq!(by_file["Cargo.toml"][0].severity(), G3Severity::Error);
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_config_checks(&crawl)
        .expect_err("malformed owned root cargo should fail config ingestion");

    assert!(
        matches!(error, crate::IngestionError::ParseFailed { .. }),
        "unexpected error: {error:?}"
    );
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_config_checks(&crawl)
        .expect_err("unreadable owned config should fail config ingestion");

    assert!(
        matches!(error, crate::IngestionError::Unreadable { .. }),
        "unexpected error: {error:?}"
    );
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
        "#[cfg(test)]\n#[path = \"rs_code_ast_24_path_attr_with_reason_tests/mod.rs\"]\nmod rs_code_ast_24_path_attr_with_reason_tests;\n",
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
    let by_file = findings_by_file(&results);

    assert_eq!(by_file["src/crate_allow.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/crate_allow.rs"][0].id(), "RS-CODE-SOURCE-01");

    assert_eq!(by_file["src/unused_crate_deps.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/unused_crate_deps.rs"][0].id(), "RS-CODE-SOURCE-02");

    assert_eq!(
        by_file["src/item_allow_missing_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/item_allow_missing_reason.rs"][0].id(),
        "RS-CODE-SOURCE-03"
    );

    assert_eq!(
        by_file["src/item_allow_with_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/item_allow_with_reason.rs"][0].id(),
        "RS-CODE-SOURCE-04"
    );

    assert_eq!(by_file["src/garde_skip.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/garde_skip.rs"][0].id(), "RS-CODE-SOURCE-06");

    assert_eq!(
        by_file["src/garde_skip_no_comment.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/garde_skip_no_comment.rs"][0].id(),
        "RS-CODE-SOURCE-05"
    );

    assert_eq!(by_file["src/too_many_lines.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/too_many_lines.rs"][0].id(), "RS-CODE-SOURCE-09");
    assert!(
        !by_file.contains_key("src/raw_string_payload_only.rs"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/too_many_uses.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/too_many_uses.rs"][0].id(), "RS-CODE-SOURCE-10");
    assert_eq!(
        by_file["src/use_error_boundary_clean.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/use_error_boundary_clean.rs"][0].id(),
        "RS-CODE-SOURCE-11"
    );

    assert_eq!(by_file["src/many_uses.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/many_uses.rs"][0].id(), "RS-CODE-SOURCE-11");
    assert!(
        !by_file.contains_key("src/use_warn_boundary_clean.rs"),
        "{results:#?}"
    );
    assert!(!by_file.contains_key("tests/use_exempt.rs"), "{results:#?}");

    assert_eq!(by_file["src/large_struct.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/large_struct.rs"][0].id(), "RS-CODE-SOURCE-19");
    assert_eq!(by_file["src/large_enum.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/large_enum.rs"][0].id(), "RS-CODE-SOURCE-19");

    assert_eq!(by_file["src/path_reason.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/path_reason.rs"][0].id(), "RS-CODE-SOURCE-24");
    assert_eq!(
        by_file["src/path_reason.rs"][0].title(),
        "#[path] with reason",
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/path_missing_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/path_missing_reason.rs"][0].id(), "RS-CODE-SOURCE-24");
    assert_eq!(
        by_file["src/path_missing_reason.rs"][0].title(),
        "#[path] without reason",
        "{results:#?}"
    );
    assert_eq!(by_file["src/path_weak_reason.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/path_weak_reason.rs"][0].id(), "RS-CODE-SOURCE-24");
    assert_eq!(
        by_file["src/path_weak_reason.rs"][0].title(),
        "#[path] reason too weak",
        "{results:#?}"
    );
    assert_eq!(by_file["src/path_escape.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/path_escape.rs"][0].id(), "RS-CODE-SOURCE-24");
    assert_eq!(
        by_file["src/path_escape.rs"][0].title(),
        "#[path] escapes parent directory",
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/path_cfg_attr_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/path_cfg_attr_reason.rs"][0].id(), "RS-CODE-SOURCE-24");
    assert!(
        !by_file.contains_key("src/path_cfg_attr_known_false.rs"),
        "{results:#?}"
    );
    assert!(
        !by_file.contains_key("src/path_sidecar_exempt.rs"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/cfg_attr_unknown.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/cfg_attr_unknown.rs"][0].id(), "RS-CODE-SOURCE-08");

    assert_eq!(
        by_file["src/deny_without_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/deny_without_reason.rs"][0].id(), "RS-CODE-SOURCE-22");

    assert_eq!(by_file["src/impl_allow.rs"].len(), 2, "{results:#?}");
    assert!(
        by_file["src/impl_allow.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-03"),
        "{results:#?}"
    );
    assert!(
        by_file["src/impl_allow.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-17"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/cfg_attr.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/cfg_attr.rs"][0].id(), "RS-CODE-SOURCE-18");

    assert_eq!(by_file["src/ffi.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/ffi.rs"][0].id(), "RS-CODE-SOURCE-20");

    assert_eq!(by_file["src/fs_glob.rs"].len(), 2, "{results:#?}");
    assert!(
        by_file["src/fs_glob.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-15"),
        "{results:#?}"
    );
    assert!(
        by_file["src/fs_glob.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-21"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/include_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/include_probe.rs"][0].id(), "RS-CODE-SOURCE-23");

    assert_eq!(by_file["src/forbid_inventory.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/forbid_inventory.rs"][0].id(), "RS-CODE-SOURCE-22");
    assert!(
        by_file["src/forbid_inventory.rs"][0].inventory(),
        "{results:#?}"
    );

    assert_eq!(by_file["tests/expect_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["tests/expect_probe.rs"][0].id(), "RS-CODE-SOURCE-32");

    assert_eq!(by_file["src/generic_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/generic_probe.rs"][0].id(), "RS-CODE-SOURCE-34");

    assert_eq!(by_file["src/large_trait.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/large_trait.rs"][0].id(), "RS-CODE-SOURCE-29");
    assert_eq!(
        by_file["src/large_trait.rs"][0].severity(),
        G3Severity::Warn
    );
    assert_eq!(
        by_file["src/large_trait_boundary.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/large_trait_boundary.rs"][0].id(), "RS-CODE-SOURCE-29");
    assert_eq!(
        by_file["src/large_trait_boundary.rs"][0].severity(),
        G3Severity::Warn
    );
    assert!(!by_file.contains_key("src/small_trait.rs"), "{results:#?}");

    assert_eq!(by_file["src/public_field_bag.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/public_field_bag.rs"][0].id(), "RS-CODE-SOURCE-31");
    assert_eq!(
        by_file["src/public_field_bag.rs"][0].severity(),
        G3Severity::Warn
    );
    assert_eq!(
        by_file["src/public_field_warn_boundary.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/public_field_warn_boundary.rs"][0].id(),
        "RS-CODE-SOURCE-31"
    );
    assert_eq!(
        by_file["src/public_field_warn_boundary.rs"][0].severity(),
        G3Severity::Warn
    );
    assert_eq!(
        by_file["src/public_field_error_boundary.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/public_field_error_boundary.rs"][0].id(),
        "RS-CODE-SOURCE-31"
    );
    assert_eq!(
        by_file["src/public_field_error_boundary.rs"][0].severity(),
        G3Severity::Error
    );
    assert!(
        !by_file.contains_key("src/private_field_struct.rs"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/public_weak_error.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/public_weak_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert_eq!(
        by_file["src/public_trait_weak_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/public_trait_weak_error.rs"][0].id(),
        "RS-CODE-SOURCE-33"
    );
    assert_eq!(
        by_file["src/public_impl_weak_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/public_impl_weak_error.rs"][0].id(),
        "RS-CODE-SOURCE-33"
    );
    assert_eq!(
        by_file["src/public_str_ref_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/public_str_ref_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert!(
        !by_file.contains_key("src/typed_public_error.rs"),
        "{results:#?}"
    );
    assert!(
        !by_file.contains_key("src/private_weak_error.rs"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/string_dispatch.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/string_dispatch.rs"][0].id(), "RS-CODE-SOURCE-36");
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
    let by_file = findings_by_file(&results);

    assert!(
        !by_file.contains_key("src/line_cap.rs"),
        "exactly 500 effective lines should stay clean: {results:#?}"
    );
    assert_eq!(by_file["src/line_over_cap.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/line_over_cap.rs"][0].id(), "RS-CODE-SOURCE-09");
    assert!(
        !by_file.contains_key("src/string_dispatch_clean.rs"),
        "exactly 10 string branches should stay clean: {results:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert!(
        !by_file.contains_key("src/trait_clean.rs"),
        "8-method trait should stay clean: {results:#?}"
    );
    assert_eq!(by_file["src/trait_warn.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/trait_warn.rs"][0].id(), "RS-CODE-SOURCE-29");
    assert_eq!(by_file["src/trait_error.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/trait_error.rs"][0].id(), "RS-CODE-SOURCE-29");

    assert_eq!(
        by_file["src/public_string_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/public_string_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert_eq!(by_file["src/public_str_error.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/public_str_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert_eq!(
        by_file["src/public_anyhow_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/public_anyhow_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert_eq!(by_file["src/public_box_error.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/public_box_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert!(
        !by_file.contains_key("src/private_string_error.rs"),
        "private weak error helper should stay clean: {results:#?}"
    );
    assert_eq!(
        by_file["src/public_trait_error.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/public_trait_error.rs"][0].id(), "RS-CODE-SOURCE-33");
    assert_eq!(by_file["src/public_impl_error.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/public_impl_error.rs"][0].id(), "RS-CODE-SOURCE-33");
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
    let by_file = findings_by_file(&results);

    assert_eq!(
        by_file["src/include_str_escape.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/include_str_escape.rs"][0].id(), "RS-CODE-SOURCE-23");
    assert_eq!(
        by_file["src/include_str_escape.rs"][0].title(),
        "include path traversal",
        "{results:#?}"
    );
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

    assert_eq!(
        results.len(),
        2,
        "only todo! should currently fire in test-owned files: {results:#?}"
    );
    assert!(
        results.iter().all(|result| result.id() == "RS-CODE-SOURCE-13"),
        "test-owned files should currently suppress RS-CODE-SOURCE-15 and RS-CODE-SOURCE-16 only: {results:#?}"
    );
}

#[test]
fn pipeline_emits_explicit_input_failure_for_parse_error() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/broken.rs"), "fn broken( {");

    let results = run_pipeline(root);

    assert_eq!(
        results.len(),
        1,
        "broken source should emit one input failure"
    );
    let result = &results[0];
    assert_eq!(result.id(), "RS-CODE-SOURCE-30");
    assert_eq!(result.title(), "code-family input failure");
    assert_eq!(result.file(), Some("src/broken.rs"));
    assert!(
        result
            .message()
            .starts_with("Failed to parse Rust source file:"),
        "unexpected message: {result:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert!(
        by_file["src/broken.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-30"),
        "broken file should still emit parse failure: {results:#?}"
    );
    assert!(
        by_file["src/has_todo.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-13"),
        "valid file should still emit its finding: {results:#?}"
    );
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
    let by_file = findings_by_file(&results);

    assert_eq!(by_file["lib/api.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["lib/api.rs"][0].id(), "RS-CODE-SOURCE-29");
    assert_eq!(by_file["lib/api.rs"][0].severity(), G3Severity::Warn);
    assert_eq!(by_file["cmd/worker.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["cmd/worker.rs"][0].id(), "RS-CODE-SOURCE-29");
    assert_eq!(by_file["cmd/worker.rs"][0].severity(), G3Severity::Error);
}
