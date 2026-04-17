use g3rs_clippy_types::{G3RsClippyConfigState, G3RsClippyRustPolicyState};
use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

fn typed_msrv(input: &g3rs_clippy_types::G3RsClippyConfigChecksInput) -> Option<&str> {
    match &input.clippy {
        G3RsClippyConfigState::Parsed(document) => {
            clippy_toml_parser::typed(document).and_then(|clippy| clippy.msrv.as_deref())
        }
        G3RsClippyConfigState::Unreadable { .. }
        | G3RsClippyConfigState::ParseError { .. } => None,
    }
}

fn clippy_conf_dir_override_rel_paths(
    input: &g3rs_clippy_types::G3RsClippyConfigChecksInput,
) -> Vec<&str> {
    input
        .cargo_configs
        .iter()
        .filter_map(|cargo_config| match cargo_config {
            g3rs_clippy_types::G3RsClippyCargoConfigState::Parsed {
                rel_path,
                cargo_config,
            } if cargo_config.env.contains_key("CLIPPY_CONF_DIR") => Some(rel_path.as_str()),
            _ => None,
        })
        .collect()
}

fn typed_cargo_root_rel_path(
    input: &g3rs_clippy_types::G3RsClippyConfigChecksInput,
) -> Option<&str> {
    match &input.cargo_root {
        g3rs_clippy_types::G3RsClippyCargoRootState::Parsed { rel_path, .. } => Some(rel_path),
        g3rs_clippy_types::G3RsClippyCargoRootState::Missing
        | g3rs_clippy_types::G3RsClippyCargoRootState::Unreadable { .. }
        | g3rs_clippy_types::G3RsClippyCargoRootState::ParseError { .. } => None,
    }
}

fn typed_cargo_member_rels(
    input: &g3rs_clippy_types::G3RsClippyConfigChecksInput,
) -> Vec<&str> {
    input
        .cargo_workspace_members
        .iter()
        .filter_map(|member| match member {
            g3rs_clippy_types::G3RsClippyCargoMemberState::Parsed { member_rel, .. } => {
                Some(member_rel.as_str())
            }
            g3rs_clippy_types::G3RsClippyCargoMemberState::Unreadable { .. }
            | g3rs_clippy_types::G3RsClippyCargoMemberState::ParseError { .. } => None,
        })
        .collect()
}

#[test]
fn ingests_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the fixture file"
    );
}

#[test]
fn ingests_dot_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid .clippy.toml");
    assert_eq!(
        input.clippy_rel_path, ".clippy.toml",
        "clippy_rel_path should be the dot-prefixed variant when only .clippy.toml exists"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the dot-prefixed fixture file"
    );
}

#[test]
fn prefers_dot_clippy_toml_over_plain_variant() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed when both clippy config variants exist");
    assert_eq!(
        input.clippy_rel_path, ".clippy.toml",
        ".clippy.toml should win same-root precedence when both variants exist"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.80"),
        "parsed content should come from .clippy.toml (1.80), not clippy.toml (1.85)"
    );
}

#[test]
fn keeps_raw_parseable_but_typed_invalid_clippy_for_config_checks() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "disallowed-methods = [{ path = 7 }]\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect(
        "config ingestion should preserve raw-parseable clippy.toml for parseability and section-shape checks instead of aborting on typed parse failure",
    );
    match input.clippy {
        G3RsClippyConfigState::Parsed(document) => match clippy_toml_parser::parse_error_reason(&document) {
            Some(reason) => assert!(
                reason.contains("path"),
                "typed parse error should preserve the parser reason: {reason}"
            ),
            None => panic!("expected typed parse error document"),
        },
        other => panic!("expected raw-parseable typed-invalid clippy state, got {other:#?}"),
    }
}

#[test]
fn filetree_ingestion_is_not_a_stub_when_root_configs_exist() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_file_tree_checks(&crawl).expect(
        "filetree ingestion should build real root coverage and same-root conflict facts instead of returning a stub error",
    );
    assert_eq!(
        input,
        g3rs_clippy_types::G3RsClippyFileTreeChecksInput {
            preferred_root_config_rel_path: Some(".clippy.toml".to_owned()),
            shadowed_same_root_configs: vec![g3rs_clippy_types::G3RsClippyShadowedConfig {
                rel_path: "clippy.toml".to_owned(),
                preferred_rel_path: ".clippy.toml".to_owned(),
            }],
        }
    );
}

#[test]
fn fails_when_clippy_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::run::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should return ClippyTomlNotFound when no clippy config exists in the workspace"
    );
}

#[test]
fn keeps_raw_parse_error_state_for_malformed_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "{{{{not valid toml at all}}}}");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve malformed clippy.toml state");
    assert!(
        matches!(input.clippy, G3RsClippyConfigState::ParseError { .. }),
        "{input:#?}"
    );
}

#[test]
fn keeps_typed_parse_error_state_for_unknown_fields() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "totally_fake_field = true\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve typed parse errors for unknown fields");
    assert!(
        matches!(
            input.clippy,
            G3RsClippyConfigState::Parsed(_)
        ),
        "{input:#?}"
    );
}

