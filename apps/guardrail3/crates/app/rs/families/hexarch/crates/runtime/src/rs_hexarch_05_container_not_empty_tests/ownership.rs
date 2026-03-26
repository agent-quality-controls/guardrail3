use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use crate::test_support::{copy_fixture, empty_dir, remove_dir, write_file};

#[test]
fn files_only_container_is_owned_by_rule_05() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(
        rule_05.len(),
        1,
        "expected one files-only container error: {rule_05:#?}"
    );
    assert_eq!(
        rule_05[0].file.as_deref(),
        Some("apps/devctl/crates/domain")
    );
    assert!(rule_05[0].message.contains("README.md"));
}

#[test]
fn missing_container_dir_is_not_owned_by_rule_05() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(rule_05.len(), 1, "{rule_05:#?}");
    assert_eq!(
        rule_05[0].file.as_deref(),
        Some("apps/devctl/crates/domain")
    );
}
