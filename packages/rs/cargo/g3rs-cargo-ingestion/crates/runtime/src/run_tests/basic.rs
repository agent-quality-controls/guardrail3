use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_cargo_ingestion_assertions::run as assertions;
use tempfile::tempdir;

/// Initialize a real git repo in `path` for ingestion fixtures.
///
/// The centralized fs/process bans apply to production code paths; these
/// test-only fixture helpers materialize real on-disk inputs to exercise the
/// workspace crawl end-to-end.
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk git+files to exercise the workspace crawl; the centralized fs/process bans target production code paths only"
)]
fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

/// Write a fixture file, materializing missing parent directories. See [`git_init`].
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk git+files to exercise the workspace crawl; the centralized fs/process bans target production code paths only"
)]
fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

#[cfg(unix)]
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper that toggles real on-disk file permissions to exercise the Unreadable code paths; the centralized fs bans target production code paths only"
)]
fn make_unreadable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("stat should succeed")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(path, permissions).expect("chmod should succeed");
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn ingests_workspace_root_with_members_for_config_checks() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("ingestion should succeed for a valid workspace root");

    assert_eq!(input.root.cargo_rel_path, "Cargo.toml");
    assert_eq!(
        input.root.kind,
        g3rs_cargo_types::G3RsCargoPolicyRootKind::WorkspaceRoot
    );
    assert_eq!(input.workspace_members.len(), 1);
    assert_eq!(input.workspace_members[0].member_rel, "crates/api");
    assert!(matches!(
        cargo_toml_parser::document::lints_workspace_state(&input.workspace_members[0].cargo),
        cargo_toml_parser::types::CargoBoolFieldState::Value(true)
    ));
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn member_may_inherit_workspace_edition() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            version = "0.1.0"
            edition.workspace = true

            [lints]
            workspace = true
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("ingestion should accept workspace edition inheritance");

    assert_eq!(input.workspace_members.len(), 1);
    assert!(matches!(
        cargo_toml_parser::document::package_string_field(
            &input.workspace_members[0].cargo,
            "edition"
        ),
        cargo_toml_parser::types::CargoStringFieldState::Inherit
    ));
}

#[test]
fn ingests_hybrid_root_with_package_fallback_fields() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = []
            resolver = "2"

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"

            [lints.rust]
            warnings = "deny"
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("ingestion should succeed for a valid hybrid root");

    assert_eq!(
        input.root.kind,
        g3rs_cargo_types::G3RsCargoPolicyRootKind::WorkspaceRoot
    );
    assert!(matches!(
        cargo_toml_parser::document::root_package_string_field(&input.root.cargo, "edition"),
        cargo_toml_parser::types::CargoStringFieldState::Value("2024")
    ));
    assert!(cargo_toml_parser::document::policy_lints(&input.root.cargo, "rust").is_some());
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn guardrail3_rs_toml_drives_profile_and_ignores_legacy_guardrail3_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("guardrail3-rs.toml"),
        "profile = \"library\"\n\n[[waivers]]\nrule = \"g3rs-cargo/approved-allow-inventory\"\nfile = \"Cargo.toml\"\nselector = \"clippy:module_name_repetitions\"\nreason = \"Temporary lint suppression while API cleanup lands.\"\n",
    );
    write(
        root.join("guardrail3.toml"),
        "[profile]\nname = \"service\"\n\n[[escape_hatches]]\nfamily = \"cargo\"\nfile = \"Cargo.toml\"\nkind = \"lint_allow\"\nselector = \"clippy:wrong\"\nreason = \"wrong\"\n",
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("config ingestion should succeed with rust-only policy");

    assert!(
        matches!(
            &input.root.rust_policy,
            g3rs_cargo_types::G3RsCargoRustPolicyState::Parsed { .. }
        ),
        "expected parsed rust policy, got {:?}",
        &input.root.rust_policy
    );
    let g3rs_cargo_types::G3RsCargoRustPolicyState::Parsed {
        profile, waivers, ..
    } = &input.root.rust_policy
    else {
        return;
    };
    assert_eq!(
        *profile,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Library)
    );
    assert_eq!(waivers.len(), 1, "{waivers:#?}");
    assert_eq!(waivers[0].rule, "g3rs-cargo/approved-allow-inventory");
    assert_eq!(waivers[0].selector, "clippy:module_name_repetitions");
}

