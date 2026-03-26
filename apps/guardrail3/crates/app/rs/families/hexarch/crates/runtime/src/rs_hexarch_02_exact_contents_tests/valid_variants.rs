use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

#[test]
fn crates_with_only_gitkeep_still_defer_to_missing_required_dirs_not_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = assertions::run_family(tmp.path());
    let rule_01 = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    let rule_02 = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert!(
        rule_01.is_empty(),
        "rule 01 should treat .gitkeep as present: {rule_01:#?}"
    );

    let devctl_rule_02: Vec<_> = rule_02
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();
    assert_eq!(
        devctl_rule_02.len(),
        4,
        "expected four missing top-level dirs for devctl: {devctl_rule_02:#?}"
    );
}

#[test]
fn optional_macros_dir_is_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/macros/.gitkeep", "");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert!(
        errors.is_empty(),
        "optional crates/macros/ should not trigger rule 02: {errors:#?}"
    );
}
