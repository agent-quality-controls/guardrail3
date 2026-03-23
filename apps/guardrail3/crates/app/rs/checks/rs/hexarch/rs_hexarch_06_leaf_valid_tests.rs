use super::super::test_support::{assert_no_error, copy_fixture, create_dir, errors_by_id, run_family, write_file};

#[test]
fn golden_has_no_rule_06_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-06");
}

#[test]
fn orphan_leaf_without_cargo_or_crates_errors() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/devctl/crates/domain/orphan");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "expected one orphan-leaf error: {errors:#?}");
    assert!(errors[0].title.contains("missing Cargo.toml"));
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/domain/orphan")
    );
}

#[test]
fn gitkeep_only_leaf_is_valid_placeholder() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/domain/placeholder/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(errors.is_empty(), "gitkeep-only leaf should be valid: {errors:#?}");
}

#[test]
fn leaf_with_both_cargo_and_crates_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/hybrid/Cargo.toml",
        "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/hybrid/crates/domain/.gitkeep",
        "",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "expected one hybrid error: {errors:#?}");
    assert!(errors[0].title.contains("both Cargo.toml and crates/"));
}

#[test]
fn inner_hex_owner_without_cargo_or_crates_is_leaf_error() {
    let tmp = copy_fixture();
    std::fs::remove_dir_all(tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"))
        .expect("remove inner crates");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "expected one inner-hex leaf error: {errors:#?}");
    assert!(errors[0].title.contains("missing Cargo.toml"));
    assert!(
        errors[0]
            .file
            .as_deref()
            .unwrap_or("")
            .contains("apps/backend/crates/adapters/inbound/mcp"),
        "expected mcp path in file field: {:?}",
        errors[0]
    );
}
