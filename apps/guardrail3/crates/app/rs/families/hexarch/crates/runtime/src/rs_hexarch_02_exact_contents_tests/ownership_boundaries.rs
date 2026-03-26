use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn packages_invalid_crates_shape_is_not_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "packages/phantom/crates/misc/.gitkeep", "");
    write_file(tmp.path(), "packages/phantom/crates/mod.rs", "// stray");

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-02").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn stray_app_without_cargo_toml_is_not_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/fake-service/crates/misc/.gitkeep", "");
    write_file(tmp.path(), "apps/fake-service/crates/mod.rs", "// stray");

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-02").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn newly_discovered_rust_app_with_partial_crates_is_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/scheduler/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write_file(tmp.path(), "apps/scheduler/crates/domain/.gitkeep", "");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    let scheduler: Vec<_> = errors
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/scheduler/crates"))
        .collect();
    assert_eq!(scheduler.len(), 3, "{scheduler:#?}");
    assert!(
        scheduler
            .iter()
            .any(|error| error.title.contains("adapters/"))
    );
    assert!(scheduler.iter().any(|error| error.title.contains("app/")));
    assert!(scheduler.iter().any(|error| error.title.contains("ports/")));
}

#[test]
fn non_owned_nested_looking_shape_under_packages_is_still_out_of_scope() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/lookalike/crates/adapters/inbound/mcp/crates/utils/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "packages/lookalike/crates/adapters/inbound/mcp/crates/mod.rs",
        "// stray",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-02").is_empty(),
        "{results:#?}"
    );
}
