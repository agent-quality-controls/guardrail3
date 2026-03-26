use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn ignored_untracked_invalid_leaf_still_errors() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "orphan/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan/src/lib.rs",
        "pub fn orphan() {}\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app/orphan")
    );
    assert!(
        errors[0].title.contains("missing Cargo.toml"),
        "{errors:#?}"
    );
}

#[test]
fn ignored_untracked_valid_crate_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "valid_crate/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/valid_crate/Cargo.toml",
        "[package]\nname = \"valid-crate\"\nversion = \"0.1.0\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "ignored valid crate leaf should not be misclassified: {errors:#?}"
    );
}

#[test]
fn ignored_untracked_valid_nested_hex_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "valid_hex/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/valid_hex/crates/app/.gitkeep",
        "",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "ignored valid nested-hex leaf should not be misclassified: {errors:#?}"
    );
}

#[test]
fn ignored_untracked_gitkeep_placeholder_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "future_leaf/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/future_leaf/.gitkeep",
        "",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "ignored .gitkeep placeholder leaf should stay valid: {errors:#?}"
    );
}

#[test]
fn ignored_untracked_hybrid_leaf_still_hits_both_branch() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "hybrid_leaf/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid_leaf/Cargo.toml",
        "[package]\nname = \"hybrid-leaf\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid_leaf/crates/app/.gitkeep",
        "",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app/hybrid_leaf")
    );
    assert!(
        errors[0].title.contains("both Cargo.toml and crates/"),
        "{errors:#?}"
    );
}