#[test]
fn malformed_guardrail3_rs_toml_degrades_to_rust_policy_parse_error() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = [");

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("config ingestion should keep rust-policy parse failures in state");

    assertions::assert_rust_policy_parse_error(&input.root.rust_policy);
}

#[test]
fn legacy_guardrail3_toml_is_ignored() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("guardrail3.toml"), "[profile");

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("legacy guardrail3.toml must be ignored");

    assert!(matches!(
        input.root.rust_policy,
        g3rs_cargo_types::G3RsCargoRustPolicyState::Missing
    ));
}

#[test]
fn invalid_guardrail3_rs_profile_degrades_to_rust_policy_parse_error() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = []\n");

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("config ingestion should keep rust-policy shape failures in state");

    assertions::assert_rust_policy_parse_error(&input.root.rust_policy);
}

#[test]
fn invalid_guardrail3_rs_waiver_degrades_to_rust_policy_parse_error() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("guardrail3-rs.toml"),
        r#"
            profile = "library"

            [[waivers]]
            rule = "g3rs-cargo/unapproved-allow-entries"
            file = "Cargo.toml"
            selector = "rust:warnings"
            reason = []
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("config ingestion should keep invalid waiver shape in state");

    assertions::assert_rust_policy_parse_error(&input.root.rust_policy);
}

#[test]
fn config_ingestion_fails_closed_on_invalid_workspace_members_shape() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = "crates/api"
            resolver = "2"
        "#,
    );

    let result = crate::run::ingest_for_config_checks(&crawl(root));

    assert!(
        result.is_err(),
        "invalid workspace members must fail closed"
    );
}

#[test]
fn config_ingestion_fails_closed_on_invalid_workspace_member_glob() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/["]
            resolver = "2"
        "#,
    );

    let result = crate::run::ingest_for_config_checks(&crawl(root));

    assert!(
        result.is_err(),
        "invalid workspace member glob syntax must fail closed"
    );
}

#[test]
fn config_ingestion_fails_closed_on_invalid_workspace_exclude_glob() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            exclude = ["crates/["]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"
        "#,
    );

    let result = crate::run::ingest_for_config_checks(&crawl(root));

    assert!(
        result.is_err(),
        "invalid workspace exclude glob syntax must fail closed"
    );
}

#[test]
fn config_ingestion_fails_closed_on_invalid_workspace_exclude_shape() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            exclude = [1]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"
        "#,
    );

    let result = crate::run::ingest_for_config_checks(&crawl(root));

    assert!(
        result.is_err(),
        "invalid workspace exclude entries must fail closed"
    );
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn config_ingestion_normalizes_root_member_dot_patterns() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = [".", "./"]
            resolver = "2"

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("root member dot patterns should normalize to the root manifest");

    assert_eq!(
        input.workspace_members.len(),
        1,
        "{:#?}",
        input.workspace_members
    );
    assert_eq!(input.workspace_members[0].member_rel, "");
    assert_eq!(input.workspace_members[0].cargo_rel_path, "Cargo.toml");
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn config_ingestion_keeps_healthy_members_when_another_member_manifest_is_malformed() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api", "crates/broken"]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true
        "#,
    );
    write(root.join("crates/broken/Cargo.toml"), "[package");

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("malformed sibling member should not abort config ingestion");

    assert_eq!(
        input.workspace_members.len(),
        1,
        "{:#?}",
        input.workspace_members
    );
    assert_eq!(input.workspace_members[0].member_rel, "crates/api");
}

#[test]
fn config_ingestion_skips_missing_declared_member_manifest() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("missing declared member should be left to filetree checks");

    assert!(
        input.workspace_members.is_empty(),
        "{:#?}",
        input.workspace_members
    );
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[test]
fn config_ingestion_preserves_invalid_lints_workspace_shape_per_member() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = "yes"
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("invalid [lints].workspace should remain member-scoped config state");

    assert_eq!(input.workspace_members.len(), 1);
    assert!(match cargo_toml_parser::document::lints_workspace_state(
        &input.workspace_members[0].cargo
    ) {
        cargo_toml_parser::types::CargoBoolFieldState::WrongType(toml::Value::String(value)) => {
            value == "yes"
        }
        cargo_toml_parser::types::CargoBoolFieldState::Missing
        | cargo_toml_parser::types::CargoBoolFieldState::Value(_)
        | cargo_toml_parser::types::CargoBoolFieldState::WrongType(_) => false,
    });
}

