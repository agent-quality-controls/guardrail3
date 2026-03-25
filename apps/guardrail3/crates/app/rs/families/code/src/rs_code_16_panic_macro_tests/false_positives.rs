use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_panic_macro_only_in_test_paths_and_not_for_other_macro_text() {
    let fixture = copy_fixture();
    let root = fixture.path();

    write_file(
        root,
        "apps/backend/crates/app/queries/tests/panic_macro_tests.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/test/panic_macro_test.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/__tests__/panic_macro.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/panic_macro_test.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/panic_macro_tests.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/queries/src/tests.rs",
        "fn probe() { panic!(\"boom\"); }\n",
    );
    write_file(
        root,
        "apps/devctl/crates/app/core/src/lib.rs",
        "fn probe() {\n    let _ = \"panic! in string\";\n    // panic! in comment\n    todo!();\n    unimplemented!();\n    unreachable!();\n}\n",
    );

    let results = run_family(root);
    let rs_code_16_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-16")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-16"), BTreeSet::new());
    assert!(rs_code_16_results.is_empty());
}
