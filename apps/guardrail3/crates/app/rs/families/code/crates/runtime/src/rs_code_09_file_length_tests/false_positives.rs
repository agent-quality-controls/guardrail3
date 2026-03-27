use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_test_file_even_when_it_exceeds_threshold() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let filler = "fn filler() {}\n".repeat(600);
    for rel in [
        "apps/backend/crates/app/commands/tests/long_case_tests.rs",
        "apps/backend/crates/app/commands/test/long_case.rs",
        "apps/backend/crates/app/commands/src/__tests__/long_case.rs",
        "apps/backend/crates/app/commands/src/long_case_test.rs",
        "apps/backend/crates/app/commands/src/long_case_tests.rs",
        "apps/backend/crates/app/commands/src/tests.rs",
    ] {
        write_file(root, rel, &filler);
    }

    let results = run_family(root);
    let rs_code_09_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-09")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-09"), BTreeSet::new());
    assert!(rs_code_09_results.is_empty());
}

#[test]
fn skips_file_with_many_comment_lines_but_only_500_effective_lines() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let effective = "fn filler() {}\n".repeat(500);
    let comments = "// comment padding\n".repeat(200);
    write_file(root, rel, &format!("{effective}{comments}"));

    let results = run_family(root);
    let rs_code_09_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-09")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-09"), BTreeSet::new());
    assert!(rs_code_09_results.is_empty());
}
