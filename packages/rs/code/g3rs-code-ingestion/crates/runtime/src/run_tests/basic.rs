#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures need direct filesystem and process access to build temp workspaces"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

use g3_workspace_crawl::crawl;
use g3rs_code_ingestion_assertions::run as assertions;
use tempfile::tempdir;

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

#[test]
fn ingests_owned_rust_files_and_classifies_tests() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn run() {}\n");
    write(root.join("src/http_tests.rs"), "pub fn helper() {}\n");
    write(root.join("tests/smoke.rs"), "#[test]\nfn smoke() {}\n");
    write(root.join("tests/fixtures/probe.rs"), "fn fixture() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 3, "fixture file should be excluded");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn run() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/http_tests.rs"),
        "src/http_tests.rs",
        true,
        None,
        false,
        "pub fn helper() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "tests/smoke.rs"),
        "tests/smoke.rs",
        true,
        None,
        false,
        "#[test]\nfn smoke() {}\n",
    );
}

#[test]
fn classifies_library_root_and_library_module() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "mod helper;\npub fn run() {}\n");
    write(root.join("src/helper.rs"), "pub struct Helper;\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "mod helper;\npub fn run() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/helper.rs"),
        "src/helper.rs",
        false,
        Some("library"),
        false,
        "pub struct Helper;\n",
    );
}

#[test]
fn classifies_binary_root_in_mixed_package() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn api() {}\n");
    write(root.join("src/main.rs"), "fn main() {}\n");
    write(root.join("src/bin/tool.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/main.rs"),
        "src/main.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/bin/tool.rs"),
        "src/bin/tool.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn classifies_custom_library_and_binary_target_paths() {
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
    write(root.join("lib/api.rs"), "mod helper;\npub fn api() {}\n");
    write(root.join("lib/helper.rs"), "pub struct Helper;\n");
    write(root.join("cmd/worker.rs"), "mod support;\nfn main() {}\n");
    write(root.join("cmd/support.rs"), "pub fn support() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "lib/api.rs"),
        "lib/api.rs",
        false,
        Some("library"),
        true,
        "mod helper;\npub fn api() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "lib/helper.rs"),
        "lib/helper.rs",
        false,
        Some("library"),
        false,
        "pub struct Helper;\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "cmd/worker.rs"),
        "cmd/worker.rs",
        false,
        Some("binary"),
        false,
        "mod support;\nfn main() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "cmd/support.rs"),
        "cmd/support.rs",
        false,
        Some("binary"),
        false,
        "pub fn support() {}\n",
    );
}

#[test]
fn marks_shared_crates_from_guardrail3_metadata() {
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
[package.metadata.guardrail3]\n\
shared = true\n",
    );
    write(root.join("src/lib.rs"), "pub struct Input;\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");
    let input = assertions::require_source_file(&inputs, "src/lib.rs");

    assertions::assert_shared_crate(input);
}

#[test]
fn leaves_shared_flag_off_for_normal_crates() {
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
    write(root.join("src/lib.rs"), "pub struct Input;\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");
    let input = assertions::require_source_file(&inputs, "src/lib.rs");

    assertions::assert_not_shared_crate(input);
}

#[test]
fn retains_source_content_without_parsing_it() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn broken( {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    let input = assertions::require_source_file(&inputs, "src/lib.rs");
    assertions::assert_source_file(
        input,
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn broken( {}\n",
    );
}

#[test]
fn threads_guardrail3_rs_waivers_into_source_inputs() {
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
    write(
        root.join("guardrail3-rs.toml"),
        "\
profile = \"library\"\n\
\n\
[[waivers]]\n\
rule = \"g3rs-code/large-type-inventory\"\n\
file = \"src/lib.rs\"\n\
selector = \"struct:CargoConfigToml\"\n\
reason = \"Schema mirror intentionally matches Cargo's top-level config surface.\"\n",
    );
    write(root.join("src/lib.rs"), "pub struct CargoConfigToml;\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");
    let input = assertions::require_source_file(&inputs, "src/lib.rs");

    assertions::assert_source_waiver(
        input,
        "g3rs-code/large-type-inventory",
        "src/lib.rs",
        "struct:CargoConfigToml",
        "Schema mirror intentionally matches Cargo's top-level config surface.",
    );
}

#[test]
fn threads_package_root_waivers_into_member_crate_sources() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/types\"]\n\
resolver = \"2\"\n",
    );
    write(
        root.join("guardrail3-rs.toml"),
        "\
profile = \"library\"\n\
\n\
[[waivers]]\n\
rule = \"g3rs-code/large-type-inventory\"\n\
file = \"crates/types/src/lib.rs\"\n\
selector = \"struct:CargoConfigToml\"\n\
reason = \"Schema mirror intentionally matches Cargo's top-level config surface.\"\n",
    );
    write(
        root.join("crates/types/Cargo.toml"),
        "\
[package]\n\
name = \"demo-types\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n",
    );
    write(
        root.join("crates/types/src/lib.rs"),
        "pub struct CargoConfigToml;\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");
    let input = assertions::require_source_file(&inputs, "crates/types/src/lib.rs");

    assertions::assert_source_waiver(
        input,
        "g3rs-code/large-type-inventory",
        "crates/types/src/lib.rs",
        "struct:CargoConfigToml",
        "Schema mirror intentionally matches Cargo's top-level config surface.",
    );
}