#[cfg(unix)]
#[test]
fn unreadable_root_cargo_toml_fails_ingestion() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let cargo_path = root.join("Cargo.toml");
    write(
        &cargo_path,
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
    );

    make_unreadable(&cargo_path);
    let crawl = crawl(root);
    #[expect(
        clippy::disallowed_methods,
        reason = "restore fixture permissions for tests that exercise unreadable code paths; centralized fs bans target production code only"
    )]
    let _restore = fs::set_permissions(&cargo_path, fs::Permissions::from_mode(0o644));

    let err = crate::run::ingest_for_config_checks(&crawl)
        .expect_err("unreadable root Cargo.toml should fail");

    assertions::assert_unreadable_error(&err, "Cargo.toml");
}

#[test]
fn config_ingestion_keeps_healthy_members_when_another_member_has_invalid_lints_workspace() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api", "crates/bad"]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true
        "#,
    );
    write(
        root.join("crates/bad/Cargo.toml"),
        r#"
            [package]
            name = "bad"
            edition = "2024"

            [lints]
            workspace = "yes"
        "#,
    );

    let input = crate::run::ingest_for_config_checks(&crawl(root))
        .expect("invalid sibling member must not abort healthy config fan-out");

    assert_eq!(
        input.workspace_members.len(),
        2,
        "{:#?}",
        input.workspace_members
    );
    assert!(
        input.workspace_members.iter().any(|member| {
            member.member_rel == "crates/api"
                && matches!(
                    cargo_toml_parser::document::lints_workspace_state(&member.cargo),
                    cargo_toml_parser::types::CargoBoolFieldState::Value(true)
                )
        }),
        "{:#?}",
        input.workspace_members
    );
    assert!(
        input.workspace_members.iter().any(|member| {
            member.member_rel == "crates/bad"
                && match cargo_toml_parser::document::lints_workspace_state(&member.cargo) {
                    cargo_toml_parser::types::CargoBoolFieldState::WrongType(
                        toml::Value::String(value),
                    ) => value == "yes",
                    cargo_toml_parser::types::CargoBoolFieldState::Missing
                    | cargo_toml_parser::types::CargoBoolFieldState::Value(_)
                    | cargo_toml_parser::types::CargoBoolFieldState::WrongType(_) => false,
                }
        }),
        "{:#?}",
        input.workspace_members
    );
}

#[test]
fn config_ingestion_fails_closed_on_invalid_workspace_rust_version_before_package_fallback() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            rust-version = []

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"
            rust-version = "1.84"
        "#,
    );

    let err = crate::run::ingest_for_config_checks(&crawl(root))
        .expect_err("invalid workspace rust-version must fail before any package fallback");

    assertions::assert_parse_failed_error(&err, "Cargo.toml");
}

#[expect(
    clippy::indexing_slicing,
    reason = "test asserts vec/slice length above this index access; the lint flags the index but the pre-assertion guarantees it cannot panic at runtime"
)]
#[cfg(unix)]
#[test]
fn config_ingestion_skips_unreadable_member_manifest() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api", "crates/secret"]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true
        "#,
    );
    let secret = root.join("crates/secret/Cargo.toml");
    write(
        &secret,
        r#"
            [package]
            name = "secret"
            edition = "2024"
        "#,
    );

    make_unreadable(&secret);
    let crawl = crawl(root);
    #[expect(
        clippy::disallowed_methods,
        reason = "restore fixture permissions for tests that exercise unreadable code paths; centralized fs bans target production code only"
    )]
    let _restore = fs::set_permissions(&secret, fs::Permissions::from_mode(0o644));

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("unreadable sibling member must not abort healthy config fan-out");

    assert_eq!(
        input.workspace_members.len(),
        1,
        "{:#?}",
        input.workspace_members
    );
    assert_eq!(input.workspace_members[0].member_rel, "crates/api");
}

#[cfg(unix)]
#[test]
fn unreadable_guardrail3_rs_toml_degrades_to_rust_policy_unreadable() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let guardrail = root.join("guardrail3-rs.toml");
    write(&guardrail, "profile = \"library\"\n");

    make_unreadable(&guardrail);
    let crawl = crawl(root);
    #[expect(
        clippy::disallowed_methods,
        reason = "restore fixture permissions for tests that exercise unreadable code paths; centralized fs bans target production code only"
    )]
    let _restore = fs::set_permissions(&guardrail, fs::Permissions::from_mode(0o644));

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("unreadable guardrail3-rs.toml should stay in state");

    assertions::assert_rust_policy_unreadable(&input.root.rust_policy);
}