#[test]
fn keeps_typed_parse_error_state_for_wrong_value_type() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = 42\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve typed parse errors for wrong value types");
    assert!(
        matches!(
            input.clippy,
            G3RsClippyConfigState::Parsed(_)
        ),
        "{input:#?}"
    );
}

#[test]
fn empty_clippy_toml_parses_to_defaults() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for an empty clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert!(
        typed_msrv(&input).is_none(),
        "empty clippy.toml should have no msrv set"
    );
}

#[test]
fn nested_clippy_toml_is_not_selected() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("subdir/clippy.toml"),
        "msrv = \"1.85\"\n",
    );

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::run::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should not select a clippy.toml in a subdirectory, only at the workspace root"
    );
}

#[test]
fn threads_guardrail3_waivers_into_config_input() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "max-struct-bools = 9\n");
    write(
        root.join("guardrail3-rs.toml"),
        "profile = \"library\"\n\
[[waivers]]\n\
rule = \"RS-CLIPPY-CONFIG-01\"\n\
file = \"clippy.toml\"\n\
selector = \"key:max-struct-bools\"\n\
reason = \"schema mirror\"\n",
    );

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should thread guardrail waivers into config input");

    g3rs_clippy_ingestion_assertions::run::assert_single_waiver(
        &input,
        "RS-CLIPPY-CONFIG-01",
        "clippy.toml",
        "key:max-struct-bools",
        "schema mirror",
    );
}

#[test]
fn malformed_root_cargo_toml_does_not_abort_clippy_config_ingestion() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nnot = [valid");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("malformed root Cargo.toml should disable published-library policy, not abort clippy ingestion");

    assert!(
        matches!(
            &input.cargo_root,
            g3rs_clippy_types::G3RsClippyCargoRootState::ParseError { .. }
        ),
        "{input:#?}"
    );
}

#[test]
fn uses_guardrail3_rs_toml_for_library_policy() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"[workspace]
members = []

[package]
name = "workspace-root"
version = "0.1.0"
edition = "2024"
"#,
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("valid guardrail3-rs.toml should drive clippy library policy");

    assert_eq!(typed_cargo_root_rel_path(&input), Some("Cargo.toml"), "{input:#?}");
}

#[test]
fn ignores_legacy_guardrail3_toml_for_library_policy() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"[workspace]
members = []

[package]
name = "workspace-root"
version = "0.1.0"
edition = "2024"
"#,
    );
    write(root.join("guardrail3.toml"), "[profile]\nname = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("legacy guardrail3.toml should no longer drive clippy policy");

    assert_eq!(typed_cargo_root_rel_path(&input), Some("Cargo.toml"), "{input:#?}");
}

#[test]
fn surfaces_guardrail3_rs_parse_errors_in_policy_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join("guardrail3-rs.toml"), "profile = 7\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("invalid guardrail3-rs.toml should be preserved in clippy policy state");

    match input.rust_policy {
        G3RsClippyRustPolicyState::ParseError { rel_path, .. } => {
            assert_eq!(rel_path, "guardrail3-rs.toml");
        }
        other => panic!("expected ParseError policy state, got {other:#?}"),
    }
}

#[test]
fn library_policy_respects_workspace_inherited_member_publish_false() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"[workspace]
members = ["member"]

[workspace.package]
publish = false
"#,
    );
    write(
        root.join("member/Cargo.toml"),
        r#"[package]
name = "member"
version = "0.1.0"
edition = "2024"
publish = { workspace = true }
"#,
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("workspace-inherited publishability should not abort clippy ingestion");

    assert_eq!(typed_cargo_member_rels(&input), vec!["member"], "{input:#?}");
}

#[test]
fn library_policy_respects_workspace_inherited_member_publish_true() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"[workspace]
members = ["member"]

[workspace.package]
publish = true
"#,
    );
    write(
        root.join("member/Cargo.toml"),
        r#"[package]
name = "member"
version = "0.1.0"
edition = "2024"
publish = { workspace = true }
"#,
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("workspace-inherited publishability should not abort clippy ingestion");

    assert_eq!(typed_cargo_member_rels(&input), vec!["member"], "{input:#?}");
}

#[test]
fn ignored_but_recovered_clippy_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "clippy.toml\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    let input = result.expect(
        "ingestion should succeed for a gitignored clippy.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "recovered clippy.toml should still resolve to the root-relative path"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "recovered clippy.toml should be parsed correctly"
    );
}

#[test]
fn ingests_root_cargo_config_as_typed_file_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(
        root.join(".cargo/config.toml"),
        "[env]\nCLIPPY_CONF_DIR = \"config/clippy\"\n",
    );

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("clippy ingestion should preserve typed .cargo/config state");

    assert_eq!(
        clippy_conf_dir_override_rel_paths(&input),
        vec![".cargo/config.toml"],
        "clippy ingestion should pass the parsed cargo config file through instead of a pre-sliced override summary"
    );
}

#[test]
fn preserves_cargo_config_parse_error_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".cargo/config"), "[env\nbroken = true\n");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("clippy ingestion should preserve malformed cargo config state");

    assert!(
        input.cargo_configs.iter().any(|cargo_config| matches!(
            cargo_config,
            g3rs_clippy_types::G3RsClippyCargoConfigState::ParseError { rel_path, .. }
                if rel_path == ".cargo/config"
        )),
        "{input:#?}"
    );
}