#[test]
fn classifies_nested_workspace_members_from_their_own_manifest() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/api\"]\n\
resolver = \"2\"\n",
    );
    write(
        root.join("crates/api/Cargo.toml"),
        "\
[package]\n\
name = \"api\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n",
    );
    write(
        root.join("crates/api/src/lib.rs"),
        "mod helper;\npub fn api() {}\n",
    );
    write(
        root.join("crates/api/src/helper.rs"),
        "pub struct Helper;\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "crates/api/src/lib.rs"),
        "crates/api/src/lib.rs",
        false,
        Some("library"),
        true,
        "mod helper;\npub fn api() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "crates/api/src/helper.rs"),
        "crates/api/src/helper.rs",
        false,
        Some("library"),
        false,
        "pub struct Helper;\n",
    );
}

#[test]
fn classifies_custom_library_root_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/custom_lib.rs\"\n",
    );
    write(root.join("src/custom_lib.rs"), "pub fn api() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/custom_lib.rs"),
        "src/custom_lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
}

#[test]
fn classifies_explicit_binary_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[bin]]\nname = \"tool\"\npath = \"src/tool.rs\"\n",
    );
    write(root.join("src/tool.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/tool.rs"),
        "src/tool.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn classifies_nested_workspace_member_ownership() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/core/src/lib.rs"), "pub fn api() {}\n");
    write(root.join("crates/core/src/main.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "crates/core/src/lib.rs"),
        "crates/core/src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "crates/core/src/main.rs"),
        "crates/core/src/main.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn leaves_unowned_source_without_profile() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("tools/probe.rs"), "pub fn probe() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "tools/probe.rs"),
        "tools/probe.rs",
        false,
        None,
        false,
        "pub fn probe() {}\n",
    );
}

#[test]
fn virtual_workspace_root_does_not_own_src_lib() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/lib.rs"), "pub fn probe() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    assertions::assert_source_file(
        assertions::require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        None,
        false,
        "pub fn probe() {}\n",
    );
}

#[test]
fn ingested_inputs_drive_code_ast_checks() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn run() { todo!(); }\n");
    write(
        root.join("tests/smoke.rs"),
        "#[test]\nfn smoke() { panic!(\"boom\"); }\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_source_checks(&workspace_crawl).expect("ingestion should succeed");

    let lib_results =
        g3rs_code_source_checks::check(assertions::require_source_file(&inputs, "src/lib.rs"));
    let test_results =
        g3rs_code_source_checks::check(assertions::require_source_file(&inputs, "tests/smoke.rs"));

    assertions::assert_code_ast_results(&lib_results, &test_results);
}

#[cfg(unix)]
#[test]
fn unreadable_selected_source_fails_ingestion() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    let secret = root.join("src/secret.rs");
    write(&secret, "pub fn hidden() {}\n");

    let mut permissions = fs::metadata(&secret)
        .expect("metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&secret, permissions).expect("chmod should succeed");

    let workspace_crawl = crawl(root).expect("crawl should succeed even with unreadable files");
    let error = crate::run::ingest_for_source_checks(&workspace_crawl)
        .expect_err("selected unreadable source should fail ingestion");

    assertions::assert_single_unreadable_error(&error);
}

#[test]
fn malformed_nearest_cargo_toml_fails_ingestion() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    write(root.join("src/lib.rs"), "pub fn run() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let error = crate::run::ingest_for_source_checks(&workspace_crawl)
        .expect_err("malformed owning Cargo.toml should fail ingestion");

    assertions::assert_single_parse_failed_error(&error);
}

