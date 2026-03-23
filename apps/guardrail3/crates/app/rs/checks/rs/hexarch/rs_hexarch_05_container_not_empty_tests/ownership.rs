use super::super::super::test_support::{
    copy_fixture, empty_dir, errors_by_id, run_family, write_file,
};

#[test]
fn files_only_container_is_owned_by_rule_05() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
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
