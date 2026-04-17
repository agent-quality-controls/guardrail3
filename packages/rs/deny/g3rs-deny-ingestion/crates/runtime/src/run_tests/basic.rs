use g3rs_deny_types::G3RsDenyRustPolicyState;
use guardrail3_rs_toml_parser::types::RustProfile;
use tempfile::tempdir;

use super::helpers::{crawl, git_init, make_unreadable, write};

#[test]
fn ingests_deny_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("deny.toml"),
        "[advisories]\ndb-path = \"~/.cargo/advisory-db\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid deny.toml");
    assert_eq!(
        input.deny_rel_path, "deny.toml",
        "deny_rel_path should be the workspace-root-relative path to deny.toml"
    );
    assert!(
        input.deny.advisories.is_some(),
        "parsed deny.toml should contain an [advisories] section when one is defined"
    );
}

#[test]
fn ingests_dot_deny_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join(".deny.toml"),
        "[advisories]\ndb-path = \"~/.cargo/advisory-db\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid .deny.toml");
    assert_eq!(
        input.deny_rel_path, ".deny.toml",
        "deny_rel_path should be .deny.toml when only the dotfile variant exists"
    );
}

#[test]
fn ingests_root_cargo_deny_toml_when_no_higher_precedence_root_file_exists() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join(".cargo/deny.toml"),
        "[advisories]\ndb-path = \"~/.cargo/advisory-db\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for root .cargo/deny.toml");
    assert_eq!(
        input.deny_rel_path, ".cargo/deny.toml",
        "deny_rel_path should prefer root .cargo/deny.toml when no root deny.toml or .deny.toml exists"
    );
}

#[test]
fn prefers_deny_toml_over_dot_variant() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    // Write distinguishable content so we can verify which file was actually read
    write(
        root.join("deny.toml"),
        "[advisories]\ndb-path = \"from-deny-toml\"\n",
    );
    write(
        root.join(".deny.toml"),
        "[advisories]\ndb-path = \"from-dot-deny-toml\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input =
        result.expect("ingestion should succeed when both deny.toml and .deny.toml exist");
    assert_eq!(
        input.deny_rel_path, "deny.toml",
        "deny.toml should be preferred over .deny.toml when both exist"
    );
    let db_path = input
        .deny
        .advisories
        .as_ref()
        .expect("parsed deny.toml should have [advisories] section to verify correct file was read")
        .db_path
        .as_deref();
    assert_eq!(
        db_path,
        Some("from-deny-toml"),
        "content should come from deny.toml, not .deny.toml, when both exist"
    );
}

#[test]
fn fails_when_deny_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::run::IngestionError::DenyTomlNotFound)),
        "ingestion should return DenyTomlNotFound when no deny config exists in the workspace"
    );
}

#[test]
fn fails_on_malformed_deny_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::run::IngestionError::ParseFailed { .. })),
        "ingestion should return ParseFailed when deny.toml contains invalid TOML"
    );
}

#[test]
fn fails_on_unreadable_selected_deny_file() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let deny_path = root.join("deny.toml");
    write(
        &deny_path,
        "[advisories]\ndb-path = \"~/.cargo/advisory-db\"\n",
    );

    let crawl = crawl(root);
    make_unreadable(&deny_path);

    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::run::IngestionError::Unreadable { .. })),
        "ingestion should return Unreadable when the selected deny file cannot be read"
    );
}

#[test]
fn ignored_but_recovered_deny_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "deny.toml\n");
    write(
        root.join("deny.toml"),
        "[advisories]\ndb-path = \"~/.cargo/advisory-db\"\n",
    );

    let crawl = crawl(root);

    // Verify the crawl actually marked this as Ignored (proving recovery path)
    let crawl_entry = g3rs_workspace_crawl::entry(&crawl, "deny.toml")
        .expect("deny.toml should be present in crawl via recovery even when gitignored");
    assert_eq!(
        crawl_entry.ignore_state,
        g3rs_workspace_crawl::G3RsWorkspaceIgnoreState::Ignored,
        "deny.toml should have Ignored state when gitignored, proving the recovery path was exercised"
    );

    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect(
        "ingestion should succeed for a gitignored deny.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.deny_rel_path, "deny.toml",
        "recovered deny.toml should still resolve to the root-relative path"
    );
}

#[test]
fn nested_deny_toml_is_not_selected() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("deny.toml"),
        "[advisories]\ndb-path = \"root\"\n",
    );
    write(
        root.join("packages/foo/deny.toml"),
        "[advisories]\ndb-path = \"nested\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed when root deny.toml exists");
    assert_eq!(
        input.deny_rel_path, "deny.toml",
        "ingestion should select the root deny.toml, not a nested one"
    );
}

#[test]
fn empty_deny_toml_parses_to_hollow_input() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for an empty deny.toml");
    assert!(
        input.deny.advisories.is_none(),
        "empty deny.toml should have no [advisories] section"
    );
    assert!(
        input.deny.bans.is_none(),
        "empty deny.toml should have no [bans] section"
    );
}

#[test]
fn guardrail3_rs_toml_drives_library_profile() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(
        input.rust_policy,
        G3RsDenyRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: Some(RustProfile::Library),
        }
    );
}

#[test]
fn legacy_guardrail3_toml_is_ignored() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join("guardrail3.toml"), "[profile]\nname = \"library\"\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(
        input.rust_policy,
        G3RsDenyRustPolicyState::Missing,
        "legacy guardrail3.toml must not affect deny profile resolution"
    );
}

#[test]
fn guardrail3_rs_parse_errors_surface_in_rust_policy_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"invalid\"\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");

    assert!(
        matches!(
            input.rust_policy,
            G3RsDenyRustPolicyState::ParseError { ref rel_path, .. }
            if rel_path == "guardrail3-rs.toml"
        ),
        "invalid guardrail3-rs.toml must surface as deny rust policy parse error"
    );
}