#[test]
fn ingests_exception_comments_from_owned_config_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n# EXCEPTION: temporary package carve-out\nquoted = \"# not a comment\"\n",
    );
    write(
        root.join("deny.toml"),
        "advisories = { ignore = [] }\n# EXCEPTION: ignore review debt\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assert_eq!(input.files.len(), 2, "{input:#?}");
    assertions::assert_exception_comments(
        &input,
        "Cargo.toml",
        &[(4, "# EXCEPTION: temporary package carve-out")],
    );
    assertions::assert_exception_comments(
        &input,
        "deny.toml",
        &[(2, "# EXCEPTION: ignore review debt")],
    );
}

#[test]
fn ingests_multiple_exception_comments_from_one_owned_file_with_exact_lines() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("deny.toml"),
        "\
# EXCEPTION: first\n\
advisories = { ignore = [] }\n\
# EXCEPTION: second\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assert_eq!(input.files.len(), 2, "{input:#?}");
    assertions::assert_exception_comments(
        &input,
        "deny.toml",
        &[(1, "# EXCEPTION: first"), (3, "# EXCEPTION: second")],
    );
}

#[test]
fn ingests_exception_comments_from_all_supported_owned_config_filenames() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n# EXCEPTION: cargo\n",
    );
    write(root.join("guardrail3-rs.toml"), "# EXCEPTION: guardrail\n");
    write(root.join("clippy.toml"), "# EXCEPTION: clippy\n");
    write(root.join(".clippy.toml"), "# EXCEPTION: dot clippy\n");
    write(root.join("deny.toml"), "# EXCEPTION: deny\n");
    write(root.join(".deny.toml"), "# EXCEPTION: dot deny\n");
    write(root.join("rustfmt.toml"), "# EXCEPTION: rustfmt\n");
    write(
        root.join("rust-toolchain.toml"),
        "# EXCEPTION: toolchain toml\n",
    );
    write(root.join("rust-toolchain"), "# EXCEPTION: toolchain file\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assert_eq!(input.files.len(), 8, "{input:#?}");
    assert_eq!(input.exception_comments.len(), 9, "{input:#?}");
}

#[test]
fn ingests_only_parser_backed_code_config_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "# guardrail config\n");
    write(root.join("clippy.toml"), "# clippy config\n");
    write(root.join(".clippy.toml"), "# dot clippy config\n");
    write(root.join("deny.toml"), "# deny config\n");
    write(root.join(".deny.toml"), "# dot deny config\n");
    write(root.join("rustfmt.toml"), "# rustfmt config\n");
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assert_eq!(input.files.len(), 8, "{input:#?}");
    assertions::assert_parser_backed_config_files_only(&input);
}

#[test]
fn ingests_workspace_unsafe_code_lints_from_cargo() {
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

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assertions::assert_workspace_unsafe_code_level(&input, "Cargo.toml", "deny");
}

#[test]
fn ingests_workspace_unsafe_code_lints_from_detailed_form() {
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
unsafe_code = { level = \"forbid\", priority = 0 }\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assertions::assert_workspace_unsafe_code_detailed_level(&input, "Cargo.toml", "forbid");
}

#[test]
fn ignores_foreign_nested_repo_config_files() {
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
# EXCEPTION: root workspace inventory\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n# EXCEPTION: member cargo inventory\n",
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
# EXCEPTION: foreign workspace inventory\n",
    );
    write(
        root.join("vendor/foreign/deny.toml"),
        "# EXCEPTION: foreign deny inventory\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("config ingestion should succeed");

    assert_eq!(input.files.len(), 2, "{input:#?}");
    assert!(input.files.iter().all(|file| {
        file.rel_path == "Cargo.toml" || file.rel_path == "crates/core/Cargo.toml"
    }));
    assertions::assert_workspace_unsafe_code_level(&input, "Cargo.toml", "forbid");
}

#[test]
fn malformed_cargo_toml_fails_config_ingestion() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace\nbroken = true");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let error = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect_err("malformed cargo should fail config ingestion");

    assertions::assert_single_parse_failed_error(&error);
}

#[cfg(unix)]
#[test]
fn unreadable_owned_config_file_fails_config_ingestion() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    let deny_toml = root.join("deny.toml");
    write(&deny_toml, "# EXCEPTION: hidden\n");

    let mut permissions = fs::metadata(&deny_toml)
        .expect("metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&deny_toml, permissions).expect("chmod should succeed");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let error = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect_err("unreadable config should fail config ingestion");

    assertions::assert_single_unreadable_error(&error);
}

#[cfg(unix)]
#[test]
fn unreadable_foreign_config_file_does_not_fail_config_ingestion() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let foreign_deny = root.join("vendor/foreign/deny.toml");
    write(&foreign_deny, "# EXCEPTION: foreign hidden\n");

    let mut permissions = fs::metadata(&foreign_deny)
        .expect("metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&foreign_deny, permissions).expect("chmod should succeed");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&workspace_crawl)
        .expect("foreign unreadable config should be ignored");

    assert_eq!(input.files.len(), 1, "{input:#?}");
    assert_eq!(
        assertions::require_config_file(&input, "Cargo.toml").rel_path,
        "Cargo.toml"
    );
}
