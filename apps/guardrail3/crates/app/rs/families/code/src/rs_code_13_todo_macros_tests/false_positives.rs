use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_unreachable_in_test_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/commands/tests/todo_macro_tests.rs";
    write_file(
        root,
        rel,
        "fn probe() {\n    let _ = \"todo! in string\";\n    // unimplemented! in comment\n    unreachable!();\n    maybe_todo();\n}\n",
    );

    let results = run_family(root);
    let rs_code_13_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-13")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-13"), BTreeSet::new());
    assert!(rs_code_13_results.is_empty());
}
