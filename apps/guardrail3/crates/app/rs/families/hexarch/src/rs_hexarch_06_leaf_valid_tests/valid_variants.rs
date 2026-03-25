use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};
use super::cases::{nested_hex_everywhere, owned_leaf_dirs};

#[test]
fn gitkeep_only_leaf_is_valid_placeholder() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/placeholder/.gitkeep",
        "",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "gitkeep-only leaf should be valid: {errors:#?}"
    );
}

#[test]
fn inner_hex_owner_without_crates_becomes_leaf_error() {
    let tmp = copy_fixture();
    std::fs::remove_dir_all(
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("remove inner crates");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(
        errors.len(),
        1,
        "expected one inner-hex leaf error: {errors:#?}"
    );
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

#[test]
fn crate_leaf_with_gitkeep_is_valid_everywhere() {
    let tmp = copy_fixture();
    let leaf_dirs = owned_leaf_dirs(tmp.path(), "kept_crate");
    for rel in &leaf_dirs {
        write_file(tmp.path(), &format!("{rel}/.gitkeep"), "");
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            "[package]\nname = \"kept-crate\"\nversion = \"0.1.0\"\n",
        );
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "crate leaves with .gitkeep should stay valid: {errors:#?}"
    );
}

#[test]
fn nested_hex_with_gitkeep_placeholders_is_valid_everywhere() {
    let tmp = copy_fixture();
    let _leaf_dirs = nested_hex_everywhere(tmp.path(), "hex_keep");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "nested hex leaves with .gitkeep placeholders should stay valid: {errors:#?}"
    );
}
