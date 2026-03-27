use guardrail3_app_rs_family_code_assertions::rs_code_16_panic_macro::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

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
    assert_no_hits(&results);
}
