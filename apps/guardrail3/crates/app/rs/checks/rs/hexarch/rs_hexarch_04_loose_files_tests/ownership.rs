use super::super::super::test_support::{
    copy_fixture, empty_dir, errors_by_id, run_family, write_file,
};

#[test]
fn files_only_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_04.is_empty(),
        "rule 04 should not double-fire on files-only container: {rule_04:#?}"
    );
    assert_eq!(
        rule_05.len(),
        1,
        "expected rule 05 to own files-only container: {rule_05:#?}"
    );
    assert!(rule_05[0].message.contains("README.md"));
}
